use crate::context::application_context::FeApplicationContext;
use dioxus::prelude::*;

#[component]
pub(crate) fn LoadingSpinnerView() -> Element {
    rsx! {
        if FeApplicationContext::global_spinner_visible()() {
            div { class: "z-[100] absolute left-1/2 top-1/3 -translate-x-1/2 -translate-y-1/2",
                LoadingSpinner {}
            }
        }
    }
}

#[component]
fn LoadingSpinner() -> Element {
    rsx! {
        div { class: "relative flex items-center justify-center w-12 h-12",
            // Outer Glowing Ring (Pulsing)
            div { class: "absolute inset-0 rounded-full border-4 border-zsu-accent-gold/20 animate-ping" }

            // Inner Heavy Track
            div { class: "absolute inset-0 rounded-full border-[3px] border-white/5" }

            // The Primary Solid Spinner
            svg {
                class: "w-10 h-10 animate-spin text-zsu-accent-gold",
                view_box: "0 0 24 24",
                fill: "none",
                // Shadow filter for a "glow" effect
                filter: "drop-shadow(0 0 3px rgba(212, 175, 55, 0.5))",

                circle {
                    class: "opacity-25",
                    cx: "12",
                    cy: "12",
                    r: "10",
                    stroke: "currentColor",
                    stroke_width: "4",
                }
                path {
                    class: "opacity-100",
                    fill: "currentColor",
                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                }
            }
        }
    }
}