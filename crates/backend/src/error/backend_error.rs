use common::error::CadetHubError;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum CadetHubBeError {
    #[error("{reason}")]
    AuthenticationError { reason: String },

    #[error("{reason}")]
    AuthorizationError { reason: String },

    #[error("resource not found: name={name}, id={id}")]
    ResourceNotFoundError { name: String, id: String },

    #[error("resource already exists: name={name}, {unique_key_name}={unique_key_value}")]
    ResourceConflictError {
        name: String,
        unique_key_name: String,
        unique_key_value: String,
    },

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    CadetHubError(#[from] CadetHubError),
}

impl CadetHubBeError {
    pub fn pretty_debug_str(&self) -> String {
        format!("{}", self)
    }

    pub fn authentication_error<S>(reason: S) -> CadetHubBeError
    where
        S: Into<String>,
    {
        CadetHubBeError::AuthenticationError {
            reason: reason.into(),
        }
    }

    pub fn default_authentication_error() -> CadetHubBeError {
        CadetHubBeError::AuthenticationError {
            reason: "Authentication failed: incorrect login or password".into(),
        }
    }

    pub fn authorization_error<S>(reason: S) -> CadetHubBeError
    where
        S: Into<String>,
    {
        CadetHubBeError::AuthorizationError {
            reason: reason.into(),
        }
    }

    pub fn resource_not_found<S>(name: S, id: S) -> CadetHubBeError
    where
        S: Into<String>,
    {
        CadetHubBeError::ResourceNotFoundError {
            name: name.into(),
            id: id.into(),
        }
    }

    pub fn default_resource_conflict_error() -> CadetHubBeError {
        Self::resource_conflict_error("", "", "")
    }

    pub fn resource_conflict_error<S, K, V>(
        name: S,
        unique_key_name: K,
        unique_key_value: V,
    ) -> CadetHubBeError
    where
        S: Into<String>,
        K: Into<String>,
        V: Into<String>,
    {
        CadetHubBeError::ResourceConflictError {
            name: name.into(),
            unique_key_name: unique_key_name.into(),
            unique_key_value: unique_key_value.into(),
        }
    }
}
