use dioxus::prelude::*;

#[component]
pub(crate) fn ActionDrawer(#[props(default)] extra_buttons: Vec<Element>) -> Element {
    if extra_buttons.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "fixed top-1/2 right-0 -translate-y-1/2 z-50 flex items-center \
            transition-all duration-300 ease-in-out transform translate-x-[calc(100%-40px)] hover:translate-x-0 group",

            div { class: "w-12 h-48 bg-zsu-green-dark-light border border-zsu-green-light \
                rounded-l-2xl flex flex-col items-center justify-center gap-4 cursor-pointer",

                span { class: "text-zsu-accent-gold animate-pulse text-xl", "«" }
            }

            div { class: "w-56 bg-zsu-green-dark-light border border-zsu-green-light h-96 shadow-2xl flex flex-col p-6 gap-6 rounded-lg",
                {extra_buttons.into_iter()}
            }
        }
    }
}
