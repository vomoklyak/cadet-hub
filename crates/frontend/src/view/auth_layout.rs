use crate::context::application_context::FeApplicationContext;
use crate::router::router::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn AuthLayout() -> Element {
    if FeApplicationContext::logged_in_user().is_none() {
        use_navigator().replace(Route::LoginView {});
        return rsx! {};
    }
    rsx! {
        Outlet::<Route> {}
    }
}
