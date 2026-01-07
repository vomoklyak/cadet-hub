use crate::client::backend_client::BackendClient;
use crate::error::frontend_error::CadetHubFeError;
use crate::extension::i18n_extension::I18nExtension;
use crate::view::modal_view::Dialog;
use backend::context::BeApplicationContext;
use common::model::User;
use dioxus::prelude::*;
use dioxus_i18n::prelude::I18n;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct FeApplicationContext {
    i18n: Signal<I18n>,
    logged_in_user: Signal<Option<User>>,
    global_info: Signal<Option<String>>,
    global_dialog: Signal<Option<Dialog>>,
    global_error: Signal<Option<CadetHubFeError>>,
    global_spinner_visible: Signal<bool>,
    backend_client: Signal<Arc<BackendClient>>,
}

impl FeApplicationContext {
    pub(crate) fn init(backend: Arc<BeApplicationContext>, i18n: I18n) {
        use_context_provider(|| Self {
            i18n: Signal::new(i18n),
            backend_client: Signal::new(Arc::new(BackendClient::new(backend))),
            logged_in_user: Signal::new(None),
            global_info: Signal::new(None),
            global_dialog: Signal::new(None),
            global_error: Signal::new(None),
            global_spinner_visible: Signal::new(false),
        });
    }

    pub(crate) fn i18n() -> I18n {
        use_context::<FeApplicationContext>().i18n.read().clone()
    }

    pub(crate) fn translate<S: Into<String>>(key: S) -> String {
        let key = key.into();
        use_context::<FeApplicationContext>()
            .i18n
            .read()
            .try_translate_or_default(key)
    }

    pub(crate) fn translate_with_context<S, N, V>(key: S, context: HashMap<N, V>) -> String
    where
        S: Into<String>,
        N: Into<String>,
        V: Into<String>,
    {
        use_context::<FeApplicationContext>()
            .i18n
            .read()
            .try_translate_with_context_or_default(key, context)
    }

    pub(crate) fn logged_in_user() -> Option<User> {
        use_context::<FeApplicationContext>()
            .logged_in_user
            .read()
            .clone()
    }

    pub(crate) fn require_logged_in_user() -> User {
        use_context::<FeApplicationContext>()
            .logged_in_user
            .read()
            .clone()
            .expect("user not logged in")
    }

    pub(crate) fn log_in(user: User) {
        use_context::<FeApplicationContext>()
            .logged_in_user
            .set(Some(user));
    }

    pub(crate) fn log_out() {
        use_context::<FeApplicationContext>()
            .logged_in_user
            .set(None);
    }

    pub(crate) fn global_info() -> Signal<Option<String>> {
        use_context::<FeApplicationContext>().global_info
    }

    pub(crate) fn show_global_info<S: Into<String>>(info: S) {
        use_context::<FeApplicationContext>()
            .global_info
            .set(Some(info.into()));
    }

    pub(crate) fn clear_global_info() {
        use_context::<FeApplicationContext>().global_info.set(None);
    }

    pub(crate) fn global_dialog() -> Signal<Option<Dialog>> {
        use_context::<FeApplicationContext>().global_dialog
    }

    pub(crate) fn show_global_dialog(dialog: Dialog) {
        use_context::<FeApplicationContext>()
            .global_dialog
            .set(Some(dialog));
    }

    pub(crate) fn clear_global_dialog() {
        use_context::<FeApplicationContext>()
            .global_dialog
            .set(None);
    }

    pub(crate) fn global_error() -> Signal<Option<CadetHubFeError>> {
        use_context::<FeApplicationContext>().global_error
    }

    pub(crate) fn show_global_error(error: CadetHubFeError) {
        use_context::<FeApplicationContext>()
            .global_error
            .set(Some(error));
    }

    pub(crate) fn clear_global_error() {
        use_context::<FeApplicationContext>().global_error.set(None);
    }

    pub(crate) fn global_spinner_visible() -> Signal<bool> {
        use_context::<FeApplicationContext>().global_spinner_visible
    }

    pub(crate) fn show_global_spinner() {
        use_context::<FeApplicationContext>()
            .global_spinner_visible
            .set(true);
    }

    pub(crate) fn hide_global_spinner() {
        use_context::<FeApplicationContext>()
            .global_spinner_visible
            .set(false);
    }

    pub(crate) fn backend_client() -> Arc<BackendClient> {
        use_context::<FeApplicationContext>()
            .backend_client
            .read()
            .clone()
    }
}
