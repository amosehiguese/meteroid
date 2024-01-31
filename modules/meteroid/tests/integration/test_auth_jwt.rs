use testcontainers::clients::Cli;
use tonic::Code;

use crate::helpers;
use crate::meteroid_it;
use crate::meteroid_it::container::SeedLevel;
use meteroid_grpc::meteroid::api::users::v1::users_service_client::UsersServiceClient;

#[tokio::test]
async fn test_jwt() {
    // Generic setup
    helpers::init::logging();
    let docker = Cli::default();
    let (_postgres_container, postgres_connection_string) =
        meteroid_it::container::start_postgres(&docker);
    let setup =
        meteroid_it::container::start_meteroid(postgres_connection_string, SeedLevel::MINIMAL)
            .await;

    let svc = tower::ServiceBuilder::new().service(setup.channel.clone());
    let users_svc = UsersServiceClient::new(svc);

    // # authenticating with fake creds

    let auth_response = users_svc
        .clone()
        .login(tonic::Request::new(
            meteroid_grpc::meteroid::api::users::v1::LoginRequest {
                email: "fake".to_string(),
                password: "fake".to_string(),
            },
        ))
        .await;

    assert_eq!(auth_response.is_err(), true);
    assert_eq!(
        auth_response.map_err(|e| e.code()).unwrap_err(),
        Code::Unauthenticated
    );

    // # authenticating with wrong password

    let auth_response = users_svc
        .clone()
        .login(tonic::Request::new(
            meteroid_grpc::meteroid::api::users::v1::LoginRequest {
                email: meteroid_it::svc_auth::SEED_USERNAME.to_string(),
                password: "fake".to_string(),
            },
        ))
        .await;

    assert_eq!(auth_response.is_err(), true);
    assert_eq!(
        auth_response.map_err(|e| e.code()).unwrap_err(),
        Code::Unauthenticated
    );

    // # authenticating with correct creds
    let auth_response = users_svc
        .clone()
        .login(tonic::Request::new(
            meteroid_grpc::meteroid::api::users::v1::LoginRequest {
                email: meteroid_it::svc_auth::SEED_USERNAME.to_string(),
                password: meteroid_it::svc_auth::SEED_PASSWORD.to_string(),
            },
        ))
        .await;

    assert_eq!(auth_response.is_ok(), true);

    let auth = auth_response.unwrap().into_inner();
    assert_eq!(auth.token.clone().is_empty(), false);

    // # try to access secured method with fake auth token
    let clients =
        meteroid_it::clients::AllClients::from_channel(setup.channel.clone(), "faketoken", "");

    let tenants_response = clients
        .tenants
        .clone()
        .list_tenants(tonic::Request::new(
            meteroid_grpc::meteroid::api::tenants::v1::ListTenantsRequest {},
        ))
        .await;
    assert_eq!(tenants_response.is_err(), true);
    assert_eq!(
        tenants_response.map_err(|e| e.code()).unwrap_err(),
        Code::Unauthenticated
    );

    // # access secured method with correct auth token
    let clients = meteroid_it::clients::AllClients::from_channel(
        setup.channel.clone(),
        auth.token.clone().as_str(),
        "",
    );

    let tenants_response = clients
        .tenants
        .clone()
        .list_tenants(tonic::Request::new(
            meteroid_grpc::meteroid::api::tenants::v1::ListTenantsRequest {},
        ))
        .await;
    assert_eq!(tenants_response.is_ok(), true);

    // teardown
    meteroid_it::container::terminate_meteroid(setup.token, setup.join_handle).await;
}
