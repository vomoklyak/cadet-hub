use crate::error::frontend_error::CadetHubFeError::GeneralFeError;
use crate::extension::i18n_extension::I18nExtension;
use backend::error::CadetHubBeError;
use common::cadet_hub_common_prelude::*;
use dioxus_i18n::prelude::I18n;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CadetHubFeError {
    #[error("fe error occurred: {code:?}, message={message:?}")]
    GeneralFeError {
        code: CadetHubFeErrorCode,
        message: String,
        details: Option<HashMap<String, Vec<String>>>,
        source_error: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CadetHubFeErrorCode {
    AuthenticationError,
    AuthorizationError,
    CommonBeError,
    CommonFeError,
    ValidationError,
    ResourceConflictError,
    ResourceNotFoundError,
}

impl CadetHubFeError {
    pub fn fe_common_error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        GeneralFeError {
            code: CadetHubFeErrorCode::CommonFeError,
            message: message.into(),
            details: None,
            source_error: None,
        }
    }

    pub fn fe_common_error_with_source<S, M>(message: S, source_error: M) -> Self
    where
        S: Into<String>,
        M: Into<String>,
    {
        GeneralFeError {
            code: CadetHubFeErrorCode::CommonFeError,
            message: message.into(),
            details: None,
            source_error: Some(source_error.into()),
        }
    }

    pub fn source_error(&self) -> &Option<String> {
        match self {
            GeneralFeError { source_error, .. } => source_error,
        }
    }

    pub fn localized_message(&self, i18n: &I18n) -> String {
        match self {
            GeneralFeError {
                code: CadetHubFeErrorCode::ValidationError,
                message,
                details,
                ..
            } => {
                let mut message = format!("{}:", i18n.try_translate_or_default(message));
                if let Some(details) = details {
                    details.iter().for_each(|(key, value_keys)| {
                        let values = value_keys
                            .iter()
                            .map(|value_key| i18n.try_translate_or_default(value_key))
                            .collect::<Vec<_>>()
                            .join(", ");
                        message.push_str(
                            format!(" ({}: {})", i18n.try_translate_or_default(key), values)
                                .as_str(),
                        );
                    })
                }
                message
            }
            GeneralFeError {
                message, details, ..
            } => {
                let mut context = HashMap::new();
                if let Some(details) = details {
                    details.into_iter().for_each(|(key, value)| {
                        context.insert(key.clone(), i18n.try_translate_or_default(&value[0]));
                    });
                }
                i18n.try_translate_with_context_or_default(message, context)
            }
        }
    }
}

impl From<&CadetHubBeError> for CadetHubFeError {
    fn from(error: &CadetHubBeError) -> Self {
        match error {
            CadetHubBeError::AuthenticationError { .. } => GeneralFeError {
                code: CadetHubFeErrorCode::AuthenticationError,
                message: "error-authentication".to_string(),
                details: None,
                source_error: None,
            },

            CadetHubBeError::AuthorizationError { .. } => GeneralFeError {
                code: CadetHubFeErrorCode::AuthorizationError,
                message: "error-authorization".to_string(),
                details: None,
                source_error: None,
            },

            CadetHubBeError::ResourceNotFoundError { name, id } => {
                let mut details = HashMap::new();
                details.insert(
                    "resource_name".to_string(),
                    vec![name.clone().to_lowercase()],
                );
                details.insert("resource_id".to_string(), vec![id.clone().to_lowercase()]);
                GeneralFeError {
                    code: CadetHubFeErrorCode::ResourceNotFoundError,
                    message: "error-resource-not-found".to_string(),
                    details: Some(details),
                    source_error: None,
                }
            }

            CadetHubBeError::ResourceConflictError {
                name,
                unique_key_name,
                unique_key_value,
            } => {
                let mut details = HashMap::new();
                details.insert(
                    "resource_name".to_string(),
                    vec![name.clone().to_lowercase()],
                );
                details.insert("unique_key_name".to_string(), vec![unique_key_name.clone()]);
                details.insert(
                    "unique_key_value".to_string(),
                    vec![unique_key_value.clone()],
                );
                GeneralFeError {
                    code: CadetHubFeErrorCode::ResourceConflictError,
                    message: "error-resource-conflict".to_string(),
                    details: Some(details),
                    source_error: None,
                }
            }

            CadetHubBeError::ValidationError(errors) => {
                let mut details = HashMap::new();
                for (field_name, field_errors) in errors.field_errors() {
                    let mut errors = vec![];
                    for error in field_errors {
                        errors.push(error.code.to_string());
                    }
                    details.insert(field_name.to_string(), errors);
                }
                GeneralFeError {
                    code: CadetHubFeErrorCode::ValidationError,
                    message: "error-validation".to_string(),
                    details: Some(details),
                    source_error: None,
                }
            }
            _ => GeneralFeError {
                code: CadetHubFeErrorCode::CommonBeError,
                message: "error-internal".to_string(),
                details: None,
                source_error: None,
            },
        }
    }
}
