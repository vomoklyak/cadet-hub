use crate::context::application_context::FeApplicationContext;
use dioxus::prelude::*;

#[component]
pub(crate) fn TextInputField(
    #[props(default = "text".to_string(), into)] r#type: String,
    #[props(into)] name: String,
    #[props(into)] title: String,
    #[props(into)] placeholder: Option<String>,
    #[props(into)] required: Option<bool>,
    #[props] disabled: Option<bool>,
    #[props] initial_value: Option<String>,
    #[props] pattern: Option<String>,
    #[props(optional)] value: String,
    oninput: Option<EventHandler<Event<FormData>>>,
) -> Element {
    rsx! {
        div {
            label { class: "zsu-input-field-label", {title} }
            input {
                class: "validator zsu-input-field",
                r#type,
                name,
                placeholder,
                initial_value,
                pattern,
                required,
                disabled,
                value,
                oninput: move |event| {
                    if let Some(handler) = oninput {
                        handler.call(event);
                    }
                },
            }
        }
    }
}

#[component]
pub(crate) fn RegularDateInputField(
    #[props(into)] name: String,
    #[props(into)] title: String,
    #[props(into)] placeholder: Option<String>,
    #[props] required: Option<bool>,
    #[props] disabled: Option<bool>,
    #[props] initial_value: Option<String>,
    #[props(optional)] value: String,
    oninput: Option<EventHandler<Event<FormData>>>,
) -> Element {
    rsx! {
        TextInputField {
            name,
            title,
            placeholder,
            required,
            disabled,
            initial_value,
            pattern: Some(r"^(0[1-9]|1[0-2])/(0[1-9]|[12][0-9]|3[01])/\d{4}$".to_string()),
            value,
            oninput,
        }
    }
}

#[component]
pub(crate) fn SelectInputField(
    #[props(into)] name: String,
    #[props(into)] title: String,
    #[props(into)] selected: String,
    #[props] required: Option<bool>,
    #[props] disabled: Option<bool>,
    #[props] items: Vec<(String, String)>,
) -> Element {
    rsx! {
        div {
            label { class: "zsu-input-field-label", {title} }
            select {
                name,
                required,
                disabled,
                class: "appearance-none cursor-pointer zsu-input-field",
                if !required.unwrap_or_default() {
                    option {
                        class: "bg-zsu-green-dark-light",
                        initial_selected: selected.is_empty(),
                        value: "".to_string(),
                        {FeApplicationContext::translate("select-option")}
                    }
                }
                for (value , title) in items {
                    option {
                        class: "bg-zsu-green-dark-light",
                        initial_selected: selected.eq(&value),
                        value,
                        {title}
                    }
                }
            }
        }
    }
}

#[component]
pub fn PasswordInputField(
    #[props(into)] name: String,
    #[props(into)] title: String,
    #[props(into)] placeholder: Option<String>,
    #[props(into)] required: Option<bool>,
    #[props] disabled: Option<bool>,
    oninput: Option<EventHandler<Event<FormData>>>,
) -> Element {
    let mut is_visible = use_signal(|| false);
    let input_type = if is_visible() { "text" } else { "password" };

    rsx! {
        div {
            label { class: "zsu-input-field-label", {title} }
            div { class: "relative w-full",
                input {
                    class: "validator zsu-input-field",
                    r#type: "{input_type}",
                    name,
                    placeholder,
                    required,
                    disabled,
                    oninput: move |event| {
                        if let Some(handler) = oninput {
                            handler.call(event);
                        }
                    },
                }

                button {
                    class: "absolute inset-y-0 right-0 pr-3 flex items-center select-none",
                    r#type: "button",
                    onmouseenter: move |_| is_visible.toggle(),
                    onmouseleave: move |_| is_visible.toggle(),

                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" }
                        circle { cx: "12", cy: "12", r: "3" }
                    }
                }
            }
        }
    }
}
