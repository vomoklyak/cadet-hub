use common::cadet_hub_common_prelude::{Getters, Setters};
use common::model::{PageRequest, PageRequestBuilder};
use dioxus::prelude::*;

const PAGE_INDEX_SYMBOL: &str = "№";
const PAGE_SIZE_SYMBOL: &str = "#";

#[derive(Debug, Getters, Setters, Clone, Copy, PartialEq)]
#[getset(get = "pub", set = "pub")]
pub(crate) struct Pagination {
    pub page_index: i64,
    pub page_size: i64,
    pub number_of_pages: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page_index: 0,
            page_size: 20,
            number_of_pages: 0,
        }
    }
}

impl Pagination {
    pub(crate) fn reset_page_index(&mut self) {
        self.page_index = 0;
    }

    pub(crate) fn as_page_request(&self) -> PageRequest {
        PageRequestBuilder::default()
            .page_index(self.page_index().clone())
            .page_size(self.page_size().clone())
            .build()
            .expect("failed build PageRequestBuilder")
    }
}

#[component]
pub fn PaginationBar(pagination_signal: Signal<Pagination>) -> Element {
    rsx! {
        div { class: "flex items-center justify-center",
            div { class: "flex items-center justify-center px-1 bg-zsu-green-dark-light border border-zsu-green-light rounded-xl gap-1",
                // Previous Button
                PaginationButton {
                    symbol: "«",
                    disabled: pagination_signal.read().page_index == 0,
                    onclick: move |_| {
                        pagination_signal
                            .with_mut(|pagination| {
                                pagination.set_page_index(pagination.page_index() - 1);
                            });
                    },
                }

                // Page Selection
                div { class: "flex items-center gap-2 px-2",
                    PaginationSelect {
                        onchange: move |event: FormEvent| {
                            let value = event
                                .value()
                                .replace(PAGE_INDEX_SYMBOL, "")
                                .trim()
                                .parse::<i64>()
                                .unwrap_or_default();
                            pagination_signal
                                .with_mut(|pagination| {
                                    pagination.set_page_index(value - 1);
                                });
                        },
                        options: rsx! {
                            for page_index in 0..pagination_signal.read().number_of_pages {
                                option {
                                    class: "bg-zsu-green-dark-light",
                                    selected: page_index == pagination_signal.read().page_index,
                                    {format!("{} {}", PAGE_INDEX_SYMBOL, page_index + 1)}
                                }
                            }
                        },
                    }

                    // Page Size Selection
                    PaginationSelect {
                        onchange: move |event: FormEvent| {
                            let value = event
                                .value()
                                .replace(PAGE_SIZE_SYMBOL, "")
                                .trim()
                                .parse::<i64>()
                                .unwrap_or_default();
                            pagination_signal
                                .with_mut(|pagination| {
                                    pagination.set_page_index(0);
                                    pagination.set_page_size(value);
                                });
                        },
                        options: rsx! {
                            for page_size_index in 1..=5 {
                                option {
                                    class: "bg-zsu-green-dark-light",
                                    selected: *pagination_signal.read().page_size() == page_size_index * 20,
                                    {format!("{} {}", PAGE_SIZE_SYMBOL, page_size_index * 20)}
                                }
                            }
                        },
                    }
                }

                // Next Button
                PaginationButton {
                    symbol: "»",
                    disabled: *pagination_signal.read().page_index() + 1
                        >= *pagination_signal.read().number_of_pages(),
                    onclick: move |_| {
                        pagination_signal
                            .with_mut(|pagination| {
                                pagination.set_page_index(pagination.page_index() + 1);
                            });
                    },
                }
            }
        }
    }
}

#[component]
fn PaginationButton(
    #[props(into)] symbol: String,
    #[props(into)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "px-2 py-2 bg-zsu-green-dark-light border border-zsu-green-light text-zsu-accent-gold hover:bg-zsu-accent-gold/20 hover:border-zsu-accent-gold transition-all duration-300 disabled:opacity-30 disabled:cursor-not-allowed rounded-lg font-bold cursor-pointer",
            disabled,
            onclick: move |event| {
                onclick.call(event);
            },
            span { class: "text-zsu-accent-gold animate-pulse text-xl", {symbol} }
        }
    }
}

#[component]
fn PaginationSelect(options: Element, onchange: EventHandler<FormEvent>) -> Element {
    rsx! {
        select {
            class: "appearance-none bg-zsu-green-dark-light border border-zsu-green-light text-grey-light px-2 py-2 rounded-lg focus:outline-none focus:border-zsu-accent-gold cursor-pointer hover:bg-zsu-accent-gold/20 transition-colors",
            onchange: move |event| {
                event.stop_propagation();
                onchange.call(event);
            },
            {options}
        }
    }
}
