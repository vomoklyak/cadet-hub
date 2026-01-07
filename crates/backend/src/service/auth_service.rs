use crate::error::CadetHubBeError;
use crate::repository::user_repository::UserRepository;
use crate::CadetHubBeResult;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use common::model::{User, UserRolePermission};
use common::logger::error;
use std::sync::Arc;
use common::error::CadetHubError;

#[derive(Clone)]
pub(crate) struct AuthService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    hasher: Argon2<'static>,
}

impl AuthService {
    pub(crate) fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self {
            user_repository,
            hasher: Argon2::default(),
        }
    }

    pub(crate) async fn login(&self, login: &str, password: &str) -> CadetHubBeResult<User> {
        let user = self
            .user_repository
            .find_user_by_login(login)
            .await?
            .ok_or_else(CadetHubBeError::default_authentication_error)?;
        self.check_password(password, &user.password())?;
        Ok(user)
    }

    pub(crate) fn check_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> CadetHubBeResult<()> {
        let parsed_password_hash = PasswordHash::new(&password_hash).map_err(|error| {
            CadetHubError::general_error_with_context(format!(
                "failed parse password hash: {error:?}"
            ))
        })?;
        self.hasher
            .verify_password(password.as_bytes(), &parsed_password_hash)
            .map_err(|error| {
                error!("failed verify password {:?}", error);
                CadetHubBeError::default_authentication_error()
            })
    }

    pub(crate) fn check_permission(
        &self,
        actor_user: &User,
        required_permission: &UserRolePermission,
    ) -> CadetHubBeResult<()> {
        if !&actor_user
            .role()
            .permissions()
            .contains(required_permission)
        {
            Err(CadetHubBeError::authorization_error(format!(
                "Authorization failed: required_permission={:?}",
                required_permission
            )))
        } else {
            Ok(())
        }
    }

    pub(crate) fn hash_password(&self, password: &str) -> CadetHubBeResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        self.hasher
            .hash_password(password.as_bytes(), &salt)
            .map(|password_hash| password_hash.to_string())
            .map_err(|error| {
                CadetHubError::general_error_with_context(format!(
                    "failed hash password: {error:?}"
                )).into()
            })
    }
}
