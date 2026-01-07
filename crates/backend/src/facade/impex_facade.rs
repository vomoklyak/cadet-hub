use crate::error::CadetHubBeError;
use crate::service::auth_service::AuthService;
use crate::service::cadet_impex_service::ImpexService;
use crate::service::cadet_service::CadetService;
use crate::service::excel_service::ExcelService;
use crate::CadetHubBeResult;
use common::logger::info;
use common::model::{
    ExcelTemplate, ExcelTemplateName, ExportCadetCourseResponse, ExportCadetCourseResponseBuilder,
    ImportCadetCourseRequest, ImportCadetCourseResponse, SearchCadetCourseRequest,
    WriteExcelFileRequest,
};
use common::model::{ImpexCadetCourseEntry, ReadExcelFileRequest};
use common::model::{User, UserRolePermission};
use std::sync::Arc;
use validator::Validate;

pub struct ImpexFacade {
    auth_service: Arc<AuthService>,
    cadet_service: Arc<CadetService>,
    excel_service: Arc<ExcelService>,
    impex_service: Arc<ImpexService>,
}

impl ImpexFacade {
    pub(crate) fn new(
        auth_service: Arc<AuthService>,
        cadet_service: Arc<CadetService>,
        excel_service: Arc<ExcelService>,
        impex_service: Arc<ImpexService>,
    ) -> Self {
        Self {
            auth_service,
            cadet_service,
            excel_service,
            impex_service,
        }
    }

    pub async fn get_excel_template(
        &self,
        actor_user: User,
        name: ExcelTemplateName,
    ) -> CadetHubBeResult<ExcelTemplate> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        let excel_template = self.excel_service.get_excel_template(name).await.clone();
        Ok(excel_template)
    }

    pub async fn read_excel_file(
        &self,
        actor_user: User,
        request: ReadExcelFileRequest,
    ) -> CadetHubBeResult<Vec<ImpexCadetCourseEntry>> {
        info!(
            "Start excel file read: path={:?}, sheet_number={:?}, first_column={:?}, first_row={:?}, last_row={:?}, skip_optional_validation={:?}",
            request.path(),
            request.sheet_number(),
            request.first_column(),
            request.first_row(),
            request.last_row(),
            request.skip_optional_validation()
        );
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        let entries = self
            .excel_service
            .read_impex_cadet_course_entries(request)?;
        info!(
            "Finish excel file read: number_of_entries={}",
            entries.len()
        );
        Ok(entries)
    }

    pub async fn write_excel_file(
        &self,
        actor_user: User,
        request: WriteExcelFileRequest,
        entities: Vec<ImpexCadetCourseEntry>,
    ) -> CadetHubBeResult<()> {
        info!(
            "Start excel file write: path={}, number_of_entities={}",
            request.path(),
            entities.len()
        );
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        self.excel_service
            .write_impex_cadet_course_entries(request, entities)?;
        info!("Finish excel file write");
        Ok(())
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
    use crate::service::excel_service::ExcelService;
    use crate::test::test_util::{
        actor_user_reader, actor_user_writer, create_file_if_not_exist, entry, file_content,
        impex_entry,
    };
    use common::error::CadetHubError;
    use common::model::{
        ExcelTemplateName, ImpexCadetCourseEntry, ReadExcelFileRequestBuilder,
        WriteExcelFileRequestBuilder,
    };
    use common::model::{
        ImportCadetCourseRequest, ImportCadetCourseRequestBuilder, SearchCadetCourseRequest,
        SearchCadetCourseRequestBuilder,
    };
    use spectral::iter::ContainingIntoIterAssertions;
    use spectral::prelude::VecAssertions;
    use spectral::prelude::*;
    use std::sync::Arc;

    fn sut(
        cadet_repository: MockCadetRepository,
        user_repository: MockUserRepository,
    ) -> ImpexFacade {
        let cadet_repository = Arc::new(cadet_repository);
        let user_repository = Arc::new(user_repository);
        let auth_service = Arc::new(AuthService::new(user_repository.clone()));
        let cadet_service = Arc::new(CadetService::new(cadet_repository.clone()));
        let excel_service = Arc::new(ExcelService::new());
        let impex_service = Arc::new(ImpexService::new(cadet_repository.clone()));
        ImpexFacade::new(
            auth_service.clone(),
            cadet_service.clone(),
            excel_service.clone(),
            impex_service.clone(),
        )
    }

    #[tokio::test]
    async fn should_get_excel_template() {
        // Given
        let actor_user = actor_user_reader();
        let name = ExcelTemplateName::CadetCourseReport;

        // When
        let result = sut(MockCadetRepository::new(), MockUserRepository::new())
            .get_excel_template(actor_user, name)
            .await
            .expect("failed get excel template");

        // Then
        assert_that(result.sheet_number()).is_equal_to(0);
        assert_that(result.first_column()).is_equal_to(1);
        assert_that(result.first_row()).is_equal_to(8);
        assert_that(result.data())
            .is_equal_to(file_content("src/resources/templates/cadet_course_report.xlsx").await);
    }

    #[tokio::test]
    async fn should_read_write_excel_file() {
        // Given
        let actor_user = actor_user_reader();
        let write_path =
            create_file_if_not_exist("../../target/test/impex_facade/test_cadet_courses.xlsx")
                .await;
        let read_request = ReadExcelFileRequestBuilder::default()
            .path("src/test/resources/test_cadet_courses.xlsx")
            .build()
            .expect("failed build ReadExcelFileRequest");
        let write_request = WriteExcelFileRequestBuilder::default()
            .path(write_path)
            .build()
            .expect("failed build WriteExcelFileRequest");
        let read_write_request = ReadExcelFileRequestBuilder::default()
            .path(write_request.path())
            .build()
            .expect("failed build ReadExcelFileRequest");
        let sut = sut(MockCadetRepository::new(), MockUserRepository::new());

        // When
        let read_result = sut
            .read_excel_file(actor_user.clone(), read_request)
            .await
            .expect("failed read excel file");

        let write_result = sut
            .write_excel_file(actor_user.clone(), write_request, read_result.clone())
            .await;

        let read_write_result = sut
            .read_excel_file(actor_user.clone(), read_write_request)
            .await
            .expect("failed read excel file");

        // Then
        assert_that(&read_result).has_length(2);
        assert_that(&write_result).is_ok();
        assert_that(&read_write_result).has_length(2);
        assert_that(&read_result).is_equal_to(&read_write_result);
        assert_that(&read_write_result).contains(&impex_entry(0));
        assert_that(&read_write_result).contains(&impex_entry(1));
    }

    #[tokio::test]
    //noinspection DuplicatedCode
    async fn should_import_cadet_courses() {
        // Given
        let actor_user = actor_user_writer();
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
        let actor_user = actor_user_reader();
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
        let actor_user = actor_user_writer();
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
        let actor_user = actor_user_writer();
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
        let actor_user = actor_user_reader();
        let request = cadet_course_search_request();
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_find_cadet_course_entries_by_search_request()
            .times(1)
            .return_once(move |_| Ok(vec![entry(1, "1"), entry(2, "2")]));

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
}
