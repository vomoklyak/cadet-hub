use crate::error::CadetHubBeError;
use common::error::CadetHubError;
use sqlx::Error;

pub(super) fn handle(sqlx_error: Error) -> CadetHubBeError {
    match sqlx_error {
        Error::Database(db_error) => {
            let code = db_error.code();
            if code.as_deref() == Some("2067") || code.as_deref() == Some("1555") {
                return CadetHubBeError::default_resource_conflict_error();
            }
            CadetHubError::general_error_with_source(db_error).into()
        }
        _ => CadetHubError::general_error_with_source(sqlx_error).into(),
    }
}
