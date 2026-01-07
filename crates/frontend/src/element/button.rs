use crate::context::application_context::FeApplicationContext;
use crate::view::modal_view::Dialog;
use dioxus::prelude::*;

#[component]
pub(crate) fn SubmitButton(
    #[props(into)] name: String,
    #[props(into)] title: String,
    #[props(optional)] disabled: bool,
    #[props(default = "submit".to_string(), into)] r#type: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        button {
            r#type,
            class: "w-full py-3 bg-zsu-accent-gold hover:bg-zsu-accent-gold-dark text-zsu-green-dark font-bold rounded uppercase tracking-wider transition-colors",
            name,
            disabled,
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },
            "{title}"
        }
    }
}

#[component]
pub(crate) fn EditButton(
    #[props(into)] name: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "p-1.5 hover:text-blue-400 transition-colors",
            name,
            title: "Редагувати",
            onclick: move |event| {
                onclick.call(event);
            },
            svg {
                class: "w-4 h-4",
                fill: "none",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z",
                }
            }
        }
    }
}

#[component]
pub(crate) fn DeleteButton(
    #[props(into)] name: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "p-1.5 hover:text-red-500 transition-colors",
            title: "Видалити",
            onclick: move |event| {
                event.prevent_default();
                event.stop_propagation();

                let dialog = Dialog::new(
                    FeApplicationContext::translate("dialog-delete-confirmation"),
                    EventHandler::new(move |event| {
                        onclick.call(event);
                    }),
                );
                FeApplicationContext::show_global_dialog(dialog);
            },
            svg {
                class: "w-4 h-4",
                fill: "none",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16",
                }
            }
        }
    }
}

#[component]
pub(crate) fn GhostButton(
    #[props(into)] name: String,
    #[props(into)] title: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "text-xs uppercase text-grey-light hover:text-zsu-accent-gold transition-colors",
            onclick: move |event| {
                onclick.call(event);
            },
            {title}
        }
    }
}

#[component]
pub(crate) fn RegularButton(
    #[props(default = "button".to_string(), into)] r#type: String,
    #[props(into)] name: String,
    #[props(into)] title: Option<String>,
    #[props(into)] symbol: Option<String>,
    #[props(into)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        button {
            class: "flex items-center gap-3 p-3 rounded-lg bg-zsu-green-light border border-zsu-green-light
                    text-grey-light text-sm hover:bg-zsu-accent-gold/20 hover:border-zsu-accent-gold
                    transition-all",
            r#type,
            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },
            if let Some(symbol) = symbol {
                span { class: "text-lg text-zsu-accent-gold", {symbol} }
            }
            if let Some(title) = title {
                span { {title} }
            }
        }
    }
}
