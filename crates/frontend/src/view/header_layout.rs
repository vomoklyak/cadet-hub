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
            // Added 'relative' to allow absolute positioning of the spinner
            class: "sticky top-0 z-50 bg-zsu-green border-b border-zsu-green-light px-6 py-4 flex items-center justify-between",

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

            // --- CENTRAL PRELOADER ---
            if FeApplicationContext::global_spinner_visible()() {
                div { class: "absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2",
                    LoadingSpinner {}
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
                    class: "flex items-center gap-2 px-4 py-2 border border-red-500/50 text-red-400 hover:bg-red-500 hover:text-white rounded transition-all text-sm font-medium uppercase",
                    {FeApplicationContext::translate("log-out")}
                }
            }
        }
        br {}
        div { class: "flex flex-col px-12", Outlet::<Route> {} }
    }
}

#[component]
fn LoadingSpinner() -> Element {
    rsx! {
        div { class: "relative flex items-center justify-center w-12 h-12",
            // Outer Glowing Ring (Pulsing)
            div { class: "absolute inset-0 rounded-full border-4 border-zsu-accent-gold/20 animate-ping" }

            // Inner Heavy Track
            div { class: "absolute inset-0 rounded-full border-[3px] border-white/5" }

            // The Primary Solid Spinner
            svg {
                class: "w-10 h-10 animate-spin text-zsu-accent-gold",
                view_box: "0 0 24 24",
                fill: "none",
                // Shadow filter for a "glow" effect
                filter: "drop-shadow(0 0 3px rgba(212, 175, 55, 0.5))",

                circle {
                    class: "opacity-25",
                    cx: "12",
                    cy: "12",
                    r: "10",
                    stroke: "currentColor",
                    stroke_width: "4",
                }
                path {
                    class: "opacity-100",
                    fill: "currentColor",
                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                }
            }
        }
    }
}
