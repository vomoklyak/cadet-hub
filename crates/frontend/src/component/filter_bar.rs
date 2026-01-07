use crate::context::application_context::FeApplicationContext;
use crate::element::button::GhostButton;
use dioxus::prelude::*;

#[component]
pub(crate) fn FilterBar(
    filter: Element,
    on_clear: EventHandler<()>,
    on_apply: EventHandler<()>,
) -> Element {
    let mut filter_panel_expanded = use_signal(|| false);

    rsx! {
        section { class: "bg-zsu-green-dark-light border border-zsu-green-light p-2 rounded-lg transition-all duration-300",
            div {
                class: "flex justify-between items-center cursor-pointer select-none",
                onclick: move |_| filter_panel_expanded.toggle(),

                div { class: "flex items-center gap-2",
                    span { class: "w-2 h-2 bg-zsu-accent-gold rounded-full animate-pulse" }
                    h3 { class: "text-xs font-bold uppercase text-zsu-accent-gold tracking-widest",
                        {FeApplicationContext::translate("search-filters")}
                    }
                }

                span {
                    class: format!(
                        "text-zsu-accent-gold transition-transform duration-300 {}",
                        if filter_panel_expanded() { "rotate-180" } else { "rotate-0" },
                    ),
                    span { class: "text-zsu-accent-gold animate-pulse text-xl inline-block transform rotate-270",
                        "«"
                    }
                }
            }

            if filter_panel_expanded() {
                form {
                    div { class: "grid grid-cols-1 md:grid-cols-4 gap-4 mt-2 animate-in fade-in slide-in-from-top-2 duration-300",
                        {filter}
                    }
                }

                div { class: "mt-4 flex justify-between",
                    GhostButton {
                        name: "clear_filters_button",
                        title: FeApplicationContext::translate("clear"),
                        onclick: move |_| {
                            on_clear.call(());
                        },
                    }
                    GhostButton {
                        name: "apply_filters_button",
                        title: FeApplicationContext::translate("apply"),
                        onclick: move |_| {
                            on_apply.call(());
                        },
                    }
                }
            }
        }
    }
}
