use crate::repository::user_repository::UserRepository;
use common::model::{SearchUserRequest, UpdateUserRequest};
use common::model::{User, USER_STRUCT_NAME};
use std::sync::Arc;
use crate::CadetHubBeResult;
use crate::error::CadetHubBeError;

#[derive(Clone)]
pub(crate) struct UserService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub(crate) fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }

    pub(crate) async fn save_user(&self, user: User) -> CadetHubBeResult<User> {
        self.user_repository.save_user(user).await
    }

    pub(crate) async fn update_user(&self, request: UpdateUserRequest) -> CadetHubBeResult<User> {
        let mut db_user = self.require_user(request.id().clone()).await?;
        if let Some(password) = request.password() {
            db_user.set_password(password.to_string());
        }
        if let Some(role) = request.role() {
            db_user.set_role(role.to_owned());
        }
        self.user_repository.update_user(db_user).await
    }

    pub(crate) async fn delete_user(&self, id: i64) -> CadetHubBeResult<()> {
        let user = self.require_user(id).await?;
        self.user_repository.delete_user(user.require_id()).await
    }

    pub(crate) async fn get_users_by_search_request(
        &self,
        request: SearchUserRequest,
    ) -> CadetHubBeResult<Vec<User>> {
        self.user_repository
            .find_users_by_search_request(request)
            .await
    }

    pub(crate) async fn require_user(&self, id: i64) -> CadetHubBeResult<User> {
        self.user_repository
            .find_user_by_id(id)
            .await?
            .ok_or(CadetHubBeError::resource_not_found(
                USER_STRUCT_NAME,
                format!("{id}").as_str(),
            ))
    }
}
