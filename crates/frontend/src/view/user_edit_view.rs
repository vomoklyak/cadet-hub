use crate::context::application_context::FeApplicationContext;
use crate::element::button::RegularButton;
use crate::element::form_data_extension::FormDataExtension;
use crate::element::input_field::{PasswordInputField, SelectInputField, TextInputField};
use common::model::UpdateUserRequestBuilder;
use common::model::{User, UserBuilder, UserRole};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

#[component]
pub(crate) fn UserEditView(
    user_edit_id_signal: Signal<Option<i64>>,
    on_edit_cancel_user: EventHandler<bool>,
) -> Element {
    let user_resource = use_resource(move || async move {
        let user = FeApplicationContext::require_logged_in_user();
        match user_edit_id_signal() {
            Some(id) => FeApplicationContext::backend_client()
                .get_user(user, id)
                .await
                .unwrap_or_else(|error| {
                    FeApplicationContext::show_global_error(error);
                    User::default()
                }),
            None => User::default(),
        }
    });
    let user = user_resource.suspend()?.read().cloned();

    let new_user = user_edit_id_signal().is_none();

    let self_update = user_resource
        .suspend()?
        .read()
        .id()
        .map(|id| FeApplicationContext::require_logged_in_user().has_id(id))
        .unwrap_or(false);

    let has_administrate_permission =
        FeApplicationContext::require_logged_in_user().has_administrate_permission();

    let on_submit = move |event: FormEvent| async move {
        event.prevent_default();
        event.stop_propagation();

        let actor_user = FeApplicationContext::require_logged_in_user();
        let result = if let Some(user_id) = user_edit_id_signal() {
            let password = event
                .get_str("password")
                .filter(|password| !password.is_empty());
            let role = event
                .get_str("role")
                .map(|role| UserRole::from_str(&role).unwrap_or_default());
            let user = UpdateUserRequestBuilder::default()
                .id(user_id)
                .password(password)
                .role(role)
                .build()
                .expect("failed build User");
            FeApplicationContext::backend_client()
                .update_user(actor_user, user)
                .await
        } else {
            let login = event.get_str("login").expect("login not set");
            let password = event.get_str("password").expect("password not set");
            let role = event.get_str("role").expect("role not set");
            let user = UserBuilder::default()
                .login(login)
                .password(password)
                .role(UserRole::from_str(&role).unwrap_or_default())
                .build()
                .expect("failed build User");
            FeApplicationContext::backend_client()
                .create_user(actor_user, user)
                .await
        };
        match result {
            Ok(_) => on_edit_cancel_user.call(true),
            Err(error) => {
                error!("failed to create user: {error:?}");
                FeApplicationContext::show_global_error(error);
            }
        }
    };

    rsx! {
        form { onsubmit: on_submit,
            div { class: "zsu-input-grid",
                TextInputField {
                    name: "login",
                    title: FeApplicationContext::translate("login"),
                    placeholder: FeApplicationContext::translate("enter-login"),
                    required: true,
                    disabled: !new_user,
                    value: user.login(),
                }

                if has_administrate_permission || self_update {
                    PasswordInputField {
                        name: "password",
                        title: FeApplicationContext::translate("password"),
                        placeholder: FeApplicationContext::translate("enter-password"),
                        required: new_user,
                    }
                }

                SelectInputField {
                    name: "role",
                    title: FeApplicationContext::translate("role"),
                    disabled: !has_administrate_permission || self_update,
                    selected: user.role().to_str(),
                    required: true,
                    items: roles_with_localized_names(),
                }
            }
            div { class: "zsu-button-cell",
                RegularButton {
                    name: "cancel_button",
                    title: FeApplicationContext::translate("back"),
                    symbol: "←",
                    onclick: move |event: Event<MouseData>| {
                        event.prevent_default();
                        event.stop_propagation();

                        on_edit_cancel_user.call(false);
                    },
                }

                if has_administrate_permission || self_update {
                    RegularButton {
                        r#type: "submit",
                        name: "save_button",
                        title: FeApplicationContext::translate("save"),
                        symbol: "✓",
                        onclick: move |_| {},
                    }
                }
            }
        }
    }
}

fn roles_with_localized_names() -> Vec<(String, String)> {
    UserRole::names()
        .into_iter()
        .map(|role_name| {
            let localized_role_name = FeApplicationContext::translate_with_context(
                "role-name",
                HashMap::from([("name", role_name.to_lowercase())]),
            );
            (role_name, localized_role_name)
        })
        .collect::<Vec<_>>()
}
