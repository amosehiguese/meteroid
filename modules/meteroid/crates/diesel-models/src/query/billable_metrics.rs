use crate::billable_metrics::{BillableMetric, BillableMetricMeta, BillableMetricNew};
use crate::errors::IntoDbResult;

use crate::{DbResult, PgConn};

use crate::extend::pagination::{Paginate, PaginatedVec, PaginationRequest};
use diesel::{debug_query, JoinOnDsl, SelectableHelper};
use diesel::{ExpressionMethods, QueryDsl};
use error_stack::ResultExt;

impl BillableMetricNew {
    pub async fn insert(&self, conn: &mut PgConn) -> DbResult<BillableMetric> {
        use crate::schema::billable_metric::dsl::*;
        use diesel_async::RunQueryDsl;

        let query = diesel::insert_into(billable_metric).values(self);

        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .get_result(conn)
            .await
            .attach_printable("Error while inserting billable metric")
            .into_db_result()
    }
}

impl BillableMetric {
    pub async fn find_by_id(
        conn: &mut PgConn,
        param_billable_metric_id: uuid::Uuid,
        param_tenant_id: uuid::Uuid,
    ) -> DbResult<BillableMetric> {
        use crate::schema::billable_metric::dsl::*;
        use diesel_async::RunQueryDsl;

        let query = billable_metric
            .filter(id.eq(param_billable_metric_id))
            .filter(tenant_id.eq(param_tenant_id));
        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .first(conn)
            .await
            .attach_printable("Error while finding billable metric by id")
            .into_db_result()
    }

    pub async fn get_by_ids(
        conn: &mut PgConn,
        metric_ids: &[uuid::Uuid],
        tenant_id_param: &uuid::Uuid,
    ) -> DbResult<Vec<BillableMetric>> {
        use crate::schema::billable_metric::dsl::*;
        use diesel_async::RunQueryDsl;

        billable_metric
            .filter(id.eq_any(metric_ids))
            .filter(tenant_id.eq(tenant_id_param))
            .get_results(conn)
            .await
            .attach_printable("Error while fetching billable metrics")
            .into_db_result()
    }

    pub async fn list(
        conn: &mut PgConn,
        param_tenant_id: uuid::Uuid,
        pagination: PaginationRequest,
        param_product_family_external_id: String,
    ) -> DbResult<PaginatedVec<BillableMetricMeta>> {
        use crate::schema::billable_metric::dsl as bm_dsl;
        use crate::schema::product_family::dsl as pf_dsl;

        let query = bm_dsl::billable_metric
            .inner_join(pf_dsl::product_family.on(bm_dsl::product_family_id.eq(pf_dsl::id)))
            .filter(bm_dsl::tenant_id.eq(param_tenant_id))
            .filter(pf_dsl::external_id.eq(param_product_family_external_id))
            .order(bm_dsl::created_at.asc())
            .select(BillableMetricMeta::as_select());

        let paginated_query = query.paginate(pagination);

        log::debug!(
            "{}",
            debug_query::<diesel::pg::Pg, _>(&paginated_query).to_string()
        );

        paginated_query
            .load_and_count_pages(conn)
            .await
            .attach_printable("Error while fetching billable metrics")
            .into_db_result()
    }
}
