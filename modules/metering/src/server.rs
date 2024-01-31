use crate::auth::ExternalApiAuthLayer;
use crate::config::Config;
use crate::connectors::clickhouse::ClickhouseConnector;
use crate::ingest;
use crate::ingest::sinks::kafka::KafkaSink;

use common_grpc::middleware::server as common_middleware;

use common_grpc::middleware::client::{build_layered_client_service, LayeredClientService};
use meteroid_grpc::meteroid::internal::v1::internal_service_client::InternalServiceClient;
use std::sync::Arc;
use tonic::transport::{Channel, Endpoint, Server};
use tonic_tracing_opentelemetry::middleware as otel_middleware;

fn only_internal(path: &str) -> bool {
    path.starts_with("/meteroid.metering.v1.UsageQueryService")
        || path.starts_with("/meteroid.metering.v1.MetersService")
}

fn only_api(path: &str) -> bool {
    path.starts_with("/meteroid.metering.v1.EventsService")
}

pub async fn start_api_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    log::info!(
        "Starting Metering API grpc server on port {}",
        config.listen_addr.port()
    );

    let clickhouse_connector =
        Arc::new(ClickhouseConnector::init(&config.clickhouse, &config.kafka).await?);
    let kafka_sink = Arc::new(KafkaSink::new(&config.kafka)?);

    // make it lazy ? we want to be able to start the server without the billing api ?

    let channel = Endpoint::from_shared(config.meteroid_endpoint.clone())
        .expect("Failed to create channel to meteroid from shared endpoint");
    let channel = channel
        .connect()
        .await
        .or_else(|e| {
            log::warn!("Failed to connect to the meteroid GRPC channel for endpoint {}: {}. Starting in lazy mode.", config.meteroid_endpoint.clone(), e);
            Ok::<Channel, tonic::transport::Error>(channel.connect_lazy())
        })?;

    let service = build_layered_client_service(channel, &config.internal_auth);

    let internal_client: InternalServiceClient<LayeredClientService> =
        InternalServiceClient::new(service.clone());

    let (_, health_service) = tonic_health::server::health_reporter();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(meteroid_grpc::_reflection::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let admin_auth_layer =
        common_middleware::auth::create_admin(&config.internal_auth).filter(only_internal);

    let api_key_auth_layer = ExternalApiAuthLayer::new(internal_client.clone()).filter(only_api);

    // Ingest => Api key only (though we may want a way to ingest from the  for debugging, later)
    let event_service = ingest::service(internal_client.clone(), kafka_sink.clone());

    // Meters & queries => Admin only. Some passthrough is possible via admin
    let meter_service = crate::meters::service(clickhouse_connector.clone());
    let query_service = crate::query::service(clickhouse_connector.clone());

    Server::builder()
        .layer(common_middleware::metric::create())
        .layer(api_key_auth_layer.clone())
        .layer(admin_auth_layer.clone())
        .layer(
            otel_middleware::server::OtelGrpcLayer::default()
                .filter(otel_middleware::filters::reject_healthcheck),
        )
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(meter_service)
        .add_service(query_service)
        .add_service(event_service)
        .serve(config.listen_addr)
        .await?;

    Ok(())
}
