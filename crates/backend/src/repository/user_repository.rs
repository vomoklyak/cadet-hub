use crate::repository::database::Database;
use crate::repository::entity::UserEntity;
use crate::repository::query::add_in_clause;
use crate::CadetHubBeResult;
use async_trait::async_trait;
use common::error::CadetHubError;
use common::logger::info;
use common::model::SearchUserRequest;
use common::model::User;
#[cfg(test)]
use mockall::automock;
use sqlx::{QueryBuilder, Sqlite};
use std::sync::Arc;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save_user(&self, user: User) -> CadetHubBeResult<User>;
    async fn update_user(&self, user: User) -> CadetHubBeResult<User>;
    async fn delete_user(&self, id: i64) -> CadetHubBeResult<()>;
    async fn find_users_by_search_request(
        &self,
        request: SearchUserRequest,
    ) -> CadetHubBeResult<Vec<User>>;
    async fn find_user_by_id(&self, id: i64) -> CadetHubBeResult<Option<User>>;
    async fn find_user_by_login(&self, login: &str) -> CadetHubBeResult<Option<User>>;
}

#[derive(Clone)]
pub(crate) struct DefaultUserRepository {
    database: Arc<Database>,
}

impl DefaultUserRepository {
    pub(crate) fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[async_trait]
impl UserRepository for DefaultUserRepository {
    async fn save_user(&self, user: User) -> CadetHubBeResult<User> {
        let mut user_entity = UserEntity::from(&user);
        info!(
            "Save user: login={:?}, role={:?}",
            user_entity.login(),
            user_entity.role()
        );
        let insert_statement = r#"
            INSERT INTO users (
                login,
                password,
                role
            ) VALUES (?, ?, ?)
        "#;
        let user_id = sqlx::query(insert_statement)
            .bind(user_entity.login().as_str())
            .bind(user_entity.password().as_str())
            .bind(user_entity.role())
            .execute(self.database.share_pool())
            .await
            .map_err(CadetHubError::general_error_with_source)?
            .last_insert_rowid();
        user_entity.set_id(Some(user_id));
        info!(
            "User saved: id={:?}, login={:?}, role={:?}",
            user_entity.id(),
            user_entity.login(),
            user_entity.role()
        );
        Ok(User::from(user_entity))
    }

    async fn update_user(&self, user: User) -> CadetHubBeResult<User> {
        let user_entity = UserEntity::from(&user);
        info!(
            "Update user: id={:?}, login={:?}, role={:?}",
            user_entity.id(),
            user_entity.login(),
            user_entity.role()
        );
        let update_statement = r#"
            UPDATE users SET
                password = ?,
                role = ?,
                updated_at = strftime('%s', 'now')
            WHERE id = ?
            RETURNING *;
        "#;
        let user = sqlx::query_as::<_, UserEntity>(update_statement)
            .bind(user_entity.password().clone())
            .bind(user_entity.role().clone())
            .bind(user_entity.id().clone())
            .fetch_one(self.database.share_pool())
            .await
            .map(User::from)
            .map_err(CadetHubError::general_error_with_source)?;
        info!(
            "User updated: id={:?}, login={:?}, role={:?}",
            user_entity.id(),
            user_entity.login(),
            user_entity.role()
        );
        Ok(user)
    }

    async fn delete_user(&self, id: i64) -> CadetHubBeResult<()> {
        info!("Delete user: id={:?}", id);
        let delete_statement = r#"
            DELETE FROM users WHERE id = ?
        "#;
        sqlx::query(delete_statement)
            .bind(id)
            .execute(self.database.share_pool())
            .await
            .map_err(CadetHubError::general_error_with_source)?;
        info!("User deleted: id={:?}", id);
        Ok(())
    }

    async fn find_users_by_search_request(
        &self,
        request: SearchUserRequest,
    ) -> CadetHubBeResult<Vec<User>> {
        info!(
            "Find users: number_of_logins={:?}, number_of_roles={:?}, page_index={}, page_size={}",
            request.logins().as_ref().map_or(0, |vector| vector.len()),
            request.roles().as_ref().map_or(0, |vector| vector.len()),
            request.page_request().page_index(),
            request.page_request().page_size(),
        );
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            SELECT * FROM users WHERE 1=1
        "#,
        );
        add_in_clause(&mut query_builder, "login", request.logins());

        let role_names = request
            .roles()
            .as_ref()
            .map(|roles| roles.iter().map(|role| role.to_str()).collect::<Vec<_>>());
        add_in_clause(&mut query_builder, "role", &role_names);

        query_builder.push(" ORDER BY id ");
        query_builder.push(" LIMIT ");
        query_builder.push_bind(request.page_request().limit());
        query_builder.push(" OFFSET ");
        query_builder.push_bind(request.page_request().offset());

        let users: Vec<User> = query_builder
            .build_query_as::<UserEntity>()
            .fetch_all(self.database.share_pool())
            .await
            .map(|entities| entities.into_iter().map(User::from).collect())
            .map_err(CadetHubError::general_error_with_source)?;
        info!("Found users: number_of_users={:?}", users.len());
        Ok(users)
    }

    async fn find_user_by_id(&self, id: i64) -> CadetHubBeResult<Option<User>> {
        info!("Find user: id={:?}", id);
        let select_statement = r#"
            SELECT * FROM users WHERE id = ?
        "#;
        let user_entity_opt = sqlx::query_as::<_, UserEntity>(select_statement)
            .bind(id)
            .fetch_optional(self.database.share_pool())
            .await
            .map_err(CadetHubError::general_error_with_source)?;
        info!("User: id={:?}, found={:?}", id, user_entity_opt.is_some());
        Ok(user_entity_opt.map(User::from))
    }

    async fn find_user_by_login(&self, login: &str) -> CadetHubBeResult<Option<User>> {
        info!("Find user: login={:?}", login);
        let select_statement = r#"
            SELECT * FROM users WHERE login = ?
        "#;
        let user_entity_opt = sqlx::query_as::<_, UserEntity>(select_statement)
            .bind(login)
            .fetch_optional(self.database.share_pool())
            .await
            .map_err(CadetHubError::general_error_with_source)?;
        info!(
            "User: login={:?}, found={}",
            login,
            user_entity_opt.is_some()
        );
        Ok(user_entity_opt.map(User::from))
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::database::Database;
    use crate::repository::user_repository::{DefaultUserRepository, UserRepository};
    use common::config::{ApplicationConfigBuilder, DatabaseConfigBuilder};
    use common::model::{
        PageRequest, PageRequestBuilder, SearchUserRequest, SearchUserRequestBuilder,
    };
    use common::model::{User, UserBuilder, UserRole};
    use spectral::prelude::*;
    use std::sync::Arc;

    async fn sut() -> DefaultUserRepository {
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
        DefaultUserRepository::new(Arc::new(database))
    }

    #[tokio::test]
    async fn should_check_default_admin() {
        // Given
        let default_admin_login = "admin".to_string();
        let sut = sut().await;

        // When
        let result = sut
            .find_user_by_login(&default_admin_login)
            .await
            .expect("failed find user by login")
            .expect("not existent user");

        // Then
        assert_that(&result.id()).is_equal_to(&Some(1));
        assert_that(&result.login()).is_equal_to(&default_admin_login);
        assert_that(&result.role()).is_equal_to(&UserRole::Admin);
    }

    #[tokio::test]
    async fn should_save_user() {
        // Given
        let user = user(2);
        let sut = sut().await;

        // When
        let result = sut
            .save_user(user.clone())
            .await
            .expect("failed to save user");

        // Then
        assert_that(&result.id()).is_equal_to(&Some(2));
        assert_that(&result.login()).is_equal_to(user.login());
        assert_that(&result.password()).is_equal_to(user.password());
        assert_that(&result.role()).is_equal_to(user.role());
    }

    #[tokio::test]
    async fn should_update_user() {
        // Given
        let sut = sut().await;
        let user = sut.save_user(user(2)).await.expect("failed to save user");

        let mut updated_user = user.clone();
        updated_user.set_password("updated_password".to_string());
        updated_user.set_role(UserRole::Reader);

        // When
        sut.update_user(updated_user.clone())
            .await
            .expect("failed to update user");

        let result = sut
            .find_user_by_id(user.require_id())
            .await
            .expect("failed to find user")
            .expect("not existent user");

        // Then
        assert_that(&result.password()).is_equal_to(updated_user.password());
        assert_that(&result.role()).is_equal_to(updated_user.role());
    }

    #[tokio::test]
    async fn should_find_user_by_id() {
        // Given
        let sut = sut().await;
        let user = sut.save_user(user(2)).await.expect("failed save user");

        // When
        let result = sut
            .find_user_by_id(user.require_id())
            .await
            .expect("failed find user by id")
            .expect("not existent user");

        // Then
        assert_that(&result).is_equal_to(&user);
    }

    #[tokio::test]
    async fn should_find_user_by_id_case_non_existent_user() {
        // Given
        let non_existent_user_id = 3;
        let sut = sut().await;
        sut.save_user(user(2)).await.expect("failed save user");

        // When
        let result = sut
            .find_user_by_id(non_existent_user_id)
            .await
            .expect("failed find user by id");

        // Then
        assert_that(&result).is_none();
    }

    #[tokio::test]
    async fn should_find_user_by_login() {
        // Given
        let sut = sut().await;
        let user = sut.save_user(user(1)).await.expect("failed save user");

        // When
        let result = sut
            .find_user_by_login(user.login())
            .await
            .expect("failed find user by login")
            .expect("not existent user");

        // Then
        assert_that(&result).is_equal_to(&user);
    }

    #[tokio::test]
    async fn should_find_user_by_login_case_non_existent_login() {
        // Given
        let non_existent_login = "non_existent_login";
        let sut = sut().await;
        sut.save_user(user(2)).await.expect("failed save user");

        // When
        let result = sut
            .find_user_by_login(non_existent_login)
            .await
            .expect("failed find user by login");

        // Then
        assert_that(&result).is_none();
    }

    #[tokio::test]
    async fn should_delete_user() {
        // Given
        let sut = sut().await;
        let user = sut.save_user(user(1)).await.expect("failed save user");

        // When
        sut.delete_user(user.require_id())
            .await
            .expect("failed delete user");

        let result = sut
            .find_user_by_id(user.require_id())
            .await
            .expect("failed find user by id");

        // Then
        assert_that(&result).is_none();
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request() {
        // Given
        let sut = sut().await;
        let user = sut.save_user(user(1)).await.expect("failed save user");
        let request = search_user_request(&user);

        // When
        let result = sut
            .find_users_by_search_request(request)
            .await
            .expect("failed find user by search request");

        // Then
        assert_that(&result).has_length(1);
        assert_that(&result).contains(&user);
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request_case_pagination() {
        // Given
        let sut = sut().await;
        sut.save_user(user(1)).await.expect("failed save user");
        sut.save_user(user(2)).await.expect("failed save user");
        let request = SearchUserRequestBuilder::default()
            .page_request(page_request(1, 0))
            .build()
            .expect("failed build SearchUserRequest");

        // When
        let result = sut
            .find_users_by_search_request(request)
            .await
            .expect("failed find user by search request");

        // Then
        assert_that(&result).has_length(1);
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request_case_another_logins() {
        // Given
        let sut = sut().await;
        let user = sut.save_user(user(1)).await.expect("failed save user");
        let mut request = search_user_request(&user);
        request.set_logins(Some(vec!["another_login".to_string()]));

        // When
        let result = sut
            .find_users_by_search_request(request)
            .await
            .expect("failed find user by search request");

        // Then
        assert_that(&result).is_empty();
    }

    #[tokio::test]
    async fn should_find_cadet_by_search_request_case_another_roles() {
        // Given
        let sut = sut().await;
        let user = sut.save_user(user(1)).await.expect("failed save user");
        let mut request = search_user_request(&user);
        request.set_roles(Some(vec![UserRole::Reader]));

        // When
        let result = sut
            .find_users_by_search_request(request)
            .await
            .expect("failed find user by search request");

        // Then
        assert_that(&result).is_empty();
    }

    fn search_user_request(user: &User) -> SearchUserRequest {
        SearchUserRequestBuilder::default()
            .logins(vec![user.login().clone()])
            .roles(vec![user.role().clone()])
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

    fn user(index: usize) -> User {
        UserBuilder::default()
            .id(None)
            .login(format!("user_login_{index}"))
            .password(format!("hashed_password_{index}"))
            .role(UserRole::Admin)
            .build()
            .expect("failed build user")
    }
}
