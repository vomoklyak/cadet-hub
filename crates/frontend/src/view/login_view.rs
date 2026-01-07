use crate::context::application_context::FeApplicationContext;
use crate::element::button::SubmitButton;
use crate::element::form_data_extension::FormDataExtension;
use crate::element::input_field::{PasswordInputField, TextInputField};
use crate::router::router::Route;
use dioxus::prelude::*;

#[component]
pub fn LoginView() -> Element {
    let mut error_visible = use_signal(|| false);
    let on_submit = move |event: FormEvent| {
        spawn(async move {
            event.prevent_default();
            event.stop_propagation();

            let login = event.get_str("login").expect("login not set");
            let password = event.get_str("password").expect("password not set");
            let login_result = FeApplicationContext::backend_client()
                .login(login.as_str(), password.as_str())
                .await;
            match login_result {
                Ok(user) => {
                    FeApplicationContext::log_in(user);
                    FeApplicationContext::clear_global_error();
                    use_navigator().replace(Route::Home {});
                }
                Err(error) => {
                    error!("failed to login: {error:?}");
                    error_visible.set(true);
                }
            }
        });
    };

    rsx! {
        div { class: "pixel-pattern min-h-screen flex items-center justify-center px-4 bg-zsu-green-dark text-grey-light",
            div { class: "w-full max-w-md p-8 bg-zsu-green rounded-lg border border-zsu-green-light shadow-2xl",
                // Header / Logo
                div { class: "flex flex-col items-center mb-8",
                    div { class: "w-20 h-20 bg-white rounded-full flex items-center justify-center mb-4 shadow-inner",
                        svg {
                            view_box: "0 0 100 100",
                            class: "w-14 h-14 text-zsu-green",
                            path {
                                fill: "currentColor",
                                d: "M50 5L15 25V55L50 95L85 55V25L50 5ZM50 15L75 30V50L50 80L25 50V30L50 15Z",
                            }
                        }
                    }
                    h1 { class: "text-2xl font-bold tracking-widest text-white uppercase",
                        {FeApplicationContext::translate("system-name")}
                    }
                    p { class: "text-gray-400 text-sm mt-1 uppercase",
                        {FeApplicationContext::translate("system-department-name")}
                    }
                }

                form { onsubmit: on_submit, class: "space-y-6",
                    TextInputField {
                        name: "login",
                        placeholder: FeApplicationContext::translate("enter-login"),
                        title: FeApplicationContext::translate("login"),
                        required: true,
                    }
                    PasswordInputField {
                        name: "password",
                        placeholder: FeApplicationContext::translate("enter-password"),
                        title: FeApplicationContext::translate("password"),
                        required: true,
                    }
                    if error_visible() {
                        div { class: "text-red-400 text-sm text-center bg-red-900/20 py-2 rounded border border-red-900/50",
                            {FeApplicationContext::translate("error-authentication")}
                        }
                    }
                    SubmitButton {
                        name: "submit_login",
                        title: FeApplicationContext::translate("log-in"),
                    }
                }

                div { class: "mt-8 text-center",
                    span { class: "text-xs text-gray-500 uppercase tracking-tighter",
                        {FeApplicationContext::translate("system-restriction")}
                    }
                }
            }
        }
    }
}
