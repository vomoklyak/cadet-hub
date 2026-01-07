use crate::component::action_drawer;
use crate::context::application_context::FeApplicationContext;
use crate::element::button::RegularButton;
use crate::router::router::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "pixel-pattern min-h-screen flex flex-col bg-zsu-green-dark text-grey-light",
            if FeApplicationContext::require_logged_in_user().has_administrate_permission() {
                ActionDrawer {}
            }

            main { class: "flex-1 p-6 md:p-12 max-w-7xl mx-auto w-full",
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    DashboardTile {
                        title: FeApplicationContext::translate("cadets"),
                        description: FeApplicationContext::translate("cadet-management"),
                        accent_color: "emerald",
                        label: format!("{} {}", FeApplicationContext::translate("open"), "→"),
                        onclick: move |_| {
                            use_navigator().replace(Route::CadetListView {});
                        },
                    }

                    DashboardTile {
                        title: FeApplicationContext::translate("cadet-courses"),
                        description: FeApplicationContext::translate("cadet-course-management"),
                        accent_color: "blue",
                        label: format!("{} {}", FeApplicationContext::translate("open"), "→"),
                        onclick: move |_| {
                            use_navigator().replace(Route::CadetCourseListView {});
                        },
                    }

                    DashboardTile {
                        title: FeApplicationContext::translate("users"),
                        description: FeApplicationContext::translate("user-management"),
                        accent_color: "purple",
                        label: format!("{} {}", FeApplicationContext::translate("open"), "→"),
                        onclick: move |_| {
                            use_navigator().replace(Route::UserListView {});
                        },
                    }
                }

                footer { class: "mt-12 pt-6 border-t border-zsu-green-light flex flex-wrap gap-4 justify-between items-center text-xs text-gray-500 uppercase tracking-widest",
                    div { class: "flex items-center gap-2",
                        span { class: "w-2 h-2 rounded-full bg-green-500 animate-pulse" }
                        {FeApplicationContext::translate("system-status")}
                    }
                    div { {FeApplicationContext::translate("system-version")} }
                    div { {FeApplicationContext::translate("system-copyright")} }
                }
            }
        }
    }
}

#[rustfmt::skip]
#[component]
pub(crate) fn ActionDrawer() -> Element {
    rsx! {
        action_drawer::ActionDrawer {
            extra_buttons: vec![
                rsx! {
                    RegularButton {
                        name: "show_encryption_key_button",
                        title: FeApplicationContext::translate("show"),
                        symbol: "🔑",
                        onclick : move |event : Event < MouseData >| async move {
                            event.prevent_default();
                            event.stop_propagation();

                            let actor_user =
                                FeApplicationContext::require_logged_in_user();
                            match FeApplicationContext::backend_client().get_encryption_key(actor_user).await {
                                Ok(encryption_key) => {
                                    let info =
                                    FeApplicationContext::translate_with_context(
                                        "info-show-encryption-key",
                                        HashMap::from([("encryption-key", encryption_key)])
                                    );
                                    FeApplicationContext::show_global_info(info);
                                }
                                Err(error) => {
                                    FeApplicationContext::show_global_error(error);
                                }
                            };
                        }
                    }
                }
            ]
        }
    }
}

#[component]
fn DashboardTile(
    title: String,
    description: String,
    accent_color: String,
    label: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let color_class = match accent_color.as_str() {
        "blue" => "bg-blue-500/10 text-blue-400 group-hover:bg-blue-500",
        "emerald" => "bg-emerald-500/10 text-emerald-400 group-hover:bg-emerald-500",
        "purple" => "bg-purple-500/10 text-purple-400 group-hover:bg-purple-500",
        _ => "bg-gray-500/10 text-gray-400",
    };

    rsx! {
        div {
            class: "group bg-zsu-green-dark-light border p-8 rounded-lg hover:-translate-y-1 hover:border-zsu-accent-gold transition-all cursor-pointer flex flex-col items-center text-center shadow-xl",
            onclick: move |event| {
                onclick.call(event);
            },
            div { class: "w-16 h-16 rounded-full flex items-center justify-center mb-6 group-hover:text-white transition-colors {color_class}",
                div { class: "w-8 h-8 border-2 border-current rounded" }
            }
            h3 { class: "text-xl font-bold mb-2", "{title}" }
            p { class: "text-gray-400 text-sm", "{description}" }
            div { class: "mt-6 text-xs font-bold uppercase text-yellow-600 tracking-widest group-hover:text-yellow-400",
                "{label}"
            }
        }
    }
}
