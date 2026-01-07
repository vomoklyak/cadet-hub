use crate::component::action_drawer;
use crate::context::application_context::FeApplicationContext;
use crate::element::button::{DeleteButton, RegularButton};
use crate::util::frontend_util::display_if;
use crate::view::user_edit_view::UserEditView;
use common::model::User;
use common::model::{PageRequest, SearchUserRequestBuilder};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub(crate) fn UserListView() -> Element {
    let mut user_edit_id_signal = use_signal(|| None);
    let mut user_edit_view_visible_signal = use_signal(|| false);
    let mut user_list_resource = use_resource(move || async move {
        let user = FeApplicationContext::require_logged_in_user();
        let request = SearchUserRequestBuilder::default()
            .page_request(PageRequest::all())
            .build()
            .expect("failed build UserSearch");
        FeApplicationContext::backend_client()
            .get_users_by_search_request(user, request)
            .await
            .unwrap_or_else(|error| {
                FeApplicationContext::show_global_error(error);
                vec![]
            })
    });
    let has_administrate_permission =
        FeApplicationContext::require_logged_in_user().has_administrate_permission();

    let on_select_user = move |user_id: Option<i64>| {
        user_edit_id_signal.set(user_id);
        user_edit_view_visible_signal.set(true);
    };

    let on_delete_user = move |user_id: i64| async move {
        let actor_user = FeApplicationContext::require_logged_in_user();
        match FeApplicationContext::backend_client()
            .delete_user(actor_user, user_id)
            .await
        {
            Ok(_) => {
                user_list_resource.restart();
            }
            Err(error) => {
                FeApplicationContext::show_global_error(error);
            }
        };
    };

    let on_edit_cancel_user = move |restart_required: bool| {
        if restart_required {
            user_list_resource.restart();
        }
        user_edit_view_visible_signal.set(false);
    };

    rsx! {
        div { style: display_if(!user_edit_view_visible_signal()),
            if has_administrate_permission {
                ActionDrawer {
                    user_edit_id_signal,
                    user_edit_view_visible_signal,
                }
            }

            div { class: "bg-zsu-green-dark-light border border-zsu-green-light rounded-lg overflow-hidden shadow-2xl",
                table { class: "zsu-table",
                    thead { class: "zsu-table-thead",
                        tr {
                            th { class: "zsu-table-th", {FeApplicationContext::translate("login")} }
                            th { class: "zsu-table-th", {FeApplicationContext::translate("role")} }
                            th { class: "zsu-table-th", {FeApplicationContext::translate("actions")} }
                        }
                    }
                    tbody {
                        for user in user_list_resource.suspend()?.read().iter().cloned() {
                            UserView {
                                user,
                                on_select_user,
                                on_delete_user,
                            }
                        }
                    }
                }
            }
        }

        div { style: display_if(user_edit_view_visible_signal()),
            UserEditView { user_edit_id_signal, on_edit_cancel_user }
        }
    }
}

#[rustfmt::skip]
#[component]
pub(crate) fn ActionDrawer(
    user_edit_id_signal: Signal<Option<i64>>,
    user_edit_view_visible_signal: Signal<bool>,
) -> Element {
    rsx! {
        action_drawer::ActionDrawer {
            extra_buttons: vec![
                rsx! {
                    RegularButton {
                        name: "create_button",
                        title: FeApplicationContext::translate("create"),
                        symbol: "+",
                        onclick : move |event : Event < MouseData >| {
                            event.prevent_default();
                            event.stop_propagation();

                            user_edit_id_signal.set(None);
                            user_edit_view_visible_signal.set(true);
                        }
                    }
                }
            ]
        }
    }
}

#[component]
pub(crate) fn UserView(
    user: User,
    on_select_user: EventHandler<Option<i64>>,
    on_delete_user: EventHandler<i64>,
) -> Element {
    let edit_user_id = user.require_id();
    let delete_user_id = user.require_id();
    let has_administrate_permission =
        FeApplicationContext::require_logged_in_user().has_administrate_permission();

    rsx! {
        tr {
            class: "zsu-table-tr group hover:bg-zsu-accent-gold-dark/20",
            onclick: move |event: Event<MouseData>| {
                event.prevent_default();
                event.stop_propagation();
                on_select_user.call(Some(edit_user_id));
            },
            td { class: "zsu-table-td", {format!("{}", user.login())} }
            td { class: "zsu-table-td",
                {
                    FeApplicationContext::translate_with_context(
                        "role-name",
                        HashMap::from([("name", user.role().to_str().to_lowercase())]),
                    )
                }
            }
            td { class: "px-6 py-4 text-right",
                div { class: "flex justify-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity",
                    if has_administrate_permission && !user.root_admin() {
                        DeleteButton {
                            name: "delete_user",
                            onclick: move |event: Event<MouseData>| {
                                async move {
                                    event.prevent_default();
                                    event.stop_propagation();

                                    on_delete_user.call(delete_user_id);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
