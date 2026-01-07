use crate::service::auth_service::AuthService;
use crate::service::user_service::UserService;
use crate::CadetHubBeResult;
use common::model::{SearchUserRequest, UpdateUserRequest};
use common::model::{User, UserRolePermission};
use std::sync::Arc;
use validator::Validate;

pub struct UserFacade {
    auth_service: Arc<AuthService>,
    user_service: Arc<UserService>,
}

impl UserFacade {
    pub(crate) fn new(auth_service: Arc<AuthService>, user_service: Arc<UserService>) -> Self {
        Self {
            auth_service,
            user_service,
        }
    }

    pub async fn create_user(&self, actor_user: User, mut user: User) -> CadetHubBeResult<User> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Administrate)?;
        user.validate()?;
        user.normalize();
        let hashed_password = self.auth_service.hash_password(user.password())?;
        user.set_password(hashed_password);
        self.user_service.save_user(user).await
    }

    pub async fn update_user(
        &self,
        actor_user: User,
        mut request: UpdateUserRequest,
    ) -> CadetHubBeResult<User> {
        if !actor_user.has_admin_role() && !actor_user.has_id(request.id().clone()) {
            request.set_password(None);
        } else {
            if let Some(password) = request.password() {
                let hashed_password = self.auth_service.hash_password(&password)?;
                request.set_password(Some(hashed_password));
            }
        }
        if !actor_user.has_admin_role() {
            request.set_role(None);
        }
        self.user_service.update_user(request).await
    }

    pub async fn delete_user(&self, actor_user: User, id: i64) -> CadetHubBeResult<()> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Administrate)?;
        self.user_service.delete_user(id).await
    }

    pub async fn get_users_by_search_request(
        &self,
        actor_user: User,
        request: SearchUserRequest,
    ) -> CadetHubBeResult<Vec<User>> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        self.user_service.get_users_by_search_request(request).await
    }

    pub async fn get_user(&self, actor_user: User, id: i64) -> CadetHubBeResult<User> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Read)?;
        self.user_service.require_user(id).await
    }

    pub async fn login(&self, login: &str, password: &str) -> CadetHubBeResult<User> {
        self.auth_service
            .login(&login.to_lowercase(), password)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CadetHubBeError;
    use crate::repository::user_repository::MockUserRepository;
    use common::model::{
        PageRequest, SearchUserRequestBuilder, UpdateUserRequestBuilder,
    };
    use common::model::{UserBuilder, UserRole};
    use spectral::prelude::*;

    fn sut(user_repository: MockUserRepository) -> UserFacade {
        let user_repository = Arc::new(user_repository);
        let auth_service = Arc::new(AuthService::new(user_repository.clone()));
        let user_service = Arc::new(UserService::new(user_repository.clone()));
        UserFacade::new(auth_service.clone(), user_service.clone())
    }

    #[tokio::test]
    async fn should_create_user() {
        // Given
        let user_id = 2;
        let mut user = user(2);
        user.set_id(None);
        let actor_user = actor_user();
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_save_user()
            .times(1)
            .returning(move |mut user| {
                user.set_id(Some(user_id.clone()));
                Ok(user)
            });

        // When
        let result = sut(user_repository)
            .create_user(actor_user.clone(), user.clone())
            .await
            .expect("failed create user");

        // Then
        assert_that(result.id()).is_equal_to(&Some(user_id));
        assert_that(result.login()).is_equal_to(user.login());
        assert_that(result.password()).starts_with("$argon2id$v=19$m=19456,t=2");
        assert_that(&result.role()).is_equal_to(user.role());
    }

    #[tokio::test]
    async fn should_create_user_case_authorization_error() {
        // Given
        let user = user(2);
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Writer);
        let sut = sut(MockUserRepository::new());

        // When
        let result = sut.create_user(actor_user.clone(), user.clone()).await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    async fn should_create_user_case_validation_error() {
        // Given
        let mut user = user(2);
        user.set_login("".to_lowercase());
        let actor_user = actor_user();
        let sut = sut(MockUserRepository::new());

        // When
        let result = sut.create_user(actor_user, user).await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ValidationError { .. }));
    }

    #[tokio::test]
    async fn should_update_user() {
        // Given
        let user = user(2);
        let actor_user = actor_user();
        let new_role = UserRole::Writer;
        let request = update_user_request(user.require_id(), new_role.clone());
        let mut user_repository = MockUserRepository::new();
        {
            let user = user.clone();
            user_repository
                .expect_find_user_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(user)));
        }
        user_repository
            .expect_update_user()
            .times(1)
            .return_once(move |user| Ok(user));

        // When
        let result = sut(user_repository)
            .update_user(actor_user.clone(), request)
            .await
            .expect("failed update user");

        // Then
        assert_that(result.id()).is_equal_to(user.id());
        assert_that(result.login()).is_equal_to(user.login());
        assert_that(result.password()).starts_with("$argon2id$v=19$m=19456,t=2");
        assert_that(&result.role()).is_equal_to(&new_role);
    }

    #[tokio::test]
    async fn should_update_user_case_empty_request() {
        // Given
        let user = user(2);
        let actor_user = actor_user();
        let request = UpdateUserRequestBuilder::default()
            .id(user.require_id())
            .build()
            .expect("failed to build UpdateUserRequest");
        let mut user_repository = MockUserRepository::new();
        {
            let user = user.clone();
            user_repository
                .expect_find_user_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(user)));
        }
        user_repository
            .expect_update_user()
            .times(1)
            .return_once(move |user| Ok(user));

        // When
        let result = sut(user_repository)
            .update_user(actor_user.clone(), request)
            .await
            .expect("failed update user");

        // Then
        assert_that(result.id()).is_equal_to(user.id());
        assert_that(result.login()).is_equal_to(user.login());
        assert_that(result.password()).is_equal_to(user.password());
        assert_that(&result.role()).is_equal_to(user.role());
    }

    #[tokio::test]
    async fn should_update_user_case_self_update() {
        // Given
        let new_role = UserRole::Reader;
        let mut user = user(2);
        user.set_role(UserRole::Writer);
        let mut actor_user = user.clone();
        actor_user.set_role(UserRole::Reader);
        let request = update_user_request(user.require_id(), new_role.clone());
        let mut user_repository = MockUserRepository::new();
        {
            let user = user.clone();
            user_repository
                .expect_find_user_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(user)));
        }
        user_repository
            .expect_update_user()
            .times(1)
            .return_once(move |user| Ok(user));

        // When
        let result = sut(user_repository)
            .update_user(actor_user.clone(), request)
            .await
            .expect("failed update user");

        // Then
        assert_that(result.id()).is_equal_to(user.id());
        assert_that(result.login()).is_equal_to(user.login());
        assert_that(result.password()).starts_with("$argon2id$v=19$m=19456,t=2");
        assert_that(&result.role()).is_equal_to(user.role());
    }
    fn update_user_request(user_id: i64, role: UserRole) -> UpdateUserRequest {
        UpdateUserRequestBuilder::default()
            .id(user_id.clone())
            .password(Some("new_password".to_string()))
            .role(role.clone())
            .build()
            .expect("failed to build UpdateUserRequest")
    }

    #[tokio::test]
    async fn should_delete_user() {
        // Given
        let user = user(2);
        let actor_user = actor_user();
        let mut user_repository = MockUserRepository::new();
        {
            let user = user.clone();
            user_repository
                .expect_find_user_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(user)));
        }
        user_repository
            .expect_delete_user()
            .times(1)
            .return_once(move |_| Ok(()));

        // When
        let result = sut(user_repository)
            .delete_user(actor_user.clone(), user.require_id())
            .await;

        // Then
        assert_that(&result).is_ok();
    }

    #[tokio::test]
    async fn should_delete_user_case_authorization_error() {
        // Given
        let user = user(2);
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Writer);

        // When
        let result = sut(MockUserRepository::new())
            .create_user(actor_user.clone(), user.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    #[tokio::test]
    async fn should_get_users_by_search_request() {
        // Given
        let user = user(2);
        let actor_user = actor_user();
        let request = SearchUserRequestBuilder::default()
            .logins(vec![user.login().to_string()])
            .roles(vec![user.role().clone()])
            .page_request(PageRequest::all())
            .build()
            .expect("failed build SearchUserRequest");
        let mut user_repository = MockUserRepository::new();
        {
            let user = user.clone();
            user_repository
                .expect_find_users_by_search_request()
                .times(1)
                .return_once(move |_| Ok(vec![user]));
        }

        // When
        let result = sut(user_repository)
            .get_users_by_search_request(actor_user.clone(), request)
            .await
            .expect("failed get user by search request");

        // Then
        assert_that(&result).has_length(1);
        assert_that(&result).contains(user);
    }

    #[tokio::test]
    async fn should_get_user() {
        // Given
        let user = user(2);
        let actor_user = actor_user();
        let mut user_repository = MockUserRepository::new();
        {
            let user = user.clone();
            user_repository
                .expect_find_user_by_id()
                .times(1)
                .return_once(move |_| Ok(Some(user)));
        }

        // When
        let result = sut(user_repository)
            .get_user(actor_user.clone(), user.require_id())
            .await
            .expect("failed get user");

        // Then
        assert_that(&result).is_equal_to(&user);
    }

    #[tokio::test]
    async fn should_get_user_case_non_existing_user() {
        // Given
        let actor_user = actor_user();
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_user_by_id()
            .times(1)
            .return_once(move |_| Ok(None));

        // When
        let result = sut(user_repository).get_user(actor_user.clone(), 2).await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::ResourceNotFoundError { .. }));
    }

    #[tokio::test]
    async fn should_login() {
        // Given
        let mut user = user(2);
        let login = user.login().clone();
        let password = user.password().clone();
        let user_hashed_password = "$argon2id$v=19$m=19456,t=2,p=1$RvrHNT4wvJv018Y0CzE71A$upfY+KCzBfd1DFGuk2ggYXFr0qO1FwKT/CDiNvs/6To";
        user.set_password(user_hashed_password.to_string());
        let mut user_repository = MockUserRepository::new();
        {
            let user = user.clone();
            user_repository
                .expect_find_user_by_login()
                .times(1)
                .return_once(move |_| Ok(Some(user)));
        }

        // When
        let result = sut(user_repository)
            .login(&login, &password)
            .await
            .expect("failed get user");

        // Then
        assert_that(&result).is_equal_to(&user);
    }

    #[tokio::test]
    async fn should_login_case_incorrect_login() {
        // Given
        let mut user_repository = MockUserRepository::new();
        user_repository
            .expect_find_user_by_login()
            .times(1)
            .return_once(move |_| Ok(None));

        // When
        let result = sut(user_repository).login("login", "password").await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthenticationError { .. }));
    }

    #[tokio::test]
    async fn should_login_case_incorrect_password() {
        // Given
        let mut user = user(2);
        let login = user.login().clone();
        let password = user.password().clone();
        let user_hashed_password = "$argon2id$v=19$m=19456,t=2,p=1$RvrHNT4wvJv018Y0CzE71A$upfY+KCfBfd1DFGuk2ggYXFr0qO1FwKT/CDiNvs/6To";
        user.set_password(user_hashed_password.to_string());
        let mut user_repository = MockUserRepository::new();
        {
            let user = user.clone();
            user_repository
                .expect_find_user_by_login()
                .times(1)
                .return_once(move |_| Ok(Some(user)));
        }

        // When
        let result = sut(user_repository).login(&login, &password).await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthenticationError { .. }));
    }

    fn user(index: usize) -> User {
        UserBuilder::default()
            .id(index as i64)
            .login(format!("facade_user_login_{index}"))
            .password(format!("facade_hashed_password_{index}"))
            .role(UserRole::Reader)
            .build()
            .expect("failed build user")
    }

    fn actor_user() -> User {
        UserBuilder::default()
            .id(Some(1))
            .login("user_facade_admin")
            .password("user_facade_password")
            .role(UserRole::Admin)
            .build()
            .expect("failed build user")
    }
}
