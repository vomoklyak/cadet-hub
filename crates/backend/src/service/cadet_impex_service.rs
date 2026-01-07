use crate::error::CadetHubBeError;
use crate::repository::cadet_repository::CadetRepository;
use crate::CadetHubBeResult;
use common::model::ImpexCadetCourseEntry;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct ImpexService {
    cadet_repository: Arc<dyn CadetRepository + Send + Sync>,
}

impl ImpexService {
    pub(crate) fn new(cadet_repository: Arc<dyn CadetRepository + Send + Sync>) -> Self {
        Self { cadet_repository }
    }

    pub(crate) async fn import_cadet_courses(
        &self,
        entries: Vec<ImpexCadetCourseEntry>,
    ) -> CadetHubBeResult<Vec<(ImpexCadetCourseEntry, CadetHubBeError)>> {
        self.cadet_repository
            .save_cadet_impex_entries(entries)
            .await
    }
}
