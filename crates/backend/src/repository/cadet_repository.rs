use crate::error::CadetHubBeError;
use crate::repository::database::Database;
use crate::repository::db_error_handler;
use crate::repository::entity::{
    CadetCourseEntity, CadetCourseEntryEntity, CadetCourseStatisticEntryEntity, CadetEntity,
};
use crate::repository::query::{add_in_clause, add_pagination, InsertMode};
use crate::CadetHubBeResult;
use async_trait::async_trait;
use common::logger::{error, info};
use common::model::{
    Cadet, CadetCourse, CadetCourseEntry, CadetCourseStatisticEntry, ImpexCadetCourseEntry,
};
use common::model::{SearchCadetCourseRequest, SearchCadetRequest};
#[cfg(test)]
use mockall::automock;
use sqlx::{Executor, QueryBuilder, Row, Sqlite};
use std::sync::Arc;

#[cfg_attr(test, automock)]
#[async_trait]
pub(crate) trait CadetRepository {
    async fn save_cadet(&self, cadet: Cadet) -> CadetHubBeResult<Cadet>;
    #[allow(dead_code)]
    async fn save_cadet_if_not_exist(&self, cadet: Cadet) -> CadetHubBeResult<Cadet>;
    async fn update_cadet(&self, cadet: Cadet) -> CadetHubBeResult<Cadet>;
    async fn delete_cadet(&self, id: i64) -> CadetHubBeResult<()>;
    async fn find_cadet_by_id(&self, id: i64) -> CadetHubBeResult<Option<Cadet>>;
    async fn count_cadet_by_search_request(
        &self,
        request: SearchCadetRequest,
    ) -> CadetHubBeResult<i64>;
    async fn find_cadet_by_search_request(
        &self,
        request: SearchCadetRequest,
    ) -> CadetHubBeResult<Vec<Cadet>>;

    async fn save_cadet_course(&self, cadet_course: CadetCourse) -> CadetHubBeResult<CadetCourse>;
    #[allow(dead_code)]
    async fn save_cadet_course_if_not_exist(
        &self,
        cadet_course: CadetCourse,
    ) -> CadetHubBeResult<CadetCourse>;
    async fn update_cadet_course(&self, cadet_course: CadetCourse)
        -> CadetHubBeResult<CadetCourse>;
    async fn delete_cadet_course(&self, id: i64) -> CadetHubBeResult<()>;
    async fn find_cadet_course_by_id(&self, id: i64) -> CadetHubBeResult<Option<CadetCourse>>;
    async fn count_cadet_course_entries_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<i64>;
    async fn find_cadet_course_entries_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<Vec<CadetCourseEntry>>;

    async fn find_cadet_course_statistic_entries_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<Vec<CadetCourseStatisticEntry>>;

    async fn save_cadet_impex_entries(
        &self,
        impex_entries: Vec<ImpexCadetCourseEntry>,
    ) -> CadetHubBeResult<Vec<(ImpexCadetCourseEntry, CadetHubBeError)>>;
}

#[derive(Clone)]
pub(crate) struct DefaultCadetRepository {
    database: Arc<Database>,
}

impl DefaultCadetRepository {
    pub(crate) fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl CadetRepository for DefaultCadetRepository {
    async fn save_cadet(&self, cadet: Cadet) -> CadetHubBeResult<Cadet> {
        info!(
            "Save cadet: last_name={:?}, birth_date={:?}",
            cadet.last_name(),
            cadet.birth_date()
        );
        let cadet = internal_save_cadet(
            self.database.share_pool(),
            cadet,
            InsertMode::OnConflictFail,
        )
        .await?;
        info!(
            "Cadet saved: id={:?}, last_name={:?}, birth_date={:?}",
            cadet.require_id(),
            cadet.last_name(),
            cadet.birth_date()
        );
        Ok(cadet)
    }

    async fn save_cadet_if_not_exist(&self, cadet: Cadet) -> CadetHubBeResult<Cadet> {
        internal_save_cadet_if_not_exist(self.database.share_pool(), cadet).await
    }

    async fn update_cadet(&self, cadet: Cadet) -> CadetHubBeResult<Cadet> {
        let cadet_entity = CadetEntity::from(&cadet);
        info!(
            "Update cadet: id={:?}, last_name={:?}, birth_date={:?}",
            cadet.id(),
            cadet.last_name(),
            cadet.birth_date()
        );
        let update_statement = r#"
            UPDATE cadets SET
                tax_number = ?,
                first_name = ?,
                middle_name = ?,
                last_name = ?,
                birth_date = ?
            WHERE id = ?
            RETURNING *;
        "#;
        let cadet = sqlx::query_as::<_, CadetEntity>(update_statement)
            .bind(cadet_entity.tax_number().clone())
            .bind(cadet_entity.first_name().clone())
            .bind(cadet_entity.middle_name().clone())
            .bind(cadet_entity.last_name().clone())
            .bind(cadet_entity.birth_date().clone())
            .bind(cadet_entity.id().clone())
            .fetch_one(self.database.share_pool())
            .await
            .map(Cadet::from)
            .map_err(db_error_handler::handle)?;
        info!(
            "Cadet update: id={:?}, last_name={:?}, birth_date={:?}",
            cadet.id(),
            cadet.last_name(),
            cadet.birth_date()
        );
        Ok(cadet)
    }

    async fn delete_cadet(&self, id: i64) -> CadetHubBeResult<()> {
        info!("Delete cadet: id={:?}", id);
        let delete_statement = r#"
            DELETE FROM cadets WHERE id = ?
        "#;
        sqlx::query(delete_statement)
            .bind(id)
            .execute(self.database.share_pool())
            .await
            .map_err(db_error_handler::handle)?;
        info!("Cadet deleted: id={:?}", id);
        Ok(())
    }

    async fn find_cadet_by_id(&self, id: i64) -> CadetHubBeResult<Option<Cadet>> {
        info!("Find cadet: id={:?}", id);
        let select_statement = r#"SELECT * FROM cadets WHERE id = ?"#;
        let cadet_opt = sqlx::query_as::<_, CadetEntity>(select_statement)
            .bind(id)
            .fetch_optional(self.database.share_pool())
            .await
            .map_err(db_error_handler::handle)?
            .map(Cadet::from);
        info!("Cadet: id={:?}, found={:?}", id, cadet_opt.is_some());
        Ok(cadet_opt)
    }

    async fn count_cadet_by_search_request(
        &self,
        request: SearchCadetRequest,
    ) -> CadetHubBeResult<i64> {
        info!(
            "Count cadets: number_of_tax_numbers={:?}, number_of_last_names={:?}, birth_date_after={:?}, birth_date_before={:?}",
            request.tax_numbers().as_ref().map_or(0, |vector| vector.len()),
            request.last_names().as_ref().map_or(0, |vector| vector.len()),
            request.birth_date_after(),
            request.birth_date_before(),
        );
        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("SELECT count(*) FROM cadets c WHERE 1=1");
        apply_cadet_search_filters(
            &mut query_builder,
            request.tax_numbers(),
            request.last_names(),
            request.birth_date_after(),
            request.birth_date_before(),
        );
        let number_of_cadets = query_builder
            .build()
            .fetch_one(self.database.share_pool())
            .await
            .map_err(db_error_handler::handle)?
            .get(0);
        info!("Cadets: number_of_entries={:?}", number_of_cadets);
        Ok(number_of_cadets)
    }

    async fn find_cadet_by_search_request(
        &self,
        request: SearchCadetRequest,
    ) -> CadetHubBeResult<Vec<Cadet>> {
        info!(
            "Find cadets: number_of_tax_numbers={:?}, number_of_last_names={:?}, birth_date_after={:?}, birth_date_before={:?}, page_index={}, page_size={}",
            request.tax_numbers().as_ref().map_or(0, |vector| vector.len()),
            request.last_names().as_ref().map_or(0, |vector| vector.len()),
            request.birth_date_after(),
            request.birth_date_before(),
            request.page_request().page_index(),
            request.page_request().page_size(),
        );
        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("SELECT * FROM cadets c WHERE 1=1");
        apply_cadet_search_filters(
            &mut query_builder,
            request.tax_numbers(),
            request.last_names(),
            request.birth_date_after(),
            request.birth_date_before(),
        );
        add_pagination(
            &mut query_builder,
            "id DESC",
            request.page_request().limit(),
            request.page_request().offset(),
        );
        let cadets: Vec<Cadet> = query_builder
            .build_query_as::<CadetEntity>()
            .fetch_all(self.database.share_pool())
            .await
            .map(|entities| entities.into_iter().map(Cadet::from).collect())
            .map_err(db_error_handler::handle)?;
        info!("Found cadets: number_of_cadets={:?}", cadets.len());
        Ok(cadets)
    }

    async fn save_cadet_course(&self, cadet_course: CadetCourse) -> CadetHubBeResult<CadetCourse> {
        info!(
            "Save cadet course: cadet_id={:?}, specialty_code={:?}, end_date={:?}",
            cadet_course.require_cadet_id(),
            cadet_course.specialty_code(),
            cadet_course.end_date(),
        );
        let cadet_course = internal_save_cadet_course(
            self.database.share_pool(),
            cadet_course,
            InsertMode::OnConflictFail,
        )
        .await?;
        info!(
            "Cadet course saved: id={:?}, cadet_id={:?}, specialty_code={:?}, end_date={:?}",
            cadet_course.require_id(),
            cadet_course.require_cadet_id(),
            cadet_course.specialty_code(),
            cadet_course.end_date(),
        );
        Ok(cadet_course)
    }

    async fn save_cadet_course_if_not_exist(
        &self,
        cadet_course: CadetCourse,
    ) -> CadetHubBeResult<CadetCourse> {
        internal_save_cadet_course_if_not_exist(self.database.share_pool(), cadet_course).await
    }

    async fn update_cadet_course(
        &self,
        cadet_course: CadetCourse,
    ) -> CadetHubBeResult<CadetCourse> {
        let cadet_course_entity = CadetCourseEntity::from(&cadet_course);
        info!(
            "Update cadet course: id={:?}, cadet_id={:?}, specialty_code={:?}, end_date={:?}",
            cadet_course_entity.id(),
            cadet_course_entity.cadet_id(),
            cadet_course_entity.specialty_code(),
            cadet_course_entity.end_date(),
        );
        let update_statement = r#"
            UPDATE cadet_courses SET
                military_rank = ?,
                source_unit = ?,
                specialty_name = ?,
                specialty_code = ?,
                specialty_mos_code = ?,
                category = ?,
                training_location = ?,
                start_date = ?,
                end_date = ?,
                completion_order_number = ?,
                completion_certificate_number = ?,
                notes = ?
            WHERE id = ?
            RETURNING *;
        "#;
        let cadet_course = sqlx::query_as::<_, CadetCourseEntity>(update_statement)
            .bind(cadet_course_entity.military_rank().as_str())
            .bind(cadet_course_entity.source_unit().as_str())
            .bind(cadet_course_entity.specialty_name().as_str())
            .bind(cadet_course_entity.specialty_code().as_str())
            .bind(cadet_course_entity.specialty_mos_code().as_str())
            .bind(cadet_course_entity.category().as_str())
            .bind(cadet_course_entity.training_location().as_str())
            .bind(cadet_course_entity.start_date().clone())
            .bind(cadet_course_entity.end_date().clone())
            .bind(cadet_course_entity.completion_order_number().clone())
            .bind(cadet_course_entity.completion_certificate_number().clone())
            .bind(cadet_course_entity.notes().clone())
            .bind(cadet_course_entity.id().clone())
            .fetch_one(self.database.share_pool())
            .await
            .map(CadetCourse::from)
            .map_err(db_error_handler::handle)?;
        info!(
            "Cadet course updated: id={:?}, cadet_id={:?}, specialty_code={:?}, end_date={:?}",
            cadet_course_entity.id(),
            cadet_course_entity.cadet_id(),
            cadet_course_entity.specialty_code(),
            cadet_course_entity.end_date(),
        );
        Ok(cadet_course)
    }

    async fn delete_cadet_course(&self, id: i64) -> CadetHubBeResult<()> {
        info!("Delete cadet course: id={:?}", id);
        let delete_statement = r#"
            DELETE FROM cadet_courses WHERE id = ?
        "#;
        sqlx::query(delete_statement)
            .bind(id)
            .execute(self.database.share_pool())
            .await
            .map_err(db_error_handler::handle)?;
        info!("Cadet course deleted: id={:?}", id);
        Ok(())
    }

    async fn find_cadet_course_by_id(&self, id: i64) -> CadetHubBeResult<Option<CadetCourse>> {
        info!("Find cadet course: id={:?}", id);
        let select_statement = r#"SELECT * FROM cadet_courses WHERE id = ?"#;
        let cadet_course_opt = sqlx::query_as::<_, CadetCourseEntity>(select_statement)
            .bind(id)
            .fetch_optional(self.database.share_pool())
            .await
            .map_err(db_error_handler::handle)?
            .map(CadetCourse::from);
        info!(
            "Cadet course: id={:?}, found={:?}",
            id,
            cadet_course_opt.is_none()
        );
        Ok(cadet_course_opt)
    }

    async fn count_cadet_course_entries_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<i64> {
        info!(
            "Count cadet courses: number_of_tax_numbers={:?}, number_of_last_names={:?}, number_of_categories={:?}, birth_date_after={:?}, birth_date_before={:?}, start_date_after={:?}, end_date_after={:?}, page_index={}, page_size={}",
            request.tax_numbers().as_ref().map_or(0, |vector| vector.len()),
            request.last_names().as_ref().map_or(0, |vector| vector.len()),
            request.categories().as_ref().map_or(0, |vector| vector.len()),
            request.birth_date_after(),
            request.birth_date_before(),
            request.start_date_after(),
            request.end_date_before(),
            request.page_request().page_index(),
            request.page_request().page_size(),
        );
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            SELECT
                count(*)
            FROM cadets c INNER JOIN cadet_courses cc ON (c.id = cc.cadet_id)
            WHERE 1=1
        "#,
        );
        apply_cadet_course_search_filters(&request, &mut query_builder);
        let number_of_cadet_course_entries = query_builder
            .build()
            .fetch_one(self.database.share_pool())
            .await
            .map_err(db_error_handler::handle)?
            .get(0);
        info!(
            "Cadet course entries: number_of_entries={:?}",
            number_of_cadet_course_entries
        );
        Ok(number_of_cadet_course_entries)
    }

    async fn find_cadet_course_entries_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<Vec<CadetCourseEntry>> {
        info!(
            "Find cadet courses: number_of_tax_numbers={:?}, number_of_last_names={:?}, birth_date_after={:?}, birth_date_before={:?}, start_date_after={:?}, end_date_after={:?}, page_index={}, page_size={}",
            request.tax_numbers().as_ref().map_or(0, |vector| vector.len()),
            request.last_names().as_ref().map_or(0, |vector| vector.len()),
            request.birth_date_after(),
            request.birth_date_before(),
            request.start_date_after(),
            request.end_date_before(),
            request.page_request().page_index(),
            request.page_request().page_size(),
        );
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            SELECT
                c.tax_number,
                c.first_name,
                c.middle_name,
                c.last_name,
                c.birth_date,
                cc.*
            FROM cadets c INNER JOIN cadet_courses cc ON (c.id = cc.cadet_id)
            WHERE 1=1
        "#,
        );

        apply_cadet_course_search_filters(&request, &mut query_builder);

        add_pagination(
            &mut query_builder,
            "cc.id DESC",
            request.page_request().limit(),
            request.page_request().offset(),
        );

        let cadet_course_entries: Vec<CadetCourseEntry> = query_builder
            .build_query_as::<CadetCourseEntryEntity>()
            .fetch_all(self.database.share_pool())
            .await
            .map(|entities| entities.into_iter().map(CadetCourseEntry::from).collect())
            .map_err(db_error_handler::handle)?;
        info!(
            "Found cadet course entries: number_of_entries={:?}",
            cadet_course_entries.len()
        );
        Ok(cadet_course_entries)
    }

    async fn find_cadet_course_statistic_entries_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<Vec<CadetCourseStatisticEntry>> {
        info!(
            "Find cadet course statistic entries: number_of_tax_numbers={:?}, number_of_last_names={:?}, number_of_categories={:?}, birth_date_after={:?}, birth_date_before={:?}, start_date_after={:?}, end_date_after={:?}, page_index={}, page_size={}",
            request.tax_numbers().as_ref().map_or(0, |vector| vector.len()),
            request.last_names().as_ref().map_or(0, |vector| vector.len()),
            request.categories().as_ref().map_or(0, |vector| vector.len()),
            request.birth_date_after(),
            request.birth_date_before(),
            request.start_date_after(),
            request.end_date_before(),
            request.page_request().page_index(),
            request.page_request().page_size(),
        );
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            SELECT
                count(*) as number_of_cadet_courses,
                cc.training_location,
                cc.specialty_name,
                cc.specialty_code
            FROM cadets c INNER JOIN cadet_courses cc ON (c.id = cc.cadet_id)
            WHERE 1=1
        "#,
        );

        apply_cadet_course_search_filters(&request, &mut query_builder);

        query_builder.push(" GROUP BY cc.training_location, cc.specialty_name, cc.specialty_code ");

        let cadet_course_statistic_entries: Vec<CadetCourseStatisticEntry> = query_builder
            .build_query_as::<CadetCourseStatisticEntryEntity>()
            .fetch_all(self.database.share_pool())
            .await
            .map(|entities| {
                entities
                    .into_iter()
                    .map(CadetCourseStatisticEntry::from)
                    .collect()
            })
            .map_err(db_error_handler::handle)?;
        info!(
            "Found cadet course statistic entries: number_of_entries={:?}",
            cadet_course_statistic_entries.len()
        );
        Ok(cadet_course_statistic_entries)
    }

    async fn save_cadet_impex_entries(
        &self,
        impex_entries: Vec<ImpexCadetCourseEntry>,
    ) -> CadetHubBeResult<Vec<(ImpexCadetCourseEntry, CadetHubBeError)>> {
        let mut failed_entries = vec![];
        let mut tx = self
            .database
            .share_pool()
            .begin()
            .await
            .map_err(db_error_handler::handle)?;

        for import_entry in impex_entries.iter() {
            let mut cadet = Cadet::from(import_entry);
            cadet.normalize();
            let cadet_id = match internal_save_cadet_if_not_exist(&mut *tx, cadet).await {
                Ok(saved_cadet) => saved_cadet.require_id(),
                Err(error) => {
                    error!("{}", error.pretty_debug_str());
                    failed_entries.push((import_entry.clone(), error));
                    continue;
                }
            };

            let mut cadet_course = CadetCourse::from(import_entry);
            cadet_course.normalize();
            cadet_course.set_cadet_id(Some(cadet_id));
            if let Err(error) =
                internal_save_cadet_course_if_not_exist(&mut *tx, cadet_course).await
            {
                failed_entries.push((import_entry.clone(), error));
            }
        }

        tx.commit().await.map_err(db_error_handler::handle)?;
        Ok(failed_entries)
    }
}

async fn internal_save_cadet_if_not_exist<'a, E>(
    executor: E,
    cadet: Cadet,
) -> CadetHubBeResult<Cadet>
where
    E: Executor<'a, Database = Sqlite>,
{
    info!(
        "Save cadet if not exist: last_name={:?}, birth_date={:?}",
        cadet.last_name(),
        cadet.birth_date()
    );
    let cadet = internal_save_cadet(executor, cadet, InsertMode::OnConflictSkip).await?;
    info!(
        "Cadet saved if not exist: id={:?}, last_name={:?}, birth_date={:?}",
        cadet.id(),
        cadet.last_name(),
        cadet.birth_date()
    );
    Ok(cadet)
}

async fn internal_save_cadet<'a, E>(
    executor: E,
    cadet: Cadet,
    insert_mode: InsertMode,
) -> CadetHubBeResult<Cadet>
where
    E: Executor<'a, Database = Sqlite>,
{
    let mut cadet_entity = CadetEntity::from(&cadet);
    let mut upsert_statement = r#"
            INSERT INTO cadets (
                tax_number,
                first_name,
                middle_name,
                last_name,
                birth_date
            )
            VALUES (?, ?, ?, ?, ?)
        "#
    .to_string();
    match insert_mode {
        InsertMode::OnConflictFail => upsert_statement.push_str(
            r#"
            RETURNING id;
        "#,
        ),
        InsertMode::OnConflictSkip => upsert_statement.push_str(
            r#"
            ON CONFLICT(tax_number) DO
            UPDATE SET
                id = id
            RETURNING id;
        "#,
        ),
    }

    let cadet_id = sqlx::query(upsert_statement.as_str())
        .bind(cadet_entity.tax_number().clone())
        .bind(cadet_entity.first_name().clone())
        .bind(cadet_entity.middle_name().clone())
        .bind(cadet_entity.last_name().clone())
        .bind(cadet_entity.birth_date().clone())
        .fetch_one(executor)
        .await
        .map_err(db_error_handler::handle)?
        .get(0);

    cadet_entity.set_id(Some(cadet_id));
    Ok(Cadet::from(cadet_entity))
}

async fn internal_save_cadet_course_if_not_exist<'a, E>(
    executor: E,
    cadet_course: CadetCourse,
) -> CadetHubBeResult<CadetCourse>
where
    E: Executor<'a, Database = Sqlite>,
{
    info!(
        "Save cadet course if not exist: cadet_id={:?}, specialty_code={:?}, end_date={:?}",
        cadet_course.require_cadet_id(),
        cadet_course.specialty_code(),
        cadet_course.end_date(),
    );
    let cadet_course =
        internal_save_cadet_course(executor, cadet_course, InsertMode::OnConflictSkip).await?;
    info!(
            "Cadet course saved if not exist: id={:?}, cadet_id={:?}, specialty_code={:?}, end_date={:?}",
            cadet_course.require_id(),
            cadet_course.require_cadet_id(),
            cadet_course.specialty_code(),
            cadet_course.end_date(),
        );
    Ok(cadet_course)
}

async fn internal_save_cadet_course<'a, E>(
    executor: E,
    cadet_course: CadetCourse,
    insert_mode: InsertMode,
) -> CadetHubBeResult<CadetCourse>
where
    E: Executor<'a, Database = Sqlite>,
{
    let mut cadet_course_entity = CadetCourseEntity::from(&cadet_course);
    let mut upsert_statement = r#"
            INSERT INTO cadet_courses (
                cadet_id,
                military_rank,
                source_unit,
                specialty_name,
                specialty_code,
                specialty_mos_code,
                category,
                training_location,
                start_date,
                end_date,
                completion_order_number,
                completion_certificate_number,
                notes
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    .to_string();
    match insert_mode {
        InsertMode::OnConflictFail => upsert_statement.push_str(
            r#"
            RETURNING id;
        "#,
        ),
        InsertMode::OnConflictSkip => upsert_statement.push_str(
            r#"
            ON CONFLICT(cadet_id, specialty_code, end_date) DO
            UPDATE SET
                id = id
            RETURNING id;
        "#,
        ),
    }
    let cadet_course_id = sqlx::query(upsert_statement.as_str())
        .bind(cadet_course_entity.cadet_id().clone())
        .bind(cadet_course_entity.military_rank().clone())
        .bind(cadet_course_entity.source_unit().as_str())
        .bind(cadet_course_entity.specialty_name().as_str())
        .bind(cadet_course_entity.specialty_code().as_str())
        .bind(cadet_course_entity.specialty_mos_code().as_str())
        .bind(cadet_course_entity.category().as_str())
        .bind(cadet_course_entity.training_location().as_str())
        .bind(cadet_course_entity.start_date().clone())
        .bind(cadet_course_entity.end_date().clone())
        .bind(cadet_course_entity.completion_order_number().clone())
        .bind(cadet_course_entity.completion_certificate_number().clone())
        .bind(cadet_course_entity.notes().clone())
        .fetch_one(executor)
        .await
        .map_err(db_error_handler::handle)?
        .get(0);

    cadet_course_entity.set_id(Some(cadet_course_id));
    Ok(CadetCourse::from(cadet_course_entity))
}

fn apply_cadet_course_search_filters<'a>(
    request: &'a SearchCadetCourseRequest,
    query_builder: &mut QueryBuilder<'a, Sqlite>,
) {
    apply_cadet_search_filters(
        query_builder,
        request.tax_numbers(),
        request.last_names(),
        request.birth_date_after(),
        request.birth_date_before(),
    );
    add_in_clause(query_builder, "category", request.categories());
    if let Some(start_date_after) = request.start_date_after() {
        query_builder.push(" AND start_date >= ");
        query_builder.push_bind(start_date_after);
    }
    if let Some(start_date_after) = request.start_date_before() {
        query_builder.push(" AND start_date <= ");
        query_builder.push_bind(start_date_after);
    }
    if let Some(end_date_after) = request.end_date_after() {
        query_builder.push(" AND end_date >= ");
        query_builder.push_bind(end_date_after);
    }
    if let Some(end_date_before) = request.end_date_before() {
        query_builder.push(" AND end_date <= ");
        query_builder.push_bind(end_date_before);
    }
}

fn apply_cadet_search_filters<'a>(
    query_builder: &mut QueryBuilder<'a, Sqlite>,
    tax_numbers: &'a Option<Vec<String>>,
    last_names: &'a Option<Vec<String>>,
    birth_date_after: &'a Option<i64>,
    birth_date_before: &'a Option<i64>,
) {
    add_in_clause(query_builder, "c.tax_number", tax_numbers);
    add_in_clause(query_builder, "c.last_name", last_names);
    if let Some(after) = birth_date_after {
        query_builder.push(" AND c.birth_date >= ");
        query_builder.push_bind(*after);
    }
    if let Some(before) = birth_date_before {
        query_builder.push(" AND c.birth_date <= ");
        query_builder.push_bind(*before);
    }
}

#[cfg(test)]
mod tests {
    use crate::error::CadetHubBeError;
    use crate::repository::cadet_repository::{CadetRepository, DefaultCadetRepository};
    use crate::repository::database::Database;
    use common::config::{ApplicationConfigBuilder, DatabaseConfigBuilder};
    use common::model::{Cadet, CadetBuilder, CadetCourse, CadetCourseBuilder};
    use common::model::{
        PageRequest, PageRequestBuilder, SearchCadetCourseRequest, SearchCadetCourseRequestBuilder,
        SearchCadetRequest, SearchCadetRequestBuilder,
    };
    use spectral::prelude::*;
    use std::sync::Arc;

    async fn sut() -> DefaultCadetRepository {
        let db_config = DatabaseConfigBuilder::default()
            .url(Some("sqlite::memory:".to_string()))
            .encryption_enabled(false)
            .encryption_key(None)
            .build()
            .expect("failed build DbConfig");
        let config = ApplicationConfigBuilder::default()
            .database(db_config)
            .build()
            .expect("failed build ApplicationConfig");
        let database = Database::connect(&config).await.expect("failed to init DB");
        DefaultCadetRepository::new(Arc::new(database))
    }

    #[tokio::test]
    async fn should_save_cadet() {
        // Given
        let cadet = cadet(1);
        let sut = sut().await;

        // When
        let result = sut
            .save_cadet(cadet.clone())
            .await
            .expect("failed to save cadet");

        // Then
        assert_that(&result.id()).is_equal_to(&Some(1));
        assert_that(&result.tax_number()).is_equal_to(cadet.tax_number());
        assert_that(&result.first_name()).is_equal_to(cadet.first_name());
        assert_that(&result.middle_name()).is_equal_to(cadet.middle_name());
        assert_that(&result.last_name()).is_equal_to(cadet.last_name());
        assert_that(&result.birth_date()).is_equal_to(cadet.birth_date());
    }

    #[tokio::test]
    async fn should_save_cadet_case_conflict() {
        // Given
        let sut = sut().await;
        let cadet_one = sut
            .save_cadet(cadet(1))
            .await
            .expect("failed to save cadet");
        let mut cadet_two = cadet(2);
        cadet_two.set_tax_number(cadet_one.tax_number().clone());

        // When
        let result = sut.save_cadet(cadet_two).await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceConflictError { .. }));
    }

    #[tokio::test]
    async fn should_update_cadet() {
        // Given
        let sut = sut().await;
        let cadet = sut
            .save_cadet(cadet(1))
            .await
            .expect("failed to save cadet");
        let mut updated_cadet = cadet.clone();
        updated_cadet.set_tax_number("updated_tax_number".to_string());
        updated_cadet.set_first_name("updated_first_name".to_string());
        updated_cadet.set_middle_name("updated_middle_name".to_string());
        updated_cadet.set_last_name("updated_last_name(".to_string());
        updated_cadet.set_birth_date(cadet.birth_date().clone() + 1000);

        // When
        sut.update_cadet(updated_cadet.clone())
            .await
            .expect("failed to save cadet");
        let result = sut
            .find_cadet_by_id(cadet.require_id().clone())
            .await
            .expect("failed to find cadet")
            .expect("not existent cadet");

        // Then
        assert_that(&result).is_equal_to(&updated_cadet);
    }

    #[tokio::test]
    async fn should_update_cadet_case_conflict() {
        // Given
        let sut = sut().await;
        let cadet_one = sut
            .save_cadet(cadet(1))
            .await
            .expect("failed to save cadet");
        let mut cadet_two = sut
            .save_cadet(cadet(2))
            .await
            .expect("failed to save cadet");
        cadet_two.set_tax_number(cadet_one.tax_number().clone());

        // When
        let result = sut.update_cadet(cadet_two).await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceConflictError { .. }));
    }

    #[tokio::test]
    async fn should_save_cadet_if_not_exist() {
        // Given
        let cadet = cadet(1);
        let sut = sut().await;

        // When
        let result = sut
            .save_cadet_if_not_exist(cadet.clone())
            .await
            .expect("failed to save cadet if not exist");

        // Then
        assert_that(&result.id()).is_equal_to(&Some(1));
        assert_that(&result.tax_number()).is_equal_to(cadet.tax_number());
        assert_that(&result.first_name()).is_equal_to(cadet.first_name());
        assert_that(&result.middle_name()).is_equal_to(cadet.middle_name());
        assert_that(&result.last_name()).is_equal_to(cadet.last_name());
        assert_that(&result.birth_date()).is_equal_to(cadet.birth_date());
    }

    #[tokio::test]
    async fn should_save_cadet_if_not_exist_case_skip() {
        // Given
        let sut = sut().await;
        let cadet = sut
            .save_cadet(cadet(1))
            .await
            .expect("failed to save cadet");
        let mut updated_cadet = cadet.clone();
        updated_cadet.set_first_name("updated_first_name".to_string());
        updated_cadet.set_middle_name("updated_middle_name".to_string());
        updated_cadet.set_last_name("updated_last_name(".to_string());
        updated_cadet.set_birth_date(cadet.birth_date().clone() + 1000);

        // When
        sut.save_cadet_if_not_exist(updated_cadet.clone())
            .await
            .expect("failed to save cadet");
        let result = sut
            .find_cadet_by_id(cadet.require_id().clone())
            .await
            .expect("failed to find cadet")
            .expect("not existent cadet");

        // Then
        assert_that(&result).is_equal_to(&cadet);
    }

    #[tokio::test]
    async fn should_find_cadet_by_id() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");

        // When
        let result = sut
            .find_cadet_by_id(cadet.require_id())
            .await
            .expect("failed find cadet by id")
            .expect("not existent cadet");

        // Then
        assert_that(&result).is_equal_to(&cadet);
    }

    #[tokio::test]
    async fn should_find_cadet_by_id_case_non_existent_cadet() {
        // Given
        let non_existent_cadet_id = 2;
        let sut = sut().await;
        sut.save_cadet(cadet(1)).await.expect("failed save cadet");

        // When
        let result = sut
            .find_cadet_by_id(non_existent_cadet_id)
            .await
            .expect("failed find cadet by id");

        // Then
        assert_that(&result).is_none();
    }

    #[tokio::test]
    async fn should_count_cadet_by_search_request() {
        // Given
        let sut = sut().await;
        sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        sut.save_cadet(cadet(2)).await.expect("failed save cadet");
        let request = SearchCadetRequestBuilder::default()
            .page_request(PageRequest::all())
            .build()
            .expect("failed build CadetCourseSearchResponse");

        // When
        let result = sut
            .count_cadet_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).is_equal_to(2);
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request_case_pagination() {
        // Given
        let sut = sut().await;
        sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        sut.save_cadet(cadet(2)).await.expect("failed save cadet");
        let request = SearchCadetRequestBuilder::default()
            .page_request(page_request(1, 0))
            .build()
            .expect("failed build CadetCourseSearchResponse");

        // When
        let result = sut
            .find_cadet_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).has_length(1);
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let request = cadet_search_request(&cadet);

        // When
        let result = sut
            .find_cadet_by_search_request(request)
            .await
            .expect("failed find cadet by search request");

        // Then
        assert_that(&result).has_length(1);
        assert_that(&result).contains(&cadet);
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request_case_another_tax_number() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let mut request = cadet_search_request(&cadet);
        request.set_tax_numbers(Some(vec!["another_tax_number".to_string()]));

        // When
        let result = sut
            .find_cadet_by_search_request(request)
            .await
            .expect("failed find cadet by search request");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request_case_another_last_name() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let mut request = cadet_search_request(&cadet);
        request.set_last_names(Some(vec!["another_last_name".to_string()]));

        // When
        let result = sut
            .find_cadet_by_search_request(request)
            .await
            .expect("failed find cadet by search request");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request_case_older_birth_date() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let mut request = cadet_search_request(&cadet);
        request.set_birth_date_after(Some(cadet.birth_date().clone() + 1000));

        // When
        let result = sut
            .find_cadet_by_search_request(request)
            .await
            .expect("failed find cadet by search request");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request_case_newer_birth_date() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let mut request = cadet_search_request(&cadet);
        request.set_birth_date_before(Some(cadet.birth_date().clone() - 1000));

        // When
        let result = sut
            .find_cadet_by_search_request(request)
            .await
            .expect("failed find cadet by search request");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_delete_cadet() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");

        // When
        sut.delete_cadet(cadet.require_id().clone())
            .await
            .expect("failed delete cadet");
        let result = sut
            .find_cadet_by_id(cadet.require_id())
            .await
            .expect("failed find cadet by id");

        // Then
        assert_that(&result).is_none();
    }

    fn cadet_search_request(cadet: &Cadet) -> SearchCadetRequest {
        SearchCadetRequestBuilder::default()
            .tax_numbers(vec![cadet.tax_number().clone()])
            .last_names(vec![cadet.last_name().clone()])
            .birth_date_after(cadet.birth_date().clone() - 1000)
            .birth_date_before(cadet.birth_date().clone() + 1000)
            .page_request(PageRequest::all())
            .build()
            .expect("Failed build request")
    }

    fn cadet(index: usize) -> Cadet {
        CadetBuilder::default()
            .id(None)
            .tax_number(format!("tax_number_{index}"))
            .first_name(format!("first_name_{index}"))
            .middle_name(format!("middle_name_{index}"))
            .last_name(format!("first_name_{index}"))
            .birth_date(1000000 * index as i64)
            .build()
            .expect("failed build cadet")
    }

    #[tokio::test]
    async fn should_save_cadet_course() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = cadet_course(1, cadet.id().clone());

        // When
        let result = sut
            .save_cadet_course(cadet_course.clone())
            .await
            .expect("failed to save cadet course");

        // Then
        assert_that(&result.id()).is_equal_to(&Some(1));
        assert_that(&result.cadet_id()).is_equal_to(cadet_course.cadet_id());
        assert_that(&result.military_rank()).is_equal_to(cadet_course.military_rank());
        assert_that(&result.source_unit()).is_equal_to(cadet_course.source_unit());
        assert_that(&result.specialty_name()).is_equal_to(cadet_course.specialty_name());
        assert_that(&result.specialty_code()).is_equal_to(cadet_course.specialty_code());
        assert_that(&result.specialty_mos_code()).is_equal_to(cadet_course.specialty_mos_code());
        assert_that(&result.category()).is_equal_to(cadet_course.category());
        assert_that(&result.training_location()).is_equal_to(cadet_course.training_location());
        assert_that(&result.start_date()).is_equal_to(cadet_course.start_date());
        assert_that(&result.end_date()).is_equal_to(cadet_course.end_date());
        assert_that(&result.completion_order_number())
            .is_equal_to(cadet_course.completion_order_number());
        assert_that(&result.completion_certificate_number())
            .is_equal_to(cadet_course.completion_certificate_number());
        assert_that(&result.notes()).is_equal_to(cadet_course.notes());
    }

    #[tokio::test]
    async fn should_save_cadet_course_case_conflict() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course_one = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut cadet_course_two = cadet_course(2, cadet.id().clone());
        cadet_course_two.set_specialty_code(cadet_course_one.specialty_code().clone());
        cadet_course_two.set_end_date(cadet_course_one.end_date().clone());

        // When
        let result = sut.save_cadet_course(cadet_course_two).await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceConflictError { .. }));
    }

    #[tokio::test]
    async fn should_update_cadet_course() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut updated_cadet_course = cadet_course.clone();
        updated_cadet_course.set_military_rank("updated_military_rank".to_string());
        updated_cadet_course.set_source_unit("updated_source_unit".to_string());
        updated_cadet_course.set_specialty_name("updated_specialty_name".to_string());
        updated_cadet_course.set_specialty_code("updated_specialty_code".to_string());
        updated_cadet_course.set_specialty_mos_code("updated_specialty_mos_code".to_string());
        updated_cadet_course.set_category("CATEGORY".to_string());
        updated_cadet_course.set_training_location("updated_training_location".to_string());
        updated_cadet_course.set_start_date(cadet_course.start_date().clone() + 1000);
        updated_cadet_course.set_end_date(cadet_course.end_date().clone() + 1000);
        updated_cadet_course
            .set_completion_order_number("updated_completion_order_number".to_string());
        updated_cadet_course
            .set_completion_certificate_number("updated_completion_certificate_number".to_string());
        updated_cadet_course.set_notes(Some("updated_notes".to_string()));

        // When
        sut.update_cadet_course(updated_cadet_course.clone())
            .await
            .expect("failed to save cadet course");
        let result = sut
            .find_cadet_course_by_id(cadet_course.require_id().clone())
            .await
            .expect("failed find cadet courses")
            .expect("cadet course not found");

        // Then
        assert_that(&result).is_equal_to(&updated_cadet_course);
    }

    #[tokio::test]
    async fn should_update_cadet_course_case_conflict() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course_one = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut cadet_course_two = sut
            .save_cadet_course(cadet_course(2, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        cadet_course_two.set_specialty_code(cadet_course_one.specialty_code().clone());
        cadet_course_two.set_end_date(cadet_course_one.end_date().clone());

        // When
        let result = sut.update_cadet_course(cadet_course_two).await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceConflictError { .. }));
    }

    #[tokio::test]
    async fn should_save_cadet_course_if_not_exist() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = cadet_course(1, cadet.id().clone());

        // When
        let result = sut
            .save_cadet_course_if_not_exist(cadet_course.clone())
            .await
            .expect("failed to save cadet course if not exist");

        // Then
        assert_that(&result.id()).is_equal_to(&Some(1));
        assert_that(&result.cadet_id()).is_equal_to(cadet_course.cadet_id());
        assert_that(&result.military_rank()).is_equal_to(cadet_course.military_rank());
        assert_that(&result.source_unit()).is_equal_to(cadet_course.source_unit());
        assert_that(&result.specialty_name()).is_equal_to(cadet_course.specialty_name());
        assert_that(&result.specialty_code()).is_equal_to(cadet_course.specialty_code());
        assert_that(&result.specialty_mos_code()).is_equal_to(cadet_course.specialty_mos_code());
        assert_that(&result.category()).is_equal_to(cadet_course.category());
        assert_that(&result.training_location()).is_equal_to(cadet_course.training_location());
        assert_that(&result.start_date()).is_equal_to(cadet_course.start_date());
        assert_that(&result.end_date()).is_equal_to(cadet_course.end_date());
        assert_that(&result.completion_order_number())
            .is_equal_to(cadet_course.completion_order_number());
        assert_that(&result.completion_certificate_number())
            .is_equal_to(cadet_course.completion_certificate_number());
        assert_that(&result.notes()).is_equal_to(cadet_course.notes());
    }

    #[tokio::test]
    async fn should_save_cadet_course_if_not_exist_case_skip() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut updated_cadet_course = cadet_course.clone();
        updated_cadet_course.set_military_rank("updated_military_rank".to_string());
        updated_cadet_course.set_source_unit("updated_source_unit".to_string());
        updated_cadet_course.set_specialty_name("updated_specialty_name".to_string());
        updated_cadet_course.set_specialty_code("updated_specialty_code".to_string());
        updated_cadet_course.set_specialty_mos_code("updated_specialty_mos_code".to_string());
        updated_cadet_course.set_category("CATEGORY".to_string());
        updated_cadet_course.set_training_location("updated_training_location".to_string());
        updated_cadet_course.set_start_date(cadet_course.start_date().clone() + 1000);
        updated_cadet_course.set_end_date(cadet_course.end_date().clone() + 1000);
        updated_cadet_course
            .set_completion_order_number("updated_completion_order_number".to_string());
        updated_cadet_course
            .set_completion_certificate_number("updated_completion_certificate_number".to_string());
        updated_cadet_course.set_notes(Some("updated_notes".to_string()));

        // When
        sut.save_cadet_course_if_not_exist(updated_cadet_course.clone())
            .await
            .expect("failed to save cadet course");
        let result = sut
            .find_cadet_course_by_id(cadet_course.require_id().clone())
            .await
            .expect("failed find cadet course")
            .expect("cadet course not found");

        // Then
        assert_that(&result).is_equal_to(&cadet_course);
    }

    #[tokio::test]
    async fn should_find_cadet_course_by_id() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");

        // When
        let result = sut
            .find_cadet_course_by_id(cadet_course.require_id().clone())
            .await
            .expect("failed find cadet course")
            .expect("cadet course not found");

        // Then
        assert_that(&result).is_equal_to(&cadet_course);
    }

    #[tokio::test]
    async fn should_find_cadet_course_by_id_case_non_existent_cadet_course() {
        // Given
        let sut = sut().await;
        let non_existent_cadet_id = 2;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        sut.save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");

        // When
        let result = sut
            .find_cadet_course_by_id(non_existent_cadet_id)
            .await
            .expect("failed find cadet course");

        // Then
        assert_that(&result).is_none();
    }

    #[tokio::test]
    async fn should_count_cadet_course_entries_by_search_request() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        sut.save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        sut.save_cadet_course(cadet_course(2, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let request = SearchCadetCourseRequestBuilder::default()
            .page_request(PageRequest::all())
            .build()
            .expect("failed build CadetCourseSearchResponse");

        // When
        let result = sut
            .count_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).is_equal_to(2);
    }

    #[tokio::test]
    async fn should_find_cadet_course_entries_by_search_request() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let request = cadet_course_search_request(&cadet, &cadet_course);

        // When
        let result = sut
            .find_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).has_length(1);
        let result = result.get(0).expect("cadet course not found");
        assert_that(&result.id()).is_equal_to(&cadet_course.require_id());
        assert_that(&result.cadet_id()).is_equal_to(&cadet_course.require_cadet_id());
        assert_that(&result.tax_number()).is_equal_to(&cadet.tax_number());
        assert_that(&result.first_name()).is_equal_to(&cadet.first_name());
        assert_that(&result.middle_name()).is_equal_to(&cadet.middle_name());
        assert_that(&result.last_name()).is_equal_to(&cadet.last_name());
        assert_that(&result.birth_date()).is_equal_to(&cadet.birth_date());
        assert_that(&result.military_rank()).is_equal_to(&cadet_course.military_rank());
        assert_that(&result.source_unit()).is_equal_to(&cadet_course.source_unit());
        assert_that(&result.specialty_name()).is_equal_to(&cadet_course.specialty_name());
        assert_that(&result.specialty_code()).is_equal_to(&cadet_course.specialty_code());
        assert_that(&result.specialty_mos_code()).is_equal_to(&cadet_course.specialty_mos_code());
        assert_that(&result.category()).is_equal_to(&cadet_course.category());
        assert_that(&result.training_location()).is_equal_to(&cadet_course.training_location());
        assert_that(&result.start_date()).is_equal_to(&cadet_course.start_date());
        assert_that(&result.end_date()).is_equal_to(&cadet_course.end_date());
        assert_that(&result.completion_order_number())
            .is_equal_to(&cadet_course.completion_order_number());
        assert_that(&result.completion_certificate_number())
            .is_equal_to(&cadet_course.completion_certificate_number());
        assert_that(&result.notes()).is_equal_to(&cadet_course.notes());
    }

    #[tokio::test]
    async fn should_find_cadet_course_entries_by_search_request_case_pagination() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        sut.save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        sut.save_cadet_course(cadet_course(2, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let request = SearchCadetCourseRequestBuilder::default()
            .page_request(page_request(1, 0))
            .build()
            .expect("failed build CadetCourseSearchResponse");

        // When
        let result = sut
            .find_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).has_length(1);
    }

    #[tokio::test]
    async fn should_find_cadet_course_entries_by_search_request_case_another_category() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_categories(Some(vec!("ANOTHER_CATEGORY".to_string())));

        // When
        let result = sut
            .find_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_course_entries_by_search_request_case_newer_start_date_after() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_start_date_after(Some(cadet_course.start_date().clone() + 1000));

        // When
        let result = sut
            .find_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_course_entries_by_search_request_case_older_start_date_before() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_start_date_before(Some(cadet_course.start_date().clone() - 1000));

        // When
        let result = sut
            .find_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_course_entries_by_search_request_case_newer_end_date_after() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_end_date_after(Some(cadet_course.end_date().clone() + 1000));

        // When
        let result = sut
            .find_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_course_entries_by_search_request_case_older_end_date_before() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_end_date_before(Some(cadet_course.start_date().clone() - 1000));

        // When
        let result = sut
            .find_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_course_statistic_entries_by_search_request() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let request = cadet_course_search_request(&cadet, &cadet_course);

        // When
        let result = sut
            .find_cadet_course_statistic_entries_by_search_request(request)
            .await
            .expect("failed find cadet course statistic entry");

        // Then
        assert_that(&result).has_length(1);
        let result = result
            .get(0)
            .expect("cadet course statistic entry not found");
        assert_that(&result.specialty_name()).is_equal_to(&cadet_course.specialty_name());
        assert_that(&result.specialty_code()).is_equal_to(&cadet_course.specialty_code());
        assert_that(&result.training_location()).is_equal_to(&cadet_course.training_location());
        assert_that(&result.number_of_cadet_courses()).is_equal_to(&1);
    }

    #[tokio::test]
    async fn should_find_cadet_course_statistic_entries_by_search_request_case_newer_start_date_after(
    ) {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_start_date_after(Some(cadet_course.start_date().clone() + 1000));

        // When
        let result = sut
            .find_cadet_course_statistic_entries_by_search_request(request)
            .await
            .expect("failed find cadet course statistic entries");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_course_statistic_entries_by_search_request_case_older_start_date_before(
    ) {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_start_date_before(Some(cadet_course.start_date().clone() - 1000));

        // When
        let result = sut
            .find_cadet_course_statistic_entries_by_search_request(request)
            .await
            .expect("failed find cadet course statistic entries");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_course_statistic_entries_by_search_request_case_newer_end_date_after(
    ) {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_end_date_after(Some(cadet_course.end_date().clone() + 1000));

        // When
        let result = sut
            .find_cadet_course_statistic_entries_by_search_request(request)
            .await
            .expect("failed find cadet course statistic entries");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_find_cadet_course_statistic_entries_by_search_request_case_older_end_date_before(
    ) {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");
        let mut request = cadet_course_search_request(&cadet, &cadet_course);
        request.set_end_date_before(Some(cadet_course.start_date().clone() - 1000));

        // When
        let result = sut
            .find_cadet_course_entries_by_search_request(request)
            .await
            .expect("failed find cadet course statistic entries");

        // Then
        assert_that(&result).has_length(0);
    }

    #[tokio::test]
    async fn should_delete_cadet_course() {
        // Given
        let sut = sut().await;
        let cadet = sut.save_cadet(cadet(1)).await.expect("failed save cadet");
        let cadet_course = sut
            .save_cadet_course(cadet_course(1, cadet.id().clone()))
            .await
            .expect("failed save cadet course");

        // When
        sut.delete_cadet_course(cadet_course.require_id().clone())
            .await
            .expect("failed to save cadet course");
        let result = sut
            .find_cadet_course_by_id(cadet_course.require_id().clone())
            .await
            .expect("failed find cadet courses");

        // Then
        assert_that(&result).is_none();
    }

    fn cadet_course(index: usize, cadet_id: Option<i64>) -> CadetCourse {
        CadetCourseBuilder::default()
            .id(None)
            .cadet_id(cadet_id)
            .military_rank(format!("military_rank{index}"))
            .source_unit(format!("source_unit_{index}"))
            .specialty_name(format!("specialty_name_{index}"))
            .specialty_code(format!("specialty_code_{index}"))
            .specialty_mos_code(format!("specialty_mos_code_{index}"))
            .category(format!("CATEGORY_{index}"))
            .training_location(format!("training_location_{index}"))
            .start_date(1000000 * index as i64)
            .end_date(2000000 * index as i64)
            .completion_order_number(format!("completion_order_number_{index}"))
            .completion_certificate_number(format!("completion_certificate_number_{index}"))
            .notes(format!("notes_{index}"))
            .build()
            .expect("failed build CadetCourse")
    }

    fn cadet_course_search_request(
        cadet: &Cadet,
        cadet_course: &CadetCourse,
    ) -> SearchCadetCourseRequest {
        SearchCadetCourseRequestBuilder::default()
            .tax_numbers(vec![cadet.tax_number().clone()])
            .last_names(vec![cadet.last_name().clone()])
            .birth_date_after(cadet.birth_date().clone() - 1000)
            .birth_date_before(cadet.birth_date().clone() + 1000)
            .start_date_after(cadet_course.start_date().clone() - 1000)
            .start_date_before(cadet_course.start_date().clone() + 1000)
            .end_date_after(cadet_course.end_date().clone() - 1000)
            .end_date_before(cadet_course.end_date().clone() + 1000)
            .page_request(PageRequest::all())
            .build()
            .expect("failed build CadetCourseSearchRequest")
    }

    fn page_request(page_size: i64, page_index: i64) -> PageRequest {
        PageRequestBuilder::default()
            .page_size(page_size)
            .page_index(page_index)
            .build()
            .expect("failed build CadetCourseSearchRequest")
    }
}
