use crate::service::auth_service::AuthService;
use crate::CadetHubBeResult;
use common::config::ApplicationConfig;
use common::model::{User, UserRolePermission};
use std::sync::Arc;

pub struct AdminFacade {
    config: Arc<ApplicationConfig>,
    auth_service: Arc<AuthService>,
}

impl AdminFacade {
    pub(crate) fn new(config: Arc<ApplicationConfig>, auth_service: Arc<AuthService>) -> Self {
        Self {
            config,
            auth_service,
        }
    }

    pub async fn get_admin_encryption_key(&self, actor_user: User) -> CadetHubBeResult<String> {
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Administrate)?;
        self.config
            .admin_encryption_key()
            .map_err(|error| error.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CadetHubBeError;
    use crate::repository::user_repository::MockUserRepository;
    use common::config::ApplicationConfigBuilder;
    use common::keyring;
    use common::model::{UserBuilder, UserRole};
    use spectral::boolean::BooleanAssertions;
    use spectral::prelude::ResultAssertions;
    use spectral::*;

    fn sut(config: ApplicationConfig, user_repository: MockUserRepository) -> AdminFacade {
        let config = Arc::new(config);
        let user_repository = Arc::new(user_repository);
        let auth_service = Arc::new(AuthService::new(user_repository.clone()));
        AdminFacade::new(config, auth_service.clone())
    }

    fn clear_keyring(service: &str, user: &str) {
        if let Err(error) = keyring::delete_key(service, user) {
            println!("{:?}", error);
        }
    }

    #[tokio::test]
    async fn should_get_admin_encryption_key() {
        // Given
        let actor_user = actor_user();
        let config = config();
        let user_repository = MockUserRepository::new();
        clear_keyring(&config.service_name(), "admin");

        // When
        let result = sut(config.clone(), user_repository)
            .get_admin_encryption_key(actor_user.clone())
            .await
            .expect("failed get admin encryption key");

        // Then
        assert_that(&result.is_empty()).is_false();
        clear_keyring(&config.service_name(), "admin");
    }

    #[tokio::test]
    async fn should_get_admin_encryption_key_case_authorization_error() {
        // Given
        let mut actor_user = actor_user();
        actor_user.set_role(UserRole::Writer);
        let config = config();
        let user_repository = MockUserRepository::new();

        // When
        let result = sut(config, user_repository)
            .get_admin_encryption_key(actor_user.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
    }

    fn config() -> ApplicationConfig {
        ApplicationConfigBuilder::default()
            .qualifier("test")
            .organization("organization")
            .application("application")
            .build()
            .expect("failed build config")
    }

    fn actor_user() -> User {
        UserBuilder::default()
            .id(Some(1))
            .login("admin_facade_admin")
            .password("admin_facade_password")
            .role(UserRole::Admin)
            .build()
            .expect("failed build user")
    }
}