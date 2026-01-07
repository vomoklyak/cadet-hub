use crate::config::ApplicationConfig;
use crate::CadetHubResult;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_rolling_file::RollingFileAppenderBaseBuilder;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logger(config: &ApplicationConfig) -> CadetHubResult<WorkerGuard> {
    let data_directory_path = config.data_directory_path()?;
    let logger_config = config.logger();
    let file_appender = RollingFileAppenderBaseBuilder::default()
        .filename(logger_config.logger_file_path(data_directory_path)?)
        .condition_max_file_size(logger_config.max_file_size_bytes().clone() as u64)
        .max_filecount(logger_config.max_number_of_files())
        .build()
        .expect("failed initialize log file");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();
    Ok(guard)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ApplicationConfigBuilder, LoggerConfigBuilder};
    use crate::logger::info;
    use spectral::prelude::*;
    use std::fs;
    use tempfile::*;

    #[test]
    fn test_logger_initialization_logic() {
        // Given
        let tmp_dir = tempdir().expect("failed create temp dir");
        let log_path = tmp_dir.path().join("test.log");
        let logger_config = LoggerConfigBuilder::default()
            .level("info".to_string())
            .file_name(log_path.to_str().unwrap_or_default().to_string())
            .max_file_size_bytes(60)
            .max_number_of_files(2)
            .build()
            .expect("failed build LoggerConfig");
        let config = ApplicationConfigBuilder::default()
            .data_directory_path(tmp_dir.path().to_str().unwrap_or_default().to_string())
            .logger(logger_config)
            .build()
            .expect("failed build ApplicationConfig");
        let message = "test log message";

        // When
        let guard = init_logger(&config);
        info!(message);
        info!(message);
        info!(message);
        info!(message);
        drop(guard);
        let number_of_log_files = fs::read_dir(tmp_dir.path())
            .expect("failed read test log dir")
            .count();
        let log_file_content = fs::read_to_string(&log_path).expect("failed read test log file");

        // Then
        assert_that!(&number_of_log_files).is_equal_to(2);
        assert_that!(&log_file_content)
            .ends_with("INFO common::logger::logger::tests: test log message\n");
    }
}
