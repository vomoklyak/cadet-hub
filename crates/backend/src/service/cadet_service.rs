use crate::error::CadetHubBeError;
use crate::repository::cadet_repository::CadetRepository;
use crate::CadetHubBeResult;
use common::model::{Cadet, CadetCourse, CadetCourseEntry, CadetCourseStatisticEntry, CADET_COURSE_STRUCT_NAME, CADET_STRUCT_NAME};
use common::model::{SearchCadetCourseRequest, SearchCadetRequest};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct CadetService {
    cadet_repository: Arc<dyn CadetRepository + Send + Sync>,
}

impl CadetService {
    pub(crate) fn new(cadet_repository: Arc<dyn CadetRepository + Send + Sync>) -> Self {
        Self { cadet_repository }
    }

    pub(crate) async fn create_cadet(&self, cadet: Cadet) -> CadetHubBeResult<Cadet> {
        self.cadet_repository
            .save_cadet(cadet.clone())
            .await
            .map_err(|error| Self::on_cadet_resource_conflict_error(&cadet, error))
    }

    pub(crate) async fn update_cadet(&self, cadet: Cadet) -> CadetHubBeResult<Cadet> {
        let mut db_cadet = self.require_cadet(cadet.require_id()).await?;
        db_cadet.set_tax_number(cadet.tax_number().clone());
        db_cadet.set_first_name(cadet.first_name().clone());
        db_cadet.set_middle_name(cadet.middle_name().clone());
        db_cadet.set_last_name(cadet.last_name().clone());
        db_cadet.set_birth_date(cadet.birth_date().clone());
        self.cadet_repository
            .update_cadet(db_cadet)
            .await
            .map_err(|error| Self::on_cadet_resource_conflict_error(&cadet, error))
    }

    pub(crate) async fn delete_cadet(&self, cadet_id: i64) -> CadetHubBeResult<()> {
        let cadet = self.require_cadet(cadet_id).await?;
        self.cadet_repository
            .delete_cadet(cadet.require_id())
            .await?;
        Ok(())
    }

    pub(crate) async fn count_cadet_by_search_request(
        &self,
        request: SearchCadetRequest,
    ) -> CadetHubBeResult<i64> {
        self.cadet_repository
            .count_cadet_by_search_request(request)
            .await
    }

    pub(crate) async fn get_cadets_by_search_request(
        &self,
        request: SearchCadetRequest,
    ) -> CadetHubBeResult<Vec<Cadet>> {
        self.cadet_repository
            .find_cadet_by_search_request(request)
            .await
    }

    pub(crate) async fn require_cadet(&self, cadet_id: i64) -> CadetHubBeResult<Cadet> {
        self.cadet_repository
            .find_cadet_by_id(cadet_id)
            .await?
            .ok_or(CadetHubBeError::resource_not_found(
                CADET_STRUCT_NAME,
                format!("{cadet_id}").as_str(),
            ))
    }

    fn on_cadet_resource_conflict_error(cadet: &Cadet, error: CadetHubBeError) -> CadetHubBeError {
        match error {
            CadetHubBeError::ResourceConflictError { .. } => {
                CadetHubBeError::resource_conflict_error(
                    CADET_STRUCT_NAME,
                    "tax_number",
                    cadet.tax_number(),
                )
            }
            _ => error,
        }
    }

    // CADET COURSE
    pub(crate) async fn create_cadet_course(
        &self,
        cadet_course: CadetCourse,
    ) -> CadetHubBeResult<CadetCourse> {
        self.cadet_repository
            .save_cadet_course(cadet_course.clone())
            .await
            .map_err(|error| Self::on_cadet_course_resource_conflict_error(&cadet_course, error))
    }

    pub(crate) async fn update_cadet_course(
        &self,
        cadet_course: CadetCourse,
    ) -> CadetHubBeResult<CadetCourse> {
        let mut db_cadet_course = self.require_cadet_course(cadet_course.require_id()).await?;
        db_cadet_course.set_military_rank(cadet_course.military_rank().clone());
        db_cadet_course.set_source_unit(cadet_course.source_unit().clone());
        db_cadet_course.set_specialty_name(cadet_course.specialty_name().clone());
        db_cadet_course.set_specialty_code(cadet_course.specialty_code().clone());
        db_cadet_course.set_specialty_mos_code(cadet_course.specialty_mos_code().clone());
        db_cadet_course.set_training_location(cadet_course.training_location().clone());
        db_cadet_course.set_category(cadet_course.category().clone());
        db_cadet_course.set_start_date(cadet_course.start_date().clone());
        db_cadet_course.set_end_date(cadet_course.end_date().clone());
        db_cadet_course.set_completion_order_number(cadet_course.completion_order_number().clone());
        db_cadet_course.set_completion_certificate_number(
            cadet_course.completion_certificate_number().clone(),
        );
        db_cadet_course.set_notes(cadet_course.notes().clone());
        self.cadet_repository
            .update_cadet_course(db_cadet_course)
            .await
            .map_err(|error| Self::on_cadet_course_resource_conflict_error(&cadet_course, error))
    }

    pub(crate) async fn count_cadet_courses_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<i64> {
        self.cadet_repository
            .count_cadet_course_entries_by_search_request(request)
            .await
    }

    pub(crate) async fn get_cadet_course_entries_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<Vec<CadetCourseEntry>> {
        self.cadet_repository
            .find_cadet_course_entries_by_search_request(request)
            .await
    }

    pub(crate) async fn get_cadet_course_statistic_entries_by_search_request(
        &self,
        request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<Vec<CadetCourseStatisticEntry>> {
        self.cadet_repository
            .find_cadet_course_statistic_entries_by_search_request(request)
            .await
    }

    pub(crate) async fn delete_cadet_course(&self, cadet_course_id: i64) -> CadetHubBeResult<()> {
        let cadet_course = self.require_cadet_course(cadet_course_id).await?;
        self.cadet_repository
            .delete_cadet_course(cadet_course.require_id())
            .await?;
        Ok(())
    }

    pub(crate) async fn require_cadet_course(
        &self,
        cadet_course_id: i64,
    ) -> CadetHubBeResult<CadetCourse> {
        self.cadet_repository
            .find_cadet_course_by_id(cadet_course_id)
            .await?
            .ok_or(CadetHubBeError::resource_not_found(
                CADET_COURSE_STRUCT_NAME,
                format!("{cadet_course_id}").as_str(),
            ))
    }

    fn on_cadet_course_resource_conflict_error(
        cadet_course: &CadetCourse,
        error: CadetHubBeError,
    ) -> CadetHubBeError {
        match error {
            CadetHubBeError::ResourceConflictError { .. } => {
                CadetHubBeError::resource_conflict_error(
                    CADET_COURSE_STRUCT_NAME,
                    "specialty_code",
                    cadet_course.specialty_code(),
                )
            }
            _ => error,
        }
    }
}
