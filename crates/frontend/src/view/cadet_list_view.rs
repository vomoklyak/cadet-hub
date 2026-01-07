use crate::component::action_drawer;
use crate::component::filter_bar::FilterBar;
use crate::component::pagination_bar::{Pagination, PaginationBar};
use crate::context::application_context::FeApplicationContext;
use crate::element::button::{DeleteButton, RegularButton};
use crate::element::input_field::TextInputField;
use crate::error::frontend_error::CadetHubFeError;
use crate::util::frontend_util::{display_if, split_by_comma, to_utc_timestamp};
use crate::view::cadet_edit_view::CadetEditView;
use common::model::{BirthDateAware, Cadet, FullNameAware};
use common::model::{SearchCadetRequest, SearchCadetRequestBuilder, SearchCadetResponse};
use dioxus::prelude::*;

#[component]
pub(crate) fn CadetListView() -> Element {
    let mut pagination_signal = use_signal(Pagination::default);

    let search_cadet_page_request_signal =
        use_memo(move || pagination_signal.read().as_page_request());

    let search_cadet_request_signal = use_signal(|| {
        SearchCadetRequestBuilder::default()
            .page_request(search_cadet_page_request_signal())
            .build()
            .expect("failed build SearchCadetRequest")
    });

    let mut cadet_edit_id_signal = use_signal(|| None);
    let mut cadet_edit_tax_number_signal = use_signal(|| None);
    let mut cadet_edit_view_visible_signal = use_signal(|| false);

    let mut cadet_list_resource = use_resource(move || async move {
        let page_request = search_cadet_page_request_signal();
        let mut request = search_cadet_request_signal();
        request.set_page_request(page_request);
        let user = FeApplicationContext::require_logged_in_user();
        let response = FeApplicationContext::backend_client()
            .get_cadets_by_search_request(user, request)
            .await
            .unwrap_or_else(|error| {
                let error = CadetHubFeError::fe_common_error_with_source(
                    "error-cadet-search",
                    &error.to_string(),
                );
                FeApplicationContext::show_global_error(error);
                SearchCadetResponse::default()
            });
        pagination_signal.with_mut(|pagination| {
            pagination.set_number_of_pages(response.number_of_pages());
        });
        response.owned_page_cadets()
    });

    // EVENT HANDLER
    let on_cadet_edit_select = move |cadet: (Option<i64>, Option<String>)| {
        cadet_edit_id_signal.set(cadet.0);
        cadet_edit_tax_number_signal.set(cadet.1);
        cadet_edit_view_visible_signal.set(true);
    };

    let on_cadet_edit_cancel = move |restart_required: bool| {
        if restart_required {
            cadet_list_resource.restart();
        }
        cadet_edit_view_visible_signal.set(false);
    };

    let on_cadet_delete = move |user_id: i64| async move {
        let actor_user = FeApplicationContext::require_logged_in_user();
        match FeApplicationContext::backend_client()
            .delete_cadet(actor_user, user_id)
            .await
        {
            Ok(_) => {
                cadet_list_resource.restart();
            }
            Err(error) => {
                FeApplicationContext::show_global_error(error);
            }
        };
    };

    rsx! {
        div { style: display_if(!cadet_edit_view_visible_signal()),
            if FeApplicationContext::require_logged_in_user().has_write_permission() {
                ActionDrawer {
                    cadet_edit_id_signal,
                    cadet_edit_view_visible_signal,
                }
            }

            CadetSearchFilterBarView { search_cadet_request_signal, pagination_signal }

            div { class: "py-2",
                PaginationBar { pagination_signal }
            }

            div { class: "bg-zsu-green-dark-light border border-zsu-green-light rounded-lg overflow-hidden shadow-2xl",
                table { class: "zsu-table",
                    thead { class: "zsu-table-thead",
                        tr {
                            th { class: "zsu-table-th",
                                {FeApplicationContext::translate("full-name")}
                            }
                            th { class: "zsu-table-th",
                                {FeApplicationContext::translate("tax-number")}
                            }
                            th { class: "zsu-table-th",
                                {FeApplicationContext::translate("birth-date")}
                            }
                            th { class: "zsu-table-th", {FeApplicationContext::translate("actions")} }
                        }
                    }
                    tbody {
                        for cadet in cadet_list_resource.suspend()?.read().iter().cloned() {
                            CadetView {
                                cadet,
                                on_cadet_edit_select,
                                on_cadet_delete,
                            }
                        }
                    }
                }
            }
        }

        div { style: display_if(cadet_edit_view_visible_signal()),
            CadetEditView {
                cadet_edit_id_signal,
                cadet_edit_tax_number_signal,
                on_cadet_edit_cancel,
            }
        }
    }
}

#[rustfmt::skip]
#[component]
pub(crate) fn ActionDrawer(
    cadet_edit_id_signal: Signal<Option<i64>>,
    cadet_edit_view_visible_signal: Signal<bool>,
) -> Element {
    rsx! {
        action_drawer::ActionDrawer {
            extra_buttons: vec![
                rsx! {
                    RegularButton {
                        name: "create_cadet_button",
                        title: FeApplicationContext::translate("create"),
                        symbol : "+",
                        onclick : move |event : Event<MouseData>| {
                            event.prevent_default();
                            event.stop_propagation();

                            cadet_edit_id_signal.set(None);
                            cadet_edit_view_visible_signal.set(true);
                        }
                    }
                }
            ]
        }
    }
}

#[component]
pub(crate) fn CadetView(
    cadet: Cadet,
    on_cadet_edit_select: EventHandler<(Option<i64>, Option<String>)>,
    on_cadet_delete: EventHandler<i64>,
) -> Element {
    let edit_cadet_id = cadet.require_id();
    let delete_cadet_id = cadet.require_id();
    let edit_cadet_tax_number = cadet.tax_number().clone();
    let has_write_permission =
        FeApplicationContext::require_logged_in_user().has_write_permission();

    rsx! {
        tr {
            class: "zsu-table-tr group hover:bg-zsu-accent-gold-dark/20",
            onclick: move |event: Event<MouseData>| {
                let edit_cadet_tax_number = edit_cadet_tax_number.clone();
                async move {
                    event.prevent_default();
                    event.stop_propagation();

                    on_cadet_edit_select
                        .call((Some(edit_cadet_id), Some(edit_cadet_tax_number)));
                }
            },
            td { class: "zsu-table-td",
                div { class: "font-medium text-grey-light", {format!("{}", cadet.full_name())} }
            }
            td { class: "zsu-table-td", {format!("{}", cadet.tax_number())} }
            td { class: "zsu-table-td",
                {format!("{}", cadet.birth_date_as_forward_slash_m_d_y_str())}
            }
            td { class: "px-6 py-4 text-right",
                div { class: "flex justify-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity",
                    if has_write_permission {
                        DeleteButton {
                            name: "delete_cadet",
                            onclick: move |event: Event<MouseData>| {
                                async move {
                                    event.prevent_default();
                                    event.stop_propagation();

                                    on_cadet_delete.call(delete_cadet_id);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CadetSearchFilterBarView(
    search_cadet_request_signal: Signal<SearchCadetRequest>,
    pagination_signal: Signal<Pagination>,
) -> Element {
    let mut tax_number_filter = use_signal(String::new);
    let mut last_name_filter = use_signal(String::new);
    let mut birth_date_after_filter = use_signal(String::new);
    let mut birth_date_before_filter = use_signal(String::new);

    let on_filter_clear = move |()| {
        tax_number_filter.set(String::new());
        last_name_filter.set(String::new());
        birth_date_after_filter.set(String::new());
        birth_date_before_filter.set(String::new());
        pagination_signal.with_mut(|pagination| pagination.reset_page_index());
    };

    let on_filter_apply = move |()| {
        search_cadet_request_signal.with_mut(|request| {
            request.set_tax_numbers(split_by_comma(tax_number_filter.read().as_str()));
            request.set_last_names(split_by_comma(last_name_filter.read().as_str()));
            request.set_birth_date_after(to_utc_timestamp(birth_date_after_filter.read().as_str()));
            request
                .set_birth_date_before(to_utc_timestamp(birth_date_before_filter.read().as_str()));
        });
        pagination_signal.with_mut(|pagination| pagination.reset_page_index());
    };

    rsx! {
        FilterBar {
            filter: rsx! {
                CadetSearchFilterView {
                    tax_number_filter,
                    last_name_filter,
                    birth_date_after_filter,
                    birth_date_before_filter,
                }
            },
            on_clear: on_filter_clear,
            on_apply: on_filter_apply,
        }
    }
}

#[component]
//noinspection DuplicatedCode
fn CadetSearchFilterView(
    tax_number_filter: Signal<String>,
    last_name_filter: Signal<String>,
    birth_date_after_filter: Signal<String>,
    birth_date_before_filter: Signal<String>,
) -> Element {
    rsx! {
        div { class: "zsu-input-cell",
            TextInputField {
                name: "tax_number_filter",
                placeholder: FeApplicationContext::translate("enter-tax-number"),
                title: FeApplicationContext::translate("tax-number"),
                value: tax_number_filter(),
                oninput: move |event: FormEvent| tax_number_filter.set(event.value()),
            }
            TextInputField {
                name: "last_name_filter",
                placeholder: FeApplicationContext::translate("enter-last-name"),
                title: FeApplicationContext::translate("last-name"),
                value: last_name_filter(),
                oninput: move |event: FormEvent| last_name_filter.set(event.value()),
            }
        }

        div { class: "zsu-input-cell",
            TextInputField {
                name: "birth_date_after_filter",
                placeholder: FeApplicationContext::translate("enter-date"),
                title: FeApplicationContext::translate("birth-date-after"),
                value: birth_date_after_filter(),
                oninput: move |event: FormEvent| birth_date_after_filter.set(event.value()),
            }
            TextInputField {
                name: "birth_date_before_filter",
                placeholder: FeApplicationContext::translate("enter-date"),
                title: FeApplicationContext::translate("birth-date-before"),
                value: birth_date_before_filter(),
                oninput: move |event: FormEvent| birth_date_before_filter.set(event.value()),
            }
        }
    }
}
