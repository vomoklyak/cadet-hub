use crate::service::auth_service::AuthService;
use crate::service::cadet_service::CadetService;
use crate::CadetHubBeResult;
use common::model::{
    Cadet, CadetCourse, SearchCadetCourseStatisticResponse,
    SearchCadetCourseStatisticResponseBuilder,
};
use common::model::{
    SearchCadetCourseRequest, SearchCadetCourseResponse, SearchCadetCourseResponseBuilder,
    SearchCadetRequest, SearchCadetResponse, SearchCadetResponseBuilder,
};
use common::model::{User, UserRolePermission};
use std::sync::Arc;
use validator::Validate;

pub struct CadetFacade {
    auth_service: Arc<AuthService>,
    cadet_service: Arc<CadetService>,
}

impl CadetFacade {
    // CADET
    pub(crate) fn new(auth_service: Arc<AuthService>, cadet_service: Arc<CadetService>) -> Self {
        Self {
            auth_service,
            cadet_service,
        }
    }

    pub async fn create_cadet(
        &self,
        actor_user: User,
        mut cadet: Cadet,
    ) -> CadetHubBeResult<Cadet> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Write)?;
        cadet.validate()?;
        cadet.normalize();
        self.cadet_service.create_cadet(cadet).await
    }

    pub async fn update_cadet(
        &self,
        actor_user: User,
        mut cadet: Cadet,
    ) -> CadetHubBeResult<Cadet> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Write)?;
        cadet.validate()?;
        cadet.normalize();
        self.cadet_service.update_cadet(cadet).await
    }

    pub async fn delete_cadet(&self, actor_user: User, cadet_id: i64) -> CadetHubBeResult<()> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Write)?;
        self.cadet_service.delete_cadet(cadet_id).await?;
        Ok(())
    }

    pub async fn get_cadet(&self, actor_user: User, cadet_id: i64) -> CadetHubBeResult<Cadet> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        self.cadet_service.require_cadet(cadet_id).await
    }

    pub async fn get_cadet_by_search_request(
        &self,
        actor_user: User,
        mut request: SearchCadetRequest,
    ) -> CadetHubBeResult<SearchCadetResponse> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        request.validate()?;
        request.normalize();
        let cadets = self
            .cadet_service
            .get_cadets_by_search_request(request.clone())
            .await?;
        let total_number_of_cadets = self
            .cadet_service
            .count_cadet_by_search_request(request.clone())
            .await?;
        let response = SearchCadetResponseBuilder::default()
            .page_request(request.page_request().clone())
            .page_cadets(cadets)
            .total_number_of_cadets(total_number_of_cadets)
            .build()
            .expect("failed build SearchCadetResponse");
        Ok(response)
    }

    // CADET COURSE
    pub async fn create_cadet_course(
        &self,
        actor_user: User,
        mut cadet_course: CadetCourse,
    ) -> CadetHubBeResult<CadetCourse> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Write)?;
        cadet_course.validate()?;
        cadet_course.normalize();
        self.cadet_service.create_cadet_course(cadet_course).await
    }

    pub async fn update_cadet_course(
        &self,
        actor_user: User,
        mut cadet_course: CadetCourse,
    ) -> CadetHubBeResult<CadetCourse> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Write)?;
        cadet_course.validate()?;
        cadet_course.normalize();
        self.cadet_service.update_cadet_course(cadet_course).await
    }

    pub async fn delete_cadet_course(
        &self,
        actor_user: User,
        cadet_course_id: i64,
    ) -> CadetHubBeResult<()> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Write)?;
        self.cadet_service
            .delete_cadet_course(cadet_course_id)
            .await
    }

    pub async fn get_cadet_course(
        &self,
        actor_user: User,
        cadet_course_id: i64,
    ) -> CadetHubBeResult<CadetCourse> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        self.cadet_service
            .require_cadet_course(cadet_course_id)
            .await
    }

    pub async fn get_cadet_course_entries_by_search_request(
        &self,
        actor_user: User,
        mut request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<SearchCadetCourseResponse> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        request.validate()?;
        request.normalize();
        let entries = self
            .cadet_service
            .get_cadet_course_entries_by_search_request(request.clone())
            .await?;
        let total_number_of_entries = self
            .cadet_service
            .count_cadet_courses_by_search_request(request.clone())
            .await?;
        let response = SearchCadetCourseResponseBuilder::default()
            .page_request(request.page_request().clone())
            .page_entries(entries)
            .total_number_of_entries(total_number_of_entries)
            .build()
            .expect("failed build CadetCourseSearchResponse");
        Ok(response)
    }

    pub async fn get_cadet_course_statistic_entries_by_search_request(
        &self,
        actor_user: User,
        mut request: SearchCadetCourseRequest,
    ) -> CadetHubBeResult<SearchCadetCourseStatisticResponse> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        request.validate()?;
        request.normalize();
        let entries = self
            .cadet_service
            .get_cadet_course_statistic_entries_by_search_request(request.clone())
            .await?;
        let response = SearchCadetCourseStatisticResponseBuilder::default()
            .entries(entries)
            .build()
            .expect("failed build SearchCadetCourseStatisticResponse");
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CadetHubBeError;
    use crate::repository::cadet_repository::MockCadetRepository;
    use crate::repository::user_repository::MockUserRepository;
    use common::model::{
        CadetBuilder, CadetCourseBuilder, CadetCourseEntry, CadetCourseStatisticEntryBuilder,
        PageRequest, SearchCadetCourseRequestBuilder, SearchCadetRequestBuilder, UserBuilder,
        UserRole,
    };
    use spectral::prelude::*;

    fn sut(
        cadet_repository: MockCadetRepository,
        user_repository: MockUserRepository,
    ) -> CadetFacade {
        let user_repository = Arc::new(user_repository);
        let cadet_repository = Arc::new(cadet_repository);
        let auth_service = Arc::new(AuthService::new(user_repository.clone()));
        let cadet_service = Arc::new(CadetService::new(cadet_repository.clone()));
        CadetFacade::new(auth_service.clone(), cadet_service.clone())
    }

    #[tokio::test]
    async fn should_create_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet_id = 1;
        let mut lowercase_cadet = lowercase_cadet(cadet_id, &cadet_id.to_string());
        lowercase_cadet.set_id(None);
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_save_cadet()
            .times(1)
            .returning(move |mut cadet| {
                cadet.set_id(Some(cadet_id));
                Ok(cadet)
            });

        // When
        let result = sut(cadet_repository, user_repository)
            .create_cadet(actor_user.clone(), lowercase_cadet.clone())
            .await
            .expect("failed create cadet");

        // Then
        assert_that(&result).is_equal_to(&cadet(cadet_id, &cadet_id.to_string()));
    }

    #[tokio::test]
    async fn should_create_cadet_case_authorization_error() {
        // Given
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Reader);
        let cadet = cadet(1, "1");
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .create_cadet(actor_user.clone(), cadet.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    async fn should_create_cadet_case_validation_error() {
        // Given
        let actor_user = actor_user();
        let mut cadet = cadet(1, "1");
        cadet.set_last_name("".to_string());
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .create_cadet(actor_user.clone(), cadet.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ValidationError { .. }));
    }

    #[tokio::test]
    async fn should_update_cadet() {
        // Given
        let actor_user = actor_user();
        let update_lowercase_cadet = lowercase_cadet(1, "1_updated");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        {
            cadet_repository
                .expect_find_cadet_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(cadet(1, "1"))));
        }
        cadet_repository
            .expect_update_cadet()
            .times(1)
            .return_once(|cadet| Ok(cadet));

        // When
        let result = sut(cadet_repository, user_repository)
            .update_cadet(actor_user.clone(), update_lowercase_cadet.clone())
            .await
            .expect("failed update cadet");

        // Then
        assert_that(&result).is_equal_to(&cadet(1, "1_updated"));
    }

    #[tokio::test]
    async fn should_update_cadet_case_authorization_error() {
        // Given
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Reader);
        let cadet = cadet(1, "1");
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .update_cadet(actor_user.clone(), cadet.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    async fn should_update_cadet_case_validation_error() {
        // Given
        let actor_user = actor_user();
        let mut cadet = cadet(1, "1");
        cadet.set_last_name("".to_string());
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .update_cadet(actor_user.clone(), cadet.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ValidationError { .. }));
    }

    #[tokio::test]
    async fn should_update_cadet_case_non_existing_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet = cadet(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_find_cadet_by_id()
            .times(1)
            .return_once(move |_| Ok(None));

        // When
        let result = sut(cadet_repository, user_repository)
            .update_cadet(actor_user.clone(), cadet.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceNotFoundError { .. }));
    }

    #[tokio::test]
    async fn should_delete_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet = cadet(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        {
            let cadet = cadet.clone();
            cadet_repository
                .expect_find_cadet_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(cadet)));
        }
        cadet_repository
            .expect_delete_cadet()
            .times(1)
            .return_once(|_| Ok(()));

        // When
        let result = sut(cadet_repository, user_repository)
            .delete_cadet(actor_user.clone(), cadet.require_id())
            .await;

        // Then
        assert_that(&result).is_ok();
    }

    #[tokio::test]
    async fn should_delete_cadet_case_authorization_error() {
        // Given
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Reader);
        let cadet = cadet(1, "1");
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .delete_cadet(actor_user.clone(), cadet.require_id())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    async fn should_delete_cadet_case_non_existing_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet = cadet(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_find_cadet_by_id()
            .times(1)
            .return_once(move |_| Ok(None));

        // When
        let result = sut(cadet_repository, user_repository)
            .delete_cadet(actor_user.clone(), cadet.require_id())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceNotFoundError { .. }));
    }

    #[tokio::test]
    async fn should_get_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet = cadet(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        {
            let cadet = cadet.clone();
            cadet_repository
                .expect_find_cadet_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(cadet)));
        }

        // When
        let result = sut(cadet_repository, user_repository)
            .get_cadet(actor_user.clone(), cadet.require_id())
            .await
            .expect("failed get cadet");

        // Then
        assert_that(&result).is_equal_to(&cadet);
    }

    #[tokio::test]
    async fn should_get_cadet_case_non_existing_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet = cadet(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_find_cadet_by_id()
            .times(1)
            .return_once(move |_| Ok(None));

        // When
        let result = sut(cadet_repository, user_repository)
            .get_cadet(actor_user.clone(), cadet.require_id())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceNotFoundError { .. }));
    }

    #[tokio::test]
    async fn should_get_cadet_by_search_request() {
        // Given
        let actor_user = actor_user();
        let cadet = cadet(1, "1");
        let lowercase_cadet = lowercase_cadet(1, "1");
        let lowercase_request = search_cadet_request(&lowercase_cadet);
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_count_cadet_by_search_request()
            .times(1)
            .return_once(|_| Ok(1));
        {
            let cadet = cadet.clone();
            let response_cadet = cadet.clone();
            cadet_repository
                .expect_find_cadet_by_search_request()
                .times(1)
                .withf(move |actual_request| actual_request.eq(&search_cadet_request(&cadet)))
                .return_once(|_| Ok(vec![response_cadet]));
        }
        // When
        let result = sut(cadet_repository, user_repository)
            .get_cadet_by_search_request(actor_user.clone(), lowercase_request)
            .await
            .expect("failed search cadet by request");

        // Then
        assert_that(&result.page_request()).is_equal_to(&PageRequest::all());
        assert_that(&result.total_number_of_cadets()).is_equal_to(&1);
        assert_that(&result.page_cadets()).is_equal_to(&vec![cadet])
    }

    fn search_cadet_request(cadet: &Cadet) -> SearchCadetRequest {
        SearchCadetRequestBuilder::default()
            .page_request(PageRequest::all())
            .tax_numbers(vec![cadet.tax_number().clone()])
            .last_names(vec![cadet.last_name().clone()])
            .birth_date_after(cadet.birth_date() - 1000)
            .birth_date_before(cadet.birth_date() + 1000)
            .build()
            .expect("failed build SearchCadet")
    }

    #[tokio::test]
    async fn should_create_cadet_course() {
        // Given
        let actor_user = actor_user();
        let cadet_course_id = 1;
        let mut lowercase_cadet_course =
            lowercase_cadet_course(cadet_course_id, &cadet_course_id.to_string());
        lowercase_cadet_course.set_id(None);
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_save_cadet_course()
            .times(1)
            .returning(move |mut cadet_course| {
                cadet_course.set_id(Some(cadet_course_id));
                Ok(cadet_course)
            });

        // When
        let result = sut(cadet_repository, user_repository)
            .create_cadet_course(actor_user.clone(), lowercase_cadet_course.clone())
            .await
            .expect("failed create cadet course");

        // Then
        assert_that(&result)
            .is_equal_to(&cadet_course(cadet_course_id, &cadet_course_id.to_string()));
    }

    #[tokio::test]
    async fn should_create_cadet_course_case_authorization_error() {
        // Given
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Reader);
        let cadet_course = cadet_course(1, "1");
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .create_cadet_course(actor_user.clone(), cadet_course.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    async fn should_create_cadet_course_case_validation_error() {
        // Given
        let actor_user = actor_user();
        let mut cadet_course = cadet_course(1, "1");
        cadet_course.set_military_rank("".to_string());
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .create_cadet_course(actor_user.clone(), cadet_course)
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ValidationError { .. }));
    }

    #[tokio::test]
    async fn should_update_cadet_course() {
        // Given
        let actor_user = actor_user();
        let update_lowercase_cadet_course = lowercase_cadet_course(1, "1_updated");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        {
            cadet_repository
                .expect_find_cadet_course_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(cadet_course(1, "1"))));
        }
        cadet_repository
            .expect_update_cadet_course()
            .times(1)
            .return_once(|cadet| Ok(cadet));

        // When
        let result = sut(cadet_repository, user_repository)
            .update_cadet_course(actor_user.clone(), update_lowercase_cadet_course.clone())
            .await
            .expect("failed update cadet course");

        // Then
        assert_that(&result).is_equal_to(&cadet_course(1, "1_updated"));
    }

    #[tokio::test]
    async fn should_update_cadet_course_case_authorization_error() {
        // Given
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Reader);
        let cadet_course = cadet_course(1, "1");
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .update_cadet_course(actor_user.clone(), cadet_course.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    async fn should_update_cadet_course_case_validation_error() {
        // Given
        let actor_user = actor_user();
        let mut cadet_course = cadet_course(1, "1");
        cadet_course.set_military_rank("".to_string());
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .update_cadet_course(actor_user.clone(), cadet_course.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ValidationError { .. }));
    }

    #[tokio::test]
    async fn should_update_cadet_course_case_non_existing_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet_course = cadet_course(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_find_cadet_course_by_id()
            .times(1)
            .return_once(move |_| Ok(None));

        // When
        let result = sut(cadet_repository, user_repository)
            .update_cadet_course(actor_user.clone(), cadet_course.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceNotFoundError { .. }));
    }

    #[tokio::test]
    async fn should_delete_cadet_course() {
        // Given
        let actor_user = actor_user();
        let cadet_course = cadet_course(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        {
            let cadet_course = cadet_course.clone();
            cadet_repository
                .expect_find_cadet_course_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(cadet_course)));
        }
        cadet_repository
            .expect_delete_cadet_course()
            .times(1)
            .return_once(|_| Ok(()));

        // When
        let result = sut(cadet_repository, user_repository)
            .delete_cadet_course(actor_user.clone(), cadet_course.require_id())
            .await;

        // Then
        assert_that(&result).is_ok();
    }

    #[tokio::test]
    async fn should_delete_cadet_course_case_authorization_error() {
        // Given
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Reader);
        let cadet_course = cadet_course(1, "1");
        let user_repository = MockUserRepository::new();
        let cadet_repository = MockCadetRepository::new();

        // When
        let result = sut(cadet_repository, user_repository)
            .delete_cadet_course(actor_user.clone(), cadet_course.require_id())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    async fn should_delete_cadet_course_case_non_existing_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet_course = cadet_course(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_find_cadet_course_by_id()
            .times(1)
            .return_once(move |_| Ok(None));

        // When
        let result = sut(cadet_repository, user_repository)
            .delete_cadet_course(actor_user.clone(), cadet_course.require_id())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceNotFoundError { .. }));
    }

    #[tokio::test]
    async fn should_get_cadet_course() {
        // Given
        let actor_user = actor_user();
        let cadet_course = cadet_course(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        {
            let cadet_course = cadet_course.clone();
            cadet_repository
                .expect_find_cadet_course_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(cadet_course)));
        }

        // When
        let result = sut(cadet_repository, user_repository)
            .get_cadet_course(actor_user.clone(), cadet_course.require_id())
            .await
            .expect("failed get cadet course");

        // Then
        assert_that(&result).is_equal_to(&cadet_course);
    }

    #[tokio::test]
    async fn should_get_cadet_course_case_non_existing_cadet() {
        // Given
        let actor_user = actor_user();
        let cadet_course = cadet_course(1, "1");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_find_cadet_course_by_id()
            .times(1)
            .return_once(move |_| Ok(None));

        // When
        let result = sut(cadet_repository, user_repository)
            .get_cadet_course(actor_user.clone(), cadet_course.require_id())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceNotFoundError { .. }));
    }

    #[tokio::test]
    async fn should_get_cadet_course_entries_by_search_request() {
        // Given
        let actor_user = actor_user();
        let lowercase_request = search_cadet_course_request(&lowercase_cadet(1, "1"));
        let cadet_course_entry = CadetCourseEntry::default();
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        cadet_repository
            .expect_count_cadet_course_entries_by_search_request()
            .times(1)
            .return_once(|_| Ok(1));
        {
            let cadet_course_entry = cadet_course_entry.clone();
            cadet_repository
                .expect_find_cadet_course_entries_by_search_request()
                .times(1)
                .withf(move |actual_request| {
                    actual_request.eq(&search_cadet_course_request(&cadet(1, "1")))
                })
                .return_once(|_| Ok(vec![cadet_course_entry]));
        }
        // When
        let result = sut(cadet_repository, user_repository)
            .get_cadet_course_entries_by_search_request(actor_user.clone(), lowercase_request)
            .await
            .expect("failed search cadet course by request");

        // Then
        assert_that(&result.page_request()).is_equal_to(&PageRequest::all());
        assert_that(&result.total_number_of_entries()).is_equal_to(&1);
        assert_that(&result.page_entries()).is_equal_to(&vec![cadet_course_entry])
    }

    #[tokio::test]
    async fn should_get_cadet_course_statistic_entries_by_search_request() {
        // Given
        let actor_user = actor_user();
        let lowercase_request = search_cadet_course_request(&lowercase_cadet(1, "1"));
        let cadet_course_statistic_entry = CadetCourseStatisticEntryBuilder::default()
            .training_location("training_location")
            .specialty_name("specialty_name")
            .specialty_code("specialty_code")
            .number_of_cadet_courses(1)
            .build()
            .expect("failed build CadetCourseStatisticEntry");
        let user_repository = MockUserRepository::new();
        let mut cadet_repository = MockCadetRepository::new();
        {
            let cadet_course_statistic_entry = cadet_course_statistic_entry.clone();
            cadet_repository
                .expect_find_cadet_course_statistic_entries_by_search_request()
                .times(1)
                .withf(move |actual_request| {
                    actual_request.eq(&search_cadet_course_request(&cadet(1, "1")))
                })
                .return_once(|_| Ok(vec![cadet_course_statistic_entry]));
        }
        // When
        let result = sut(cadet_repository, user_repository)
            .get_cadet_course_statistic_entries_by_search_request(
                actor_user.clone(),
                lowercase_request,
            )
            .await
            .expect("failed search cadet course statistic entries by request");

        // Then
        assert_that(&result.entries()).is_equal_to(&vec![cadet_course_statistic_entry])
    }

    fn lowercase_cadet(id: i64, index: &str) -> Cadet {
        let mut cadet = cadet(id, index);
        cadet.set_tax_number(cadet.tax_number().to_lowercase());
        cadet.set_first_name(cadet.first_name().to_lowercase());
        cadet.set_middle_name(cadet.middle_name().to_lowercase());
        cadet.set_last_name(cadet.last_name().to_lowercase());
        cadet
    }

    fn cadet(id: i64, index: &str) -> Cadet {
        CadetBuilder::default()
            .id(id)
            .tax_number(format!("tax_number_{index}").to_uppercase())
            .first_name(format!("First_name_{index}"))
            .middle_name(format!("Middle_name_{index}"))
            .last_name(format!("Last_name_{index}"))
            .birth_date(id * 1000)
            .build()
            .expect("failed build Cadet")
    }

    fn search_cadet_course_request(cadet: &Cadet) -> SearchCadetCourseRequest {
        SearchCadetCourseRequestBuilder::default()
            .page_request(PageRequest::all())
            .tax_numbers(vec![cadet.tax_number().clone()])
            .last_names(vec![cadet.last_name().clone()])
            .categories(vec!["CATEGORY".to_string()])
            .birth_date_after(cadet.birth_date() - 1000)
            .birth_date_before(cadet.birth_date() + 1000)
            .start_date_after(0)
            .start_date_before(1000)
            .end_date_after(2000)
            .end_date_before(3000)
            .build()
            .expect("failed build SearchCadet")
    }

    fn lowercase_cadet_course(id: i64, index: &str) -> CadetCourse {
        let mut cadet_course = cadet_course(id, index);
        cadet_course.set_military_rank(cadet_course.military_rank().to_lowercase());
        cadet_course.set_category(cadet_course.category().to_lowercase());
        cadet_course
    }

    fn cadet_course(id: i64, index: &str) -> CadetCourse {
        CadetCourseBuilder::default()
            .id(id)
            .cadet_id(Some(id))
            .military_rank(format!("cadet_facade_military_rank_{index}"))
            .source_unit(format!("cadet_facade_source_unit_{index}"))
            .specialty_name(format!("cadet_facade_specialty_name_{index}"))
            .specialty_code(format!("cadet_facade_specialty_code_{index}"))
            .specialty_mos_code(format!("cadet_facade_specialty_mos_code_{index}"))
            .category("CATEGORY")
            .training_location(format!("cadet_facade_training_location_{index}"))
            .start_date(1000)
            .end_date(2000)
            .completion_order_number(format!("cadet_facade_completion_order_number_{index}"))
            .completion_certificate_number(format!(
                "cadet_facade_completion_certificate_number_{index}"
            ))
            .notes(format!("cadet_facade_notes_{index}"))
            .build()
            .expect("failed build CadetCourse")
    }

    fn actor_user() -> User {
        UserBuilder::default()
            .id(Some(1))
            .login("cadet_facade_admin")
            .password("cadet_facade_password")
            .role(UserRole::Admin)
            .build()
            .expect("failed build user")
    }
}
