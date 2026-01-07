use crate::error::CadetHubBeError;
use crate::service::auth_service::AuthService;
use crate::service::cadet_impex_service::ImpexService;
use crate::service::cadet_service::CadetService;
use crate::service::csv_service::CsvService;
use crate::CadetHubBeResult;
use common::cadet_hub_common_prelude::Serialize;
use common::logger::info;
use common::model::ImpexCadetCourseEntry;
use common::model::{
    ExportCadetCourseResponse, ExportCadetCourseResponseBuilder, ImportCadetCourseRequest,
    ImportCadetCourseResponse, SearchCadetCourseRequest,
};
use common::model::{User, UserRolePermission};
use serde::de::DeserializeOwned;
use std::path::Path;
use std::sync::Arc;
use validator::Validate;

pub struct ImpexFacade {
    csv_service: Arc<CsvService>,
    auth_service: Arc<AuthService>,
    cadet_service: Arc<CadetService>,
    impex_service: Arc<ImpexService>,
}

impl ImpexFacade {
    pub(crate) fn new(
        csv_service: Arc<CsvService>,
        auth_service: Arc<AuthService>,
        cadet_service: Arc<CadetService>,
        impex_service: Arc<ImpexService>,
    ) -> Self {
        Self {
            csv_service,
            auth_service,
            cadet_service,
            impex_service,
        }
    }

    pub async fn read_csv_file<T: DeserializeOwned + Validate>(
        &self,
        path: &Path,
    ) -> CadetHubBeResult<Vec<T>> {
        info!(
            "Start csv file read: path={}",
            path.to_str().unwrap_or_default()
        );
        let entries = self.csv_service.read_csv_file(path)?;
        info!("Finish csv file read: number_of_entries={}", entries.len());
        Ok(entries)
    }

    pub async fn write_to_csv_string<T: Serialize>(
        &self,
        entities: Vec<T>,
    ) -> CadetHubBeResult<String> {
        info!(
            "Start csv string write: number_of_entities={}",
            entities.len()
        );
        let csv_string = self.csv_service.write_to_csv_string(entities);
        info!("Finish csv string write");
        csv_string
    }

    pub async fn import_cadet_courses(
        &self,
        actor_user: User,
        request: ImportCadetCourseRequest,
    ) -> CadetHubBeResult<ImportCadetCourseResponse<CadetHubBeError>> {
        info!(
            "Start cadet course import: number_of_entries={}",
            request.entries().len()
        );
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Write)?;
        let number_of_entries = request.entries().len();
        let (valid_entries, mut failed_entries) = validate_entries(request.owned_entries());
        failed_entries.extend(
            self.impex_service
                .import_cadet_courses(valid_entries)
                .await?,
        );
        let failed_entries = failed_entries.into_iter().collect::<Vec<_>>();
        info!(
            "Finish cadet course import: number_of_entries={}, number_of_failed_entries={}",
            number_of_entries,
            failed_entries.len()
        );
        Ok(ImportCadetCourseResponse::new(failed_entries))
    }

    pub async fn export_cadet_courses(
        &self,
        actor_user: User,
        mut request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<ExportCadetCourseResponse> {
        info!("Start cadet course export");
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        request.validate()?;
        request.normalize();
        let entries = self
            .cadet_service
            .get_cadet_course_entries_by_search_request(request)
            .await?
            .iter()
            .map(ImpexCadetCourseEntry::from)
            .collect::<Vec<_>>();
        let response = ExportCadetCourseResponseBuilder::default()
            .entries(entries)
            .build()
            .expect("failed build ExportCadetCourseResponse");
        info!("Finish cadet course export");
        Ok(response)
    }
}

fn validate_entries(
    entries: Vec<ImpexCadetCourseEntry>,
) -> (
    Vec<ImpexCadetCourseEntry>,
    Vec<(ImpexCadetCourseEntry, CadetHubBeError)>,
) {
    let mut valid_entries = vec![];
    let mut failed_entries: Vec<(ImpexCadetCourseEntry, CadetHubBeError)> = vec![];
    for import_entry in entries.into_iter() {
        if let Err(error) = import_entry.validate().map_err(CadetHubBeError::from) {
            failed_entries.push((import_entry.clone(), error));
            continue;
        } else {
            valid_entries.push(import_entry);
        }
    }
    (valid_entries, failed_entries)
}

#[cfg(test)]
mod tests {
    use crate::error::CadetHubBeError;
    use crate::facade::impex_facade::ImpexFacade;
    use crate::repository::cadet_repository::MockCadetRepository;
    use crate::repository::user_repository::MockUserRepository;
    use crate::service::auth_service::AuthService;
    use crate::service::cadet_impex_service::ImpexService;
    use crate::service::cadet_service::CadetService;
    use crate::service::csv_service::CsvService;
    use common::error::CadetHubError;
    use common::model::{CadetCourseEntry, CadetCourseEntryBuilder, ImpexCadetCourseEntry};
    use common::model::{
        ImportCadetCourseRequest, ImportCadetCourseRequestBuilder, SearchCadetCourseRequest,
        SearchCadetCourseRequestBuilder,
    };
    use common::model::{User, UserBuilder, UserRole};
    use common::util::date_time_util;
    use spectral::iter::ContainingIntoIterAssertions;
    use spectral::prelude::VecAssertions;
    use spectral::prelude::*;
    use std::io::Write;
    use std::sync::Arc;
    use tempfile::NamedTempFile;

    fn sut(
        cadet_repository: MockCadetRepository,
        user_repository: MockUserRepository,
    ) -> ImpexFacade {
        let cadet_repository = Arc::new(cadet_repository);
        let user_repository = Arc::new(user_repository);
        let csv_service = Arc::new(CsvService::new());
        let auth_service = Arc::new(AuthService::new(user_repository.clone()));
        let cadet_service = Arc::new(CadetService::new(cadet_repository.clone()));
        let impex_service = Arc::new(ImpexService::new(cadet_repository.clone()));
        ImpexFacade::new(
            csv_service,
            auth_service.clone(),
            cadet_service.clone(),
            impex_service.clone(),
        )
    }

    #[tokio::test]
    async fn should_read_csv_file() {
        // Given
        let entries = vec![impex_entry(1), impex_entry(2), impex_entry(3)];
        let csv_file = csv_temp_file(entries.clone());
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result: Vec<ImpexCadetCourseEntry> = sut(cadet_repository, user_repository)
            .read_csv_file(csv_file.path())
            .await
            .expect("failed read csv file");

        // Then
        assert_that(&result).has_length(3);
        assert_that(&result).contains_all_of(entries.iter().by_ref());
    }

    #[tokio::test]
    async fn should_write_to_csv_file() {
        // Given
        let entries = vec![impex_entry(1), impex_entry(2), impex_entry(3)];
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .write_to_csv_string(entries.clone())
            .await
            .expect("failed write to csv");

        // Then
        assert_that(&result).is_equal_to(csv_string(entries));
    }

    #[tokio::test]
    //noinspection DuplicatedCode
    async fn should_import_cadet_courses() {
        // Given
        let actor_user = actor_user();
        let request = import_cadet_course_request(vec![impex_entry(1)]);
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_save_cadet_impex_entries()
            .times(1)
            .return_once(|_| Ok(vec![]));

        // When
        let result = sut(cadet_repository, user_repository)
            .import_cadet_courses(actor_user, request.clone())
            .await
            .expect("failed import cadet courses");

        // Then
        assert_that(result.failed_entries()).is_empty();
    }

    #[tokio::test]
    async fn should_import_cadet_courses_case_authorization_error() {
        // Given
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Reader);
        let request = import_cadet_course_request(vec![]);
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .import_cadet_courses(actor_user, request.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    //noinspection DuplicatedCode
    async fn should_import_cadet_courses_case_invalid_entry() {
        // Given
        let actor_user = actor_user();
        let mut invalid_entry = impex_entry(2);
        invalid_entry.set_tax_number("tax_number_not_correlated_with_birth_date".to_string());
        let request = import_cadet_course_request(vec![impex_entry(1), invalid_entry.clone()]);
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_save_cadet_impex_entries()
            .times(1)
            .return_once(|_| Ok(vec![]));

        // When
        let result = sut(cadet_repository, user_repository)
            .import_cadet_courses(actor_user, request.clone())
            .await
            .expect("failed import cadet courses");

        // Then
        assert_that(result.failed_entries()).has_length(1);
        assert_that(&result.failed_entries().get(0))
            .is_some()
            .matches(|(entry, error)| {
                let error_matches = matches!(error, CadetHubBeError::ValidationError { .. });
                error_matches && entry.tax_number() == invalid_entry.tax_number()
            });
    }

    #[tokio::test]
    //noinspection DuplicatedCode
    async fn should_import_cadet_courses_case_db_failed_entry() {
        // Given
        let actor_user = actor_user();
        let db_failed_entry = impex_entry(2);
        let request = import_cadet_course_request(vec![impex_entry(1), db_failed_entry.clone()]);
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_save_cadet_impex_entries()
            .times(1)
            .return_once(|entries| {
                Ok(vec![(
                    entries[1].clone(),
                    CadetHubError::general_error_with_context("failed to save test entry").into(),
                )])
            });

        // When
        let result = sut(cadet_repository, user_repository)
            .import_cadet_courses(actor_user, request.clone())
            .await
            .expect("failed import cadet courses");

        // Then
        assert_that(result.failed_entries()).has_length(1);
        assert_that(&result.failed_entries().get(0))
            .is_some()
            .matches(|(entry, error)| {
                let error_matches = matches!(error, CadetHubBeError::CadetHubError { .. });
                error_matches && entry.tax_number() == db_failed_entry.tax_number()
            });
    }

    fn import_cadet_course_request(
        entries: Vec<ImpexCadetCourseEntry>,
    ) -> ImportCadetCourseRequest {
        ImportCadetCourseRequestBuilder::default()
            .entries(entries)
            .build()
            .expect("failed build ImportCadetCourseRequest")
    }

    #[tokio::test]
    async fn should_export_cadet_courses() {
        // Given
        let actor_user = actor_user();
        let request = cadet_course_search_request();
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_find_cadet_course_entries_by_search_request()
            .times(1)
            .return_once(move |_| Ok(vec![entry(1), entry(2)]));

        // When
        let result = sut(cadet_repository, user_repository)
            .export_cadet_courses(actor_user, request.clone())
            .await
            .expect("failed export cadet courses");

        // Then
        assert_that(result.entries()).has_length(2);
        assert_that(result.entries())
            .contains_all_of(vec![impex_entry(1), impex_entry(2)].iter().by_ref());
    }

    fn cadet_course_search_request() -> SearchCadetCourseRequest {
        SearchCadetCourseRequestBuilder::default()
            .build()
            .expect("failed build CadetCourseSearchRequest")
    }

    fn actor_user() -> User {
        UserBuilder::default()
            .id(Some(1))
            .login("impex_facade_admin")
            .password("impex_facade_password")
            .role(UserRole::Admin)
            .build()
            .expect("failed build user")
    }

    fn impex_entry(index: i64) -> ImpexCadetCourseEntry {
        ImpexCadetCourseEntry::from(&entry(index))
    }

    fn entry(index: i64) -> CadetCourseEntry {
        CadetCourseEntryBuilder::default()
            .first_name(format!("impex_facade_first_name_{index}"))
            .middle_name(format!("impex_facade_middle_name_{index}"))
            .last_name(format!("impex_facade_last_name_{index}"))
            .military_rank(format!("impex_facade_military_rank_{index}"))
            .birth_date(timestamp(&format!("01/0{}/2020", index + 1)))
            // correlated with birth_date
            .tax_number(format!("4383{index}00010"))
            .source_unit(format!("impex_facade_source_unit_{index}"))
            .specialty_name(format!("impex_facade_specialty_name_{index}"))
            .specialty_code(format!("impex_facade_specialty_code_{index}"))
            .specialty_mos_code(format!("impex_facade_specialty_mos_code_{index}"))
            .category("CATEGORY")
            .training_location(format!("impex_facade_training_location_{index}"))
            .start_date(timestamp(&format!("01/02/201{index}")))
            .end_date(timestamp(&format!("01/02/202{index}")))
            .completion_order_number(format!("impex_facade_completion_order_number_{index}"))
            .completion_certificate_number(format!(
                "impex_facade_completion_certificate_number_{index}"
            ))
            .notes(format!("impex_facade_notes_{index}"))
            .build()
            .expect("failed build ImpexCadetCourseEntry")
    }

    fn timestamp(date_str: &str) -> i64 {
        date_time_util::forward_slash_m_d_y_str_as_utc_timestamp(date_str).unwrap_or_default()
    }

    fn csv_temp_file(entries: Vec<ImpexCadetCourseEntry>) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("failed create temp file");
        file.write_all(csv_string(entries).as_bytes())
            .expect("failed write to temp file");
        file
    }

    fn csv_string(entries: Vec<ImpexCadetCourseEntry>) -> String {
        CsvService::new()
            .write_to_csv_string(entries)
            .expect("failed write to CSV string")
    }
}
