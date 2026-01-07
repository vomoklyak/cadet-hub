use crate::facade::cadet_facade::CadetFacade;
use crate::facade::impex_facade::ImpexFacade;
use crate::facade::user_facade::UserFacade;
use crate::repository::cadet_repository::DefaultCadetRepository;
use crate::repository::database::Database;
use crate::repository::user_repository::DefaultUserRepository;
use crate::service::auth_service::AuthService;
use crate::service::cadet_impex_service::ImpexService;
use crate::service::cadet_service::CadetService;
use crate::service::csv_service::CsvService;
use crate::service::user_service::UserService;
use common::config::ApplicationConfig;
use std::sync::Arc;
use crate::facade::admin_facade::AdminFacade;

pub struct BeApplicationContext {
    pub config: Arc<ApplicationConfig>,
    pub admin_facade: Arc<AdminFacade>,
    pub cadet_facade: Arc<CadetFacade>,
    pub impex_facade: Arc<ImpexFacade>,
    pub user_facade: Arc<UserFacade>,
}

impl BeApplicationContext {
    pub async fn init(config: Arc<ApplicationConfig>) -> Self {
        let database = Arc::new(Database::connect(&config).await.expect("failed to init DB"));
        let cadet_repository = Arc::new(DefaultCadetRepository::new(database.clone()));
        let user_repository = Arc::new(DefaultUserRepository::new(database.clone()));
        let auth_service = Arc::new(AuthService::new(user_repository.clone()));
        let cadet_service = Arc::new(CadetService::new(cadet_repository.clone()));
        let csv_service = Arc::new(CsvService::new());
        let impex_service = Arc::new(ImpexService::new(cadet_repository.clone()));
        let user_service = Arc::new(UserService::new(user_repository.clone()));
        let admin_facade = Arc::new(AdminFacade::new(
            config.clone(),
            auth_service.clone(),
        ));
        let cadet_facade = Arc::new(CadetFacade::new(
            auth_service.clone(),
            cadet_service.clone(),
        ));
        let impex_facade = Arc::new(ImpexFacade::new(
            csv_service,
            auth_service.clone(),
            cadet_service.clone(),
            impex_service.clone(),
        ));
        let user_facade = Arc::new(UserFacade::new(auth_service.clone(), user_service.clone()));

        Self {
            config,
            admin_facade,
            cadet_facade,
            impex_facade,
            user_facade,
        }
    }
}

impl PartialEq for BeApplicationContext {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.config, &other.config)
            && Arc::ptr_eq(&self.cadet_facade, &other.cadet_facade)
            && Arc::ptr_eq(&self.impex_facade, &other.impex_facade)
            && Arc::ptr_eq(&self.user_facade, &other.user_facade)
    }
}
