use crate::component::breadcrumb_bar::Breadcrumb;
use crate::context::application_context::FeApplicationContext;
use crate::router::router::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub(crate) fn HeaderLayout() -> Element {
    let logged_in_user = FeApplicationContext::require_logged_in_user();

    rsx! {
        nav {
            class: "sticky top-0 bg-zsu-green border-b border-zsu-green-light px-6 py-4 flex items-center justify-between",

            // Left side: Logo and Title
            Link {
                to: Route::Home {},
                class: "flex items-center gap-4 hover:opacity-80 transition-opacity cursor-pointer",
                svg { view_box: "0 0 100 100", class: "w-10 h-10 text-white",
                    path {
                        fill: "currentColor",
                        d: "M50 5L15 25V55L50 95L85 55V25L50 5Z",
                    }
                }
                div {
                    h2 { class: "font-bold text-lg leading-tight uppercase",
                        {FeApplicationContext::translate("system-name")}
                    }
                    p { class: "text-xs text-gray-400 uppercase",
                        {FeApplicationContext::translate("system-department-name")}
                    }
                }
            }

            Breadcrumb {
            }

            // Right side: User info and Logout
            div { class: "flex items-center gap-6",
                div { class: "text-right hidden sm:block",
                    p { class: "text-sm",
                        {
                            let current_user = FeApplicationContext::translate("system-user");
                            format!("{current_user}: {}", logged_in_user.login())
                        }
                    }
                    p { class: "text-sm",
                        {
                            let access_level = FeApplicationContext::translate("system-user-role-name");
                            let role = FeApplicationContext::translate_with_context(
                                    "role-name",
                                    HashMap::from([("name", logged_in_user.role().to_str().to_lowercase())]),
                                )
                                .to_lowercase();
                            format!("{access_level}: {role}")
                        }
                    }
                }

                button {
                    onclick: move |_| {
                        FeApplicationContext::log_out();
                        use_navigator().replace(Route::LoginView {});
                    },
                    class: "flex items-center gap-2 px-4 py-2 border border-zsu-red text-zsu-red-light hover:bg-zsu-red hover:text-white rounded transition-all text-sm font-medium uppercase",
                    {FeApplicationContext::translate("log-out")}
                }
            }
        }
        br {}
        div { class: "flex flex-col px-12", Outlet::<Route> {} }
    }
}