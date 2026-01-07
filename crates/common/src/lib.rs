use crate::error::CadetHubError;

pub mod cadet_hub_common_prelude {
    pub use ::derive_builder::Builder;
    pub use ::getset::{Getters, Setters};
    pub use ::serde::{Deserialize, Serialize};
    pub use ::validator::{Validate, ValidationError};
}

pub type CadetHubResult<T> = Result<T, CadetHubError>;

pub mod config;
pub mod error;
pub mod logger;
pub mod mapper;
pub mod model;
pub mod util;
pub mod validator;
pub mod keyring;
