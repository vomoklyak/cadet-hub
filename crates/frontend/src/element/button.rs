use crate::context::application_context::FeApplicationContext;
use crate::symbol::DELETE;
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
pub(crate) fn DeleteButton(
    #[props(into)] name: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "p-1.5",
            title: FeApplicationContext::translate("delete"),
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
            span { class: "text-lg text-zsu-accent-gold hover:text-zsu-red-light justify-center", {{DELETE}} }
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
    #[props(into, default = "w-36")] width: String,
    #[props(into)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        button {
            class: "{width} h-12 flex items-center gap-3 p-3 rounded-lg bg-zsu-green-light border border-zsu-green-light
                    text-grey-light text-sm hover:bg-zsu-accent-gold/20 hover:border-zsu-accent-gold
                    transition-all",
            r#type,
            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },
            if let Some(symbol) = symbol {
                span { class: "text-lg text-zsu-accent-gold justify-center", {symbol} }
            }
            if let Some(title) = title {
                span { {title} }
            }
        }
    }
}
