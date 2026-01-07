use crate::error::CadetHubBeError;
use crate::service::auth_service::AuthService;
use crate::CadetHubBeResult;
use common::config::ApplicationConfig;
use common::error::CadetHubError;
use common::keyring;
use common::logger::info;
use common::model::{BackupDbRequest, RestoreDbRequest, User, UserRolePermission};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::task;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

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

    pub async fn import_db(
        &self,
        actor_user: User,
        request: RestoreDbRequest,
    ) -> CadetHubBeResult<()> {
        info!(
            "Start db import: path={}, db_file_name={}, encryption_key_file_name={}",
            request.path(),
            request.db_file_name(),
            request.encryption_key_file_name(),
        );
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Administrate)?;

        let config = self.config.clone();
        task::spawn_blocking(move || {
            let mut zip_file = ZipArchive::new(File::open(request.path())?)?;

            let mut encryption_key = String::new();
            zip_file
                .by_name(request.encryption_key_file_name())?
                .read_to_string(&mut encryption_key)?;
            keyring::set_admin_key(&config.service_name(), &encryption_key)?;

            let data_directory_path = config.data_directory_path()?;
            let db_file_path = config.database().file_path(data_directory_path)?;
            let mut db_file = File::create(db_file_path)?;
            let mut zip_db_file = zip_file.by_name(request.db_file_name())?;
            io::copy(&mut zip_db_file, &mut db_file)?;

            Ok::<(), CadetHubBeError>(())
        })
        .await
        .map_err(|error| CadetHubError::general_error_with_source(error))?
        .map_err(|error| CadetHubError::general_error_with_source(error))?;
        info!("Finish db import",);
        Ok(())
    }

    pub async fn export_db(
        &self,
        actor_user: User,
        request: BackupDbRequest,
    ) -> CadetHubBeResult<()> {
        info!(
            "Start db export: path={}, db_file_name={}, encryption_key_file_name={}",
            request.path(),
            request.db_file_name(),
            request.encryption_key_file_name(),
        );
        self.auth_service
            .check_permission(&actor_user, &UserRolePermission::Administrate)?;

        let config = self.config.clone();
        task::spawn_blocking(move || {
            let options: FileOptions<'_, ()> =
                FileOptions::default().compression_method(zip::CompressionMethod::Stored);
            let mut zip_file = ZipWriter::new(File::create(request.path())?);

            zip_file.start_file(request.encryption_key_file_name(), options)?;
            let encryption_key = config
                .database()
                .encryption_key(&config.service_name())?
                .unwrap_or(config.admin_encryption_key()?);
            zip_file.write_all(encryption_key.as_bytes())?;

            zip_file.start_file(request.db_file_name(), options)?;
            let data_directory_path = config.data_directory_path()?;
            let db_file_path = config.database().file_path(data_directory_path)?;
            let mut db_file = File::open(&db_file_path)?;
            io::copy(&mut db_file, &mut zip_file)?;

            zip_file.finish()?;
            Ok::<(), CadetHubBeError>(())
        })
        .await
        .map_err(|error| CadetHubError::general_error_with_source(error))?
        .map_err(|error| CadetHubError::general_error_with_source(error))?;
        info!("Finish db export");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CadetHubBeError;
    use crate::repository::user_repository::MockUserRepository;
    use crate::test::test_util::{
        actor_user_admin, actor_user_writer, file_content, temp_dir, temp_file, zip,
        zip_file_content,
    };
    use common::config::ApplicationConfigBuilder;
    use common::keyring;
    use common::keyring::{get_or_create_admin_key, set_admin_key};
    use common::model::{BackupDbRequestBuilder, RestoreDbRequestBuilder, UserRole};
    use rand::RngExt;
    use spectral::boolean::BooleanAssertions;
    use spectral::prelude::ResultAssertions;
    use spectral::*;
    use std::borrow::Cow;

    fn sut(config: ApplicationConfig, user_repository: MockUserRepository) -> AdminFacade {
        let config = Arc::new(config);
        let user_repository = Arc::new(user_repository);
        let auth_service = Arc::new(AuthService::new(user_repository.clone()));
        AdminFacade::new(config, auth_service.clone())
    }

    fn clear_keyring(service: &str) {
        if let Err(error) = keyring::delete_admin_key(service) {
            println!("{:?}", error);
        }
    }

    #[tokio::test]
    async fn should_get_admin_encryption_key() {
        // Given
        let actor_user = actor_user_admin();
        let config = default_config();
        let user_repository = MockUserRepository::new();
        clear_keyring(&config.service_name());

        // When
        let result = sut(config.clone(), user_repository)
            .get_admin_encryption_key(actor_user.clone())
            .await
            .expect("failed get admin encryption key");

        // Then
        assert_that(&result.is_empty()).is_false();
        clear_keyring(&config.service_name());
    }

    #[tokio::test]
    async fn should_get_admin_encryption_key_case_authorization_error() {
        // Given
        let mut actor_user = actor_user_admin();
        actor_user.set_role(UserRole::Writer);
        let config = default_config();
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

    #[tokio::test]
    async fn should_import_db() {
        // Given
        let actor_user = actor_user_admin();
        let encryption_key = "test-encryption-key";
        let encryption_key_old = "test-encryption-key-old";
        let db_data = "test db encrypted data";
        let data_dir = temp_dir().await.expect("failed create temp dir");
        let zip_path = data_dir.path().join("test.zip").display().to_string();
        let request = import_db_request(&zip_path);
        let config = config(Some(data_dir.path().display().to_string()));

        zip(
            &zip_path,
            vec![
                (request.db_file_name().to_string(), db_data.to_string()),
                (
                    request.encryption_key_file_name().to_string(),
                    encryption_key.to_string(),
                ),
            ],
        )
        .await
        .expect("failed zip content");

        set_admin_key(&config.service_name(), &encryption_key_old)
            .expect("failed set admin key");

        // When
        let result = sut(config.clone(), MockUserRepository::new())
            .import_db(actor_user.clone(), request)
            .await;

        // Then
        assert_that(&result).is_ok();

        let actual_db_data =
            file_content(&data_dir.path().join("data.db").display().to_string()).await;
        assert_that(&actual_db_data).is_equal_to(Cow::from(db_data.to_string().as_bytes()));

        let actual_key =
            get_or_create_admin_key(&config.service_name()).expect("failed get admin key");
        assert_that(&actual_key).is_equal_to(encryption_key.to_string());

        clear_keyring(&config.service_name());
    }

    #[tokio::test]
    async fn should_import_db_case_authorization_error() {
        // Given
        let actor_user = actor_user_writer();
        let request = import_db_request("test");
        let config = default_config();

        // When
        let result = sut(config.clone(), MockUserRepository::new())
            .import_db(actor_user.clone(), request)
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
        clear_keyring(&config.service_name());
    }

    fn import_db_request(zip_path: &str) -> RestoreDbRequest {
        RestoreDbRequestBuilder::default()
            .path(zip_path)
            .db_file_name("test_db_file_name")
            .encryption_key_file_name("test_encryption_key_file_name")
            .build()
            .expect("failed build ExportDbRequest")
    }

    #[tokio::test]
    async fn should_export_db() {
        // Given
        let actor_user = actor_user_admin();
        let db_data = "test db encrypted data";
        let encryption_key = "test-encryption-key";
        let data_dir = temp_dir().await.expect("failed create temp dir");
        temp_file(&data_dir, "data.db", db_data)
            .await
            .expect("failed create db data file");
        let data_dir_path = data_dir.path().display().to_string();
        let zip_path = data_dir.path().join("test.zip").display().to_string();
        let request = export_db_request(&zip_path);
        let config = config(Some(data_dir_path));

        set_admin_key(&config.service_name(), &encryption_key)
            .expect("failed set admin key");

        // When
        let result = sut(config.clone(), MockUserRepository::new())
            .export_db(actor_user.clone(), request.clone())
            .await;

        // Then
        assert_that(&result).is_ok();

        let actual_db_data = zip_file_content(request.path(), request.db_file_name())
            .await
            .expect("failed read zip file");
        assert_that!(&actual_db_data).is_equal_to(db_data.to_string());

        let actual_encryption_key = zip_file_content(
            request.path(),
            request.encryption_key_file_name(),
        )
        .await
        .expect("failed read zip file");
        assert_that!(&actual_encryption_key).is_equal_to(encryption_key.to_string());

        clear_keyring(&config.service_name());
    }

    #[tokio::test]
    async fn should_export_db_case_authorization_error() {
        // Given
        let actor_user = actor_user_writer();
        let config = default_config();
        let export_request = export_db_request("");

        // When
        let result = sut(config.clone(), MockUserRepository::new())
            .export_db(actor_user.clone(), export_request.clone())
            .await;

        // Then
        assert_that(&result)
            .is_err()
            .matches(|error| matches!(error, CadetHubBeError::AuthorizationError { .. }));
        clear_keyring(&config.service_name());
    }

    fn export_db_request(zip_path: &str) -> BackupDbRequest {
        BackupDbRequestBuilder::default()
            .path(zip_path)
            .db_file_name("test_db_file_name")
            .encryption_key_file_name("test_encryption_key_file_name")
            .build()
            .expect("failed build ExportDbRequest")
    }

    fn default_config() -> ApplicationConfig {
        config(None)
    }

    fn config(data_path: Option<String>) -> ApplicationConfig {
        let suffix: u32 = rand::rng().random();
        ApplicationConfigBuilder::default()
            .qualifier(format!("test_qualifier{suffix}"))
            .organization(format!("test_organization{suffix}"))
            .application(format!("test_application{suffix}"))
            .data_directory_path(data_path)
            .build()
            .expect("failed build config")
    }
}