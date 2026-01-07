use crate::context::application_context::FeApplicationContext;
use crate::router::router::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Breadcrumb() -> Element {
    let route = use_route::<Route>();

    let trail = match route {
        Route::Home {} => vec![(
            FeApplicationContext::translate("breadcrumb-home"),
            Route::Home {},
        )],
        Route::CadetCourseListView {} => vec![
            (
                FeApplicationContext::translate("breadcrumb-home"),
                Route::Home {},
            ),
            (
                FeApplicationContext::translate("breadcrumb-cadet-courses"),
                Route::CadetCourseListView {},
            ),
        ],
        Route::CadetListView {} => vec![
            (
                FeApplicationContext::translate("breadcrumb-home"),
                Route::Home {},
            ),
            (
                FeApplicationContext::translate("breadcrumb-cadets"),
                Route::CadetListView {},
            ),
        ],
        Route::UserListView {} => vec![
            (
                FeApplicationContext::translate("breadcrumb-home"),
                Route::Home {},
            ),
            (
                FeApplicationContext::translate("breadcrumb-users"),
                Route::UserListView {},
            ),
        ],
        Route::LoginView {} => vec![(
            FeApplicationContext::translate("breadcrumb-login"),
            Route::LoginView {},
        )],
    };

    let trail_len = trail.len();

    rsx! {
        nav { class: "p-4",
            ol { class: "flex list-none p-0 gap-2 items-center",
                {
                    trail
                        .into_iter()
                        .enumerate()
                        .map(|(index, (label, route))| {
                            let last = index == trail_len - 1;
                            rsx! {
                                li { key: "{label}", class: "flex items-center",
                                    if index > 0 {
                                        span { class: "mr-2", "/" }
                                    }
                                    if last {
                                        span { class: "font-bold", "{label}" }
                                    } else {
                                        Link { to: route, "{label}" }
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
