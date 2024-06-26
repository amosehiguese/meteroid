use crate::errors::IntoDbResult;
use crate::schedules::{Schedule, ScheduleNew, SchedulePatch};
use crate::schema::schedule;
use crate::{DbResult, PgConn};

use error_stack::ResultExt;

use diesel::{debug_query, ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};

impl ScheduleNew {
    pub async fn insert(&self, conn: &mut PgConn) -> DbResult<Schedule> {
        use crate::schema::schedule::dsl::*;
        use diesel_async::RunQueryDsl;

        let query = diesel::insert_into(schedule).values(self);

        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .get_result(conn)
            .await
            .attach_printable("Error while inserting schedule")
            .into_db_result()
    }
}

impl Schedule {
    pub async fn delete(
        conn: &mut PgConn,
        id: uuid::Uuid,
        tenant_id: uuid::Uuid,
    ) -> DbResult<usize> {
        use crate::schema::plan_version::dsl as pv_dsl;
        use crate::schema::schedule::dsl as s_dsl;
        use diesel_async::RunQueryDsl;

        let query = diesel::delete(s_dsl::schedule)
            .filter(s_dsl::id.eq(id))
            .filter(
                s_dsl::plan_version_id.eq_any(
                    pv_dsl::plan_version
                        .select(pv_dsl::id)
                        .filter(pv_dsl::tenant_id.eq(tenant_id))
                        .filter(pv_dsl::is_draft_version.eq(true)),
                ),
            );

        log::info!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .execute(conn)
            .await
            .attach_printable("Error while deleting schedule")
            .into_db_result()
    }

    pub async fn insert_schedule_batch(
        conn: &mut PgConn,
        batch: Vec<ScheduleNew>,
    ) -> DbResult<Vec<Schedule>> {
        use crate::schema::schedule::dsl::*;
        use diesel_async::RunQueryDsl;

        let query = diesel::insert_into(schedule).values(&batch);

        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .get_results(conn)
            .await
            .attach_printable("Error while inserting schedule batch")
            .into_db_result()
    }

    pub async fn list_schedules_by_subscription(
        conn: &mut PgConn,
        tenant_id_params: &uuid::Uuid,
        subscription_id: &uuid::Uuid,
    ) -> DbResult<Vec<Schedule>> {
        use crate::schema::schedule::dsl as schedule_dsl;
        use crate::schema::subscription;

        use diesel_async::RunQueryDsl;

        let query = schedule_dsl::schedule
            .inner_join(
                subscription::table.on(schedule::plan_version_id.eq(subscription::plan_version_id)),
            )
            .filter(subscription::id.eq(subscription_id))
            .filter(subscription::tenant_id.eq(tenant_id_params))
            .select(Schedule::as_select());

        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .get_results(conn)
            .await
            .attach_printable("Error while fetching schedules by subscription")
            .into_db_result()
    }

    pub async fn list(
        conn: &mut PgConn,
        plan_version_id: uuid::Uuid,
        tenant_id: uuid::Uuid,
    ) -> DbResult<Vec<Schedule>> {
        use crate::schema::plan_version::dsl as pv_dsl;
        use crate::schema::schedule::dsl as s_dsl;

        use diesel_async::RunQueryDsl;

        let query = s_dsl::schedule
            .inner_join(pv_dsl::plan_version.on(s_dsl::plan_version_id.eq(pv_dsl::id)))
            .filter(pv_dsl::id.eq(plan_version_id))
            .filter(pv_dsl::tenant_id.eq(tenant_id))
            .select(Schedule::as_select());

        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .get_results(conn)
            .await
            .attach_printable("Error while fetching schedules")
            .into_db_result()
    }
}

impl SchedulePatch {
    pub async fn update(&self, conn: &mut PgConn, tenant_id: uuid::Uuid) -> DbResult<Schedule> {
        use crate::schema::plan_version::dsl as pv_dsl;
        use crate::schema::schedule::dsl as s_dsl;
        use diesel_async::RunQueryDsl;

        let query = diesel::update(s_dsl::schedule)
            .filter(s_dsl::id.eq(self.id))
            .filter(
                s_dsl::plan_version_id.eq_any(
                    pv_dsl::plan_version
                        .select(pv_dsl::id)
                        .filter(pv_dsl::tenant_id.eq(tenant_id))
                        .filter(pv_dsl::is_draft_version.eq(true)),
                ),
            )
            .set(self);

        log::debug!("{}", debug_query::<diesel::pg::Pg, _>(&query).to_string());

        query
            .get_result(conn)
            .await
            .map_err(|e| {
                log::error!("Error while updating schedule: {:?}", e);
                e
            })
            .attach_printable("Error while updating schedule")
            .into_db_result()
    }
}
