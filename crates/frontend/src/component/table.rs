use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;

#[component]
pub(crate) fn ScrollableTable(table: Element) -> Element {
    rsx! {
        div { class: "bg-zsu-green-dark-light border border-zsu-green-light rounded-lg shadow-2xl w-full overflow-hidden",
            div { class: "overflow-x-auto", {table} }
        }
    }
}