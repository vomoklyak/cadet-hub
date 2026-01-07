use crate::error::frontend_error::CadetHubFeError;
use backend::error::CadetHubBeError;
use common::logger::error;

pub(crate) fn handle_error(backend_error: CadetHubBeError) -> CadetHubFeError {
    error!("{:?}", backend_error.pretty_debug_str());
    CadetHubFeError::from(&backend_error)
}
