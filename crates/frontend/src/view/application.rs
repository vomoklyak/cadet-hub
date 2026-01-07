use crate::context::application_context::FeApplicationContext;
use crate::router::router::Route;
use crate::view::modal_view::{DialogModalView, ErrorModalView, InfoModalView};
use backend::context::BeApplicationContext;
use dioxus::prelude::*;
use dioxus_i18n::prelude::{use_init_i18n, I18nConfig};
use dioxus_i18n::unic_langid::langid;
use std::sync::Arc;

#[component]
pub(crate) fn Application() -> Element {
    let i18n = use_init_i18n(|| {
        I18nConfig::new(langid!("uk-UA")) // Default language
            .with_locale((
                langid!("uk-UA"),
                include_str!("../../assets/locales/uk-UA.ftl"),
            ))
    });
    info!("I18n initiated");

    FeApplicationContext::init(use_context::<Arc<BeApplicationContext>>(), i18n);
    info!("Frontend application context initiated");

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("assets/main.css") }
        InfoModalView { global_info_signal: FeApplicationContext::global_info() }
        DialogModalView { dialog_message_signal: FeApplicationContext::global_dialog() }
        ErrorModalView { global_error_signal: FeApplicationContext::global_error() }
        Router::<Route> {}
    }
}
