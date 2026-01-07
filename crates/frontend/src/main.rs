#![windows_subsystem = "windows"]

mod client;
mod component;
mod context;
mod element;
mod error;
mod extension;
mod router;
mod symbol;
mod util;
mod view;

use crate::error::frontend_error::CadetHubFeError;
use crate::view::application::Application;
use backend::context::BeApplicationContext;
use common::config::ApplicationConfig;
use common::logger::init_logger;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use std::process;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub type CadetHubFeResult<T> = std::result::Result<T, CadetHubFeError>;

fn main() {
    let config = ApplicationConfig::load()
        .map(Arc::new)
        .expect("failed load config");

    let _logger_worker_guard = init_logger(&config).expect("failed to initialize logger");
    info!("Process id {}", process::id());
    info!("Logger initiated");

    // init backend
    let backend_runtime = Runtime::new().expect("failed create Tokio runtime");
    let backend = backend_runtime
        .block_on(async { Arc::new(BeApplicationContext::init(config.clone()).await) });
    info!("Backend application context initiated");

    // init frontend
    let data_directory_path = config
        .data_directory_path()
        .expect("failed init data directory");
    let window = WindowBuilder::new()
        .with_title("cadet-hub")
        .with_maximized(true);
    let desktop_config = Config::default()
        .with_window(window)
        .with_data_directory(data_directory_path);
    LaunchBuilder::desktop()
        .with_cfg(desktop_config)
        .with_context(backend)
        .launch(Application);
}
