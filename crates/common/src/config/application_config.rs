use crate::cadet_hub_common_prelude::*;
use crate::error::CadetHubError;
use crate::CadetHubResult;
use config::{Config, File};
use derive_builder::Builder;
use directories::ProjectDirs;
use std::path::PathBuf;

#[derive(Default, Debug, Getters, Setters, Builder, Serialize, Deserialize, Clone, PartialEq)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "kebab-case")]
#[builder(default)]
#[builder(setter(into))]
pub struct ApplicationConfig {
    qualifier: String,
    organization: String,
    application: String,
    #[getset(skip)]
    data_directory_path: Option<String>,
    #[serde(default)]
    logger: LoggerConfig,
    #[serde(default)]
    database: DatabaseConfig,
}

impl ApplicationConfig {
    pub fn load() -> CadetHubResult<Self> {
        let env = std::env::var("CADET_HUB_ENV").unwrap_or_else(|_| "prod".into());
        let env_application_config_path = format!("configs/application-{env}.yaml");
        let config = Config::builder()
            .add_source(File::with_name("configs/application.yaml"))
            .add_source(File::with_name("configs/application-local.yaml").required(false))
            .add_source(File::with_name(&env_application_config_path).required(false))
            .add_source(config::Environment::with_prefix("CADET_HUB").separator("_"))
            .build()
            .map_err(CadetHubError::general_error_with_source)?;
        config
            .try_deserialize()
            .map_err(CadetHubError::general_error_with_source)
    }


    pub fn admin_encryption_key(&self) -> CadetHubResult<String> {
        crate::keyring::get_or_create_admin_key(self.service_name().as_str())
    }

    pub fn service_name(&self) -> String {
        format!(
            "{}.{}.{}",
            self.qualifier, self.organization, self.application
        )
    }

    pub fn data_directory_path(&self) -> CadetHubResult<PathBuf> {
        match self.data_directory_path {
            Some(ref data_directory_path) => Ok(data_directory_path.into()),
            None => self.init_default_data_directory_path(),
        }
    }

    fn init_default_data_directory_path(&self) -> CadetHubResult<PathBuf> {
        let proj_dirs =
            ProjectDirs::from(self.qualifier(), self.organization(), self.application()).ok_or(
                CadetHubError::general_error_with_context(
                    "failed determine system application directory",
                ),
            )?;
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(proj_dirs.data_dir())
            .map_err(CadetHubError::general_error_with_source)?;
        Ok(data_dir.to_path_buf())
    }
}

#[derive(Default, Debug, Getters, Setters, Builder, Serialize, Deserialize, Clone, PartialEq)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "kebab-case")]
pub struct LoggerConfig {
    level: String,
    file_name: String,
    max_file_size_bytes: usize,
    #[getset(skip)]
    max_number_of_files: usize,
}

impl LoggerConfig {
    pub fn logger_file_path(&self, data_directory_path: PathBuf) -> CadetHubResult<String> {
        data_directory_path
            .join(self.file_name())
            .to_str()
            .map(|string| string.to_string())
            .ok_or_else(|| {
                CadetHubError::general_error_with_context("failed convert path to UTF-8 string")
            })
    }

    pub fn max_number_of_files(&self) -> usize {
        1.max(self.max_number_of_files - 1)
    }
}

#[derive(Default, Debug, Getters, Setters, Builder, Serialize, Deserialize, Clone, PartialEq)]
#[getset(get = "pub", set = "pub")]
#[serde(rename_all = "kebab-case")]
pub struct DatabaseConfig {
    #[getset(skip)]
    url: Option<String>,
    encryption_enabled: bool,
    #[getset(skip)]
    encryption_key: Option<String>,
}

impl DatabaseConfig {
    pub fn url(&self, data_directory_path: PathBuf) -> CadetHubResult<String> {
        match self.url {
            Some(ref url) => Ok(url.into()),
            None => data_directory_path
                .join("data.db")
                .to_str()
                .map(|path_str| format!("sqlite:{}", path_str))
                .ok_or(
                    CadetHubError::general_error_with_source("failed convert path to UTF-8 string")
                        .into(),
                ),
        }
    }

    pub fn encryption_key(&self, service_name: &str) -> CadetHubResult<Option<String>> {
        if self.encryption_enabled {
            match self.encryption_key {
                Some(ref encryption_key) => Ok(Some(encryption_key.clone())),
                None => crate::keyring::get_or_create_admin_key(service_name).map(Some),
            }
        } else {
            Ok(None)
        }
    }
}
