use crate::error::backend_error_handler;
use crate::error::frontend_error::CadetHubFeError;
use crate::CadetHubFeResult;
use backend::context::BeApplicationContext;
use backend::CadetHubBeResult;
use common::cadet_hub_common_prelude::Serialize;
use common::error::CadetHubError;
use common::model::{Cadet, CadetCourse, ImpexCadetCourseEntry};
use common::model::{
    ExportCadetCourseResponse, ImportCadetCourseRequest, ImportCadetCourseResponse,
    SearchCadetCourseRequest, SearchCadetCourseResponse, SearchCadetRequest, SearchCadetResponse,
    SearchUserRequest, UpdateUserRequest,
};
use common::model::{SearchCadetCourseStatisticResponse, User};
use std::future::Future;
use std::path::Path;
use std::sync::Arc;

pub(crate) struct BackendClient {
    backend: Arc<BeApplicationContext>,
}

impl BackendClient {
    pub(crate) fn new(backend: Arc<BeApplicationContext>) -> Self {
        Self { backend }
    }

    // ADMIN
    pub async fn get_encryption_key(&self, actor_user: User) -> CadetHubFeResult<String> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .admin_facade
                .get_admin_encryption_key(actor_user)
                .await
        })
        .await
    }

    // USER
    pub async fn create_user(&self, actor_user: User, user: User) -> CadetHubFeResult<User> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend.user_facade.create_user(actor_user, user).await
        })
        .await
    }

    pub async fn update_user(
        &self,
        actor_user: User,
        request: UpdateUserRequest,
    ) -> CadetHubFeResult<User> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend.user_facade.update_user(actor_user, request).await
        })
        .await
    }

    pub async fn delete_user(&self, actor_user: User, user_id: i64) -> CadetHubFeResult<()> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend.user_facade.delete_user(actor_user, user_id).await
        })
        .await
    }

    pub(crate) async fn get_users_by_search_request(
        &self,
        actor_user: User,
        request: SearchUserRequest,
    ) -> CadetHubFeResult<Vec<User>> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .user_facade
                .get_users_by_search_request(actor_user, request)
                .await
        })
        .await
    }

    pub(crate) async fn get_user(&self, actor_user: User, id: i64) -> CadetHubFeResult<User> {
        let backend = self.backend.clone();
        execute_backend_task(
            move || async move { backend.user_facade.get_user(actor_user, id).await },
        )
        .await
    }

    pub async fn login(&self, login: &str, password: &str) -> CadetHubFeResult<User> {
        let backend = self.backend.clone();
        let login = login.to_string();
        let password = password.to_string();
        execute_backend_task(
            move || async move { backend.user_facade.login(&login, &password).await },
        )
        .await
    }

    // IMPEX
    pub async fn read_csv_file(&self, path: &Path) -> CadetHubFeResult<Vec<ImpexCadetCourseEntry>> {
        let backend = self.backend.clone();
        let path = path.to_path_buf();
        execute_backend_task(move || async move { backend.impex_facade.read_csv_file(&path).await })
            .await
    }

    pub async fn write_to_csv_string<T: Serialize + Send + 'static>(
        &self,
        entities: Vec<T>,
    ) -> CadetHubFeResult<String> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend.impex_facade.write_to_csv_string(entities).await
        })
        .await
    }

    pub async fn import_cadet_courses(
        &self,
        actor_user: User,
        request: ImportCadetCourseRequest,
    ) -> CadetHubFeResult<ImportCadetCourseResponse<CadetHubFeError>> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            let response = backend
                .impex_facade
                .import_cadet_courses(actor_user, request)
                .await?;
            let failed_entries = response
                .owned_failed_entries()
                .into_iter()
                .map(|(entry, error)| (entry.clone(), CadetHubFeError::from(&error)))
                .collect::<Vec<_>>();
            Ok(ImportCadetCourseResponse::new(failed_entries))
        })
        .await
    }

    pub async fn export_cadet_courses(
        &self,
        actor_user: User,
        request: SearchCadetCourseRequest,
    ) -> CadetHubFeResult<ExportCadetCourseResponse> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .impex_facade
                .export_cadet_courses(actor_user, request)
                .await
        })
        .await
    }

    // CADET
    pub async fn create_cadet(&self, actor_user: User, cadet: Cadet) -> CadetHubFeResult<Cadet> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend.cadet_facade.create_cadet(actor_user, cadet).await
        })
        .await
    }

    pub async fn update_cadet(&self, actor_user: User, cadet: Cadet) -> CadetHubFeResult<Cadet> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend.cadet_facade.update_cadet(actor_user, cadet).await
        })
        .await
    }

    pub async fn delete_cadet(&self, actor_user: User, cadet_id: i64) -> CadetHubFeResult<()> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .cadet_facade
                .delete_cadet(actor_user, cadet_id)
                .await
        })
        .await
    }

    pub async fn get_cadet(&self, actor_user: User, cadet_id: i64) -> CadetHubFeResult<Cadet> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend.cadet_facade.get_cadet(actor_user, cadet_id).await
        })
        .await
    }

    pub async fn get_cadets_by_search_request(
        &self,
        actor_user: User,
        request: SearchCadetRequest,
    ) -> CadetHubFeResult<SearchCadetResponse> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .cadet_facade
                .get_cadet_by_search_request(actor_user, request)
                .await
        })
        .await
    }

    pub async fn create_cadet_course(
        &self,
        actor_user: User,
        cadet_course: CadetCourse,
    ) -> CadetHubFeResult<CadetCourse> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .cadet_facade
                .create_cadet_course(actor_user, cadet_course)
                .await
        })
        .await
    }

    pub async fn update_cadet_course(
        &self,
        actor_user: User,
        cadet_course: CadetCourse,
    ) -> CadetHubFeResult<CadetCourse> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .cadet_facade
                .update_cadet_course(actor_user, cadet_course)
                .await
        })
        .await
    }

    pub async fn delete_cadet_course(
        &self,
        actor_user: User,
        cadet_course_id: i64,
    ) -> CadetHubFeResult<()> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .cadet_facade
                .delete_cadet_course(actor_user, cadet_course_id)
                .await
        })
        .await
    }

    pub async fn get_cadet_course(
        &self,
        actor_user: User,
        cadet_course_id: i64,
    ) -> CadetHubFeResult<CadetCourse> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .cadet_facade
                .get_cadet_course(actor_user, cadet_course_id)
                .await
        })
        .await
    }

    pub async fn get_cadet_course_entries_by_search_request(
        &self,
        actor_user: User,
        request: SearchCadetCourseRequest,
    ) -> CadetHubFeResult<SearchCadetCourseResponse> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .cadet_facade
                .get_cadet_course_entries_by_search_request(actor_user, request)
                .await
        })
        .await
    }

    pub async fn get_cadet_course_statistic_entries_by_search_request(
        &self,
        actor_user: User,
        request: SearchCadetCourseRequest,
    ) -> CadetHubFeResult<SearchCadetCourseStatisticResponse> {
        let backend = self.backend.clone();
        execute_backend_task(move || async move {
            backend
                .cadet_facade
                .get_cadet_course_statistic_entries_by_search_request(actor_user, request)
                .await
        })
        .await
    }
}

async fn execute_backend_task<F, Fut, T>(task: F) -> CadetHubFeResult<T>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = CadetHubBeResult<T>> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(task())
        .await
        .map_err(|error| CadetHubError::general_error_with_source(error).into())
        .flatten()
        .map_err(|error| backend_error_handler::handle_error(error))
}