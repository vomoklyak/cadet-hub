use crate::context::application_context::FeApplicationContext;
use crate::element::button::RegularButton;
use crate::error::frontend_error::CadetHubFeError;
use crate::symbol::{CANCEL, OK};
use common::cadet_hub_common_prelude::*;
use dioxus::prelude::*;

#[component]
fn ModalView(z_index: String, content: Element) -> Element {
    rsx! {
        div { class: "{z_index} fixed inset-0 flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm animate-in fade-in duration-300",
            div { class: "p-6 w-full max-w-md bg-zsu-green-dark-light border border-zsu-green-light border-2 rounded-lg overflow-hidden animate-in zoom-in-95 slide-in-from-bottom-4 duration-300",
                div { {content} }
            }
        }
    }
}

#[component]
pub(crate) fn InputModalView(content: Element) -> Element {
    rsx! {
        ModalView {
            // below any output modal view
            z_index: "z-[80]",
            content
        }
    }
}

// on top
#[component]
pub(crate) fn OutputModalView(
    #[props(into)] title: String,
    #[props(into, default)] critical: bool,
    #[props(into)] message: Option<String>,
    #[props(default)] buttons: Vec<Element>,
) -> Element {
    let Some(message) = message else {
        return rsx! {};
    };
    let title_text_color = if critical {
        "text-red-400"
    } else {
        "text-grey-light"
    };

    rsx! {
        ModalView {
            z_index: "z-[90]",
            content: rsx! {
                div {
                    onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                    },



                    div { class: "bg-zsu-green-dark-light border-b border-zsu-green-light p-4 flex items-center gap-3 {title_text_color}",
                        span { class: "animate-pulse",
                            svg {
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "1.5",
                                stroke: "currentColor",
                                class: "w-6 h-6",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z",
                                }
                            }
                        }
                        h2 { class: "text-sm font-bold uppercase", {title} }
                    }

                    div { class: "p-6",
                        p { class: "text-grey-light text-sm leading-relaxed font-medium text-justify", {message} }
                    }

                    div { class: "p-2 flex justify-evenly", {buttons.into_iter()} }
                }
            },
        }
    }
}

#[component]
pub(crate) fn InfoModalView(global_info_signal: Signal<Option<String>>) -> Element {
    let Some(info) = global_info_signal() else {
        return rsx! {};
    };

    #[rustfmt::skip]
    rsx! {
        OutputModalView {
            title: FeApplicationContext::translate("warning"),
            message: info,
            buttons: vec![
                rsx! {
                    RegularButton {
                        name: "clear_global_error",
                        title: FeApplicationContext::translate("ok"),
                        symbol: OK,
                        onclick: move |_| {
                            FeApplicationContext::clear_global_info();
                        },
                    }
                },
            ],
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Getters, Setters)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub(crate) struct Dialog {
    message: String,
    on_approve: EventHandler<MouseEvent>,
}

impl Dialog {
    pub fn new(message: String, on_approve: EventHandler<MouseEvent>) -> Self {
        Self {
            message,
            on_approve,
        }
    }
}

#[component]
pub(crate) fn DialogModalView(dialog_message_signal: Signal<Option<Dialog>>) -> Element {
    let Some(dialog) = dialog_message_signal() else {
        return rsx! {};
    };

    #[rustfmt::skip]
    rsx! {
        OutputModalView {
            title: FeApplicationContext::translate("warning"),
            message: FeApplicationContext::translate(dialog.message()),
            buttons: vec![
                rsx! {
                    RegularButton {
                        name : "cancel_button",
                        title : FeApplicationContext::translate("cancel"),
                        symbol : CANCEL,
                        onclick : move |_| {
                            FeApplicationContext::clear_global_dialog(); },
                    }
                    RegularButton {
                        name: "accept_button",
                        title: FeApplicationContext::translate("continue"),
                        symbol: OK,
                        onclick : move | event | {
                            dialog.on_approve.call(event);
                            FeApplicationContext::clear_global_dialog();
                        },
                    }
                },
            ],
        }
    }
}

#[component]
pub(crate) fn ErrorModalView(global_error_signal: Signal<Option<CadetHubFeError>>) -> Element {
    let Some(error) = global_error_signal() else {
        return rsx! {};
    };
    if let Some(source_error) = error.source_error() {
        error!("{source_error}");
    }

    #[rustfmt::skip]
    rsx! {
        OutputModalView {
            title: format!("{}: {}", FeApplicationContext::translate("warning"), FeApplicationContext::translate("error")),
            critical: true,
            message: error.localized_message(&FeApplicationContext::i18n()),
            buttons: vec![
                rsx! {
                    RegularButton {
                        name : "clear_global_error",
                        title : FeApplicationContext::translate("ok"),
                        symbol : OK,
                        onclick : move |_| {
                            FeApplicationContext::clear_global_error();
                        },
                    }
                },
            ],
        }
    }
}
