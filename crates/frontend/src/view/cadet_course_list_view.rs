use crate::component::action_drawer::ActionDrawer;
use crate::component::filter_bar::FilterBar;
use crate::component::pagination_bar::{Pagination, PaginationBar};
use crate::context::application_context::FeApplicationContext;
use crate::element::button::RegularButton;
use crate::element::input_field::TextInputField;
use crate::error::frontend_error::CadetHubFeError;
use crate::util::frontend_util::{
    display_if, split_by_comma, to_utc_timestamp, to_utc_timestamp_next_day,
};
use crate::view::cadet_course_impex_view::{
    CadetCourseExportButton, CadetCourseImportButton, CadetCourseImportView,
};
use crate::view::modal_view::Dialog;
use crate::view::{cadet_course_grid_view, cadet_course_statistic_grid_view};
use common::cadet_hub_common_prelude::{Deserialize, Serialize};
use common::model::{
    PageRequest, SearchCadetCourseRequest, SearchCadetCourseRequestBuilder,
    SearchCadetCourseResponse, SearchCadetCourseStatisticResponse,
};
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::symbol::{DELETE, GRID, STATISTIC};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
enum CadetCourseViewType {
    #[default]
    Regular,
    Statistical,
}

#[component]
//noinspection DuplicatedCode
pub(crate) fn CadetCourseListView() -> Element {
    // HOOKS
    let mut cadet_course_view_type = use_signal(|| CadetCourseViewType::Regular);

    let cadet_course_edit_view_visible_signal = use_signal(|| false);

    let pagination_signal = use_signal(Pagination::default);

    let cadet_course_search_page_request_memo =
        use_memo(move || pagination_signal.read().as_page_request());

    let cadet_course_search_request_signal = use_signal(|| {
        SearchCadetCourseRequestBuilder::default()
            .page_request(cadet_course_search_page_request_memo())
            .build()
            .expect("failed build CadetCourseSearchRequest")
    });

    let mut cadet_course_list_update_signal = use_signal(|| 0);

    // EVENT HANDLERS
    let on_cadet_course_list_update = move |()| {
        cadet_course_list_update_signal.with_mut(|value| *value += 1);
    };

    let cadet_course_import_view_visible = use_signal(|| false);

    rsx! {
        CadetCourseImportView {
            cadet_course_import_view_visible,
            on_complete: on_cadet_course_list_update,
        }
        div { style: display_if(!cadet_course_edit_view_visible_signal()),
            if FeApplicationContext::require_logged_in_user().has_write_permission() {
                ActionDrawer {
                    extra_buttons: vec![
                        rsx! {
                            CadetCourseImportButton { cadet_course_import_view_visible }
                        },
                        rsx! {
                            CadetCourseExportButton { cadet_course_search_request_signal }
                        },
                        rsx! {
                            DeleteCadetCoursesButton {
                                on_complete : on_cadet_course_list_update,
                                cadet_course_search_request_signal
                            }
                        },
                    ],
                }
            }

            CadetCourseSearchFilterBarView { cadet_course_search_request_signal, pagination_signal }

            div { class: "flex items-center w-full py-1",
                div { class: "flex-1 flex justify-start gap-[2px]",
                    RegularButton {
                        name: "create_cadet_button",
                        symbol: GRID,
                        width: "w-10",
                        onclick: move |event: Event<MouseData>| {
                            event.prevent_default();
                            event.stop_propagation();
                            cadet_course_view_type.set(CadetCourseViewType::Regular);
                        },
                    }
                    RegularButton {
                        name: "create_cadet_button",
                        symbol: STATISTIC,
                        width: "w-10",
                        onclick: move |event: Event<MouseData>| {
                            event.prevent_default();
                            event.stop_propagation();
                            cadet_course_view_type.set(CadetCourseViewType::Statistical);
                        },
                    }
                }

                div { class: "flex-none flex justify-center whitespace-nowrap",
                    PaginationBar { pagination_signal }
                }

                div { class: "flex-1" }
            }
        }

        match *cadet_course_view_type.read() {
            CadetCourseViewType::Regular => rsx! {
                CadetCourseGridView {
                    pagination_signal,
                    cadet_course_edit_view_visible_signal,
                    cadet_course_search_page_request_memo,
                    cadet_course_search_request_signal,
                    cadet_course_list_update_signal,
                }
            },
            CadetCourseViewType::Statistical => rsx! {
                CadetCourseStatisticGridView {
                    pagination_signal,
                    cadet_course_search_request_signal,
                    cadet_course_list_update_signal,
                }
            },
        }
    }
}

#[component]
fn CadetCourseSearchFilterBarView(
    cadet_course_search_request_signal: Signal<SearchCadetCourseRequest>,
    pagination_signal: Signal<Pagination>,
) -> Element {
    // HOOK
    let mut tax_number_filter = use_signal(String::new);
    let mut last_name_filter = use_signal(String::new);
    let mut category_filter = use_signal(String::new);
    let mut specialty_code_filter = use_signal(String::new);
    let mut birth_date_after_filter = use_signal(String::new);
    let mut birth_date_before_filter = use_signal(String::new);
    let mut start_date_after_filter = use_signal(String::new);
    let mut start_date_before_filter = use_signal(String::new);
    let mut end_date_after_filter = use_signal(String::new);
    let mut end_date_before_filter = use_signal(String::new);
    let mut updated_after_filter = use_signal(String::new);
    let mut updated_before_filter = use_signal(String::new);

    // EVENT HANDLER
    let on_filter_clear = move |()| {
        tax_number_filter.set(String::new());
        last_name_filter.set(String::new());
        category_filter.set(String::new());
        specialty_code_filter.set(String::new());
        birth_date_after_filter.set(String::new());
        birth_date_before_filter.set(String::new());
        start_date_after_filter.set(String::new());
        start_date_before_filter.set(String::new());
        end_date_after_filter.set(String::new());
        end_date_before_filter.set(String::new());
        updated_after_filter.set(String::new());
        updated_before_filter.set(String::new());
        pagination_signal.with_mut(|pagination| pagination.reset_page_index());
    };

    let on_filter_apply = move |()| {
        cadet_course_search_request_signal.with_mut(|request| {
            request.set_tax_numbers(split_by_comma(tax_number_filter.read().as_str()));
            request.set_last_names(split_by_comma(last_name_filter.read().as_str()));
            request.set_categories(split_by_comma(category_filter.read().as_str()));
            request.set_specialty_codes(split_by_comma(specialty_code_filter.read().as_str()));
            request.set_birth_date_after(to_utc_timestamp(birth_date_after_filter.read().as_str()));
            request.set_birth_date_before(to_utc_timestamp_next_day(
                birth_date_before_filter.read().as_str(),
            ));
            request.set_start_date_after(to_utc_timestamp(start_date_after_filter.read().as_str()));
            request.set_start_date_before(to_utc_timestamp_next_day(
                start_date_before_filter.read().as_str(),
            ));
            request.set_end_date_after(to_utc_timestamp(end_date_after_filter.read().as_str()));
            request.set_end_date_before(to_utc_timestamp_next_day(
                end_date_before_filter.read().as_str(),
            ));
            request.set_updated_after(to_utc_timestamp(updated_after_filter.read().as_str()));
            request.set_updated_before(to_utc_timestamp_next_day(
                updated_before_filter.read().as_str(),
            ));
        });
        pagination_signal.with_mut(|pagination| pagination.reset_page_index());
    };

    rsx! {
        FilterBar {
            filter: rsx! {
                CadetCourseSearchFilterView {
                    tax_number_filter,
                    last_name_filter,
                    category_filter,
                    specialty_code_filter,
                    birth_date_after_filter,
                    birth_date_before_filter,
                    start_date_after_filter,
                    start_date_before_filter,
                    end_date_after_filter,
                    end_date_before_filter,
                    updated_after_filter,
                    updated_before_filter,
                }
            },
            on_clear: on_filter_clear,
            on_apply: on_filter_apply,
        }
    }
}

#[component]
//noinspection DuplicatedCode
fn CadetCourseSearchFilterView(
    tax_number_filter: Signal<String>,
    last_name_filter: Signal<String>,
    category_filter: Signal<String>,
    specialty_code_filter: Signal<String>,
    birth_date_after_filter: Signal<String>,
    birth_date_before_filter: Signal<String>,
    start_date_after_filter: Signal<String>,
    start_date_before_filter: Signal<String>,
    end_date_after_filter: Signal<String>,
    end_date_before_filter: Signal<String>,
    updated_after_filter: Signal<String>,
    updated_before_filter: Signal<String>,
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

        div { class: "zsu-input-cell",
            TextInputField {
                name: "last_name_filter",
                placeholder: FeApplicationContext::translate("enter-last-name"),
                title: FeApplicationContext::translate("last-name"),
                value: last_name_filter(),
                oninput: move |event: FormEvent| last_name_filter.set(event.value()),
            }
            TextInputField {
                name: "start_date_after_filter",
                placeholder: FeApplicationContext::translate("enter-date"),
                title: FeApplicationContext::translate("start-course-date-after"),
                value: start_date_after_filter(),
                oninput: move |event: FormEvent| start_date_after_filter.set(event.value()),
            }
            TextInputField {
                name: "start_date_before_filter",
                placeholder: FeApplicationContext::translate("enter-date"),
                title: FeApplicationContext::translate("start-course-date-before"),
                value: start_date_before_filter(),
                oninput: move |event: FormEvent| start_date_before_filter.set(event.value()),
            }
        }

        div { class: "zsu-input-cell",
            TextInputField {
                name: "category_filter",
                placeholder: FeApplicationContext::translate("enter-category"),
                title: FeApplicationContext::translate("category"),
                value: category_filter(),
                oninput: move |event: FormEvent| category_filter.set(event.value()),
            }
            TextInputField {
                name: "end_date_after_filter",
                placeholder: FeApplicationContext::translate("enter-date"),
                title: FeApplicationContext::translate("end-course-date-after"),
                value: end_date_after_filter(),
                oninput: move |event: FormEvent| end_date_after_filter.set(event.value()),
            }
            TextInputField {
                name: "end_date_before_filter",
                placeholder: FeApplicationContext::translate("enter-date"),
                title: FeApplicationContext::translate("end-course-date-before"),
                value: end_date_before_filter(),
                oninput: move |event: FormEvent| end_date_before_filter.set(event.value()),
            }
        }

        div { class: "zsu-input-cell",
            TextInputField {
                name: "specialty_code_filter",
                placeholder: FeApplicationContext::translate("enter-specialty-code"),
                title: FeApplicationContext::translate("specialty-code"),
                value: specialty_code_filter(),
                oninput: move |event: FormEvent| specialty_code_filter.set(event.value()),
            }
            TextInputField {
                name: "updated_after_filter",
                placeholder: FeApplicationContext::translate("enter-date"),
                title: FeApplicationContext::translate("updated-after"),
                value: updated_after_filter(),
                oninput: move |event: FormEvent| updated_after_filter.set(event.value()),
            }
            TextInputField {
                name: "updated_before_filter",
                placeholder: FeApplicationContext::translate("enter-date"),
                title: FeApplicationContext::translate("updated-before"),
                value: updated_before_filter(),
                oninput: move |event: FormEvent| updated_before_filter.set(event.value()),
            }
        }
    }
}

#[component]
//noinspection DuplicatedCode
fn CadetCourseGridView(
    pagination_signal: Signal<Pagination>,
    cadet_course_edit_view_visible_signal: Signal<bool>,
    cadet_course_search_page_request_memo: Memo<PageRequest>,
    cadet_course_search_request_signal: Signal<SearchCadetCourseRequest>,
    cadet_course_list_update_signal: Signal<i64>,
) -> Element {
    let cadet_course_entry_resource = use_resource(move || async move {
        cadet_course_list_update_signal.read();
        let page_request = cadet_course_search_page_request_memo();
        let mut request = cadet_course_search_request_signal();
        request.set_page_request(page_request);
        let user = FeApplicationContext::require_logged_in_user();
        let response = FeApplicationContext::backend_client()
            .get_cadet_course_entries_by_search_request(user, request)
            .await
            .unwrap_or_else(|error| {
                let error = CadetHubFeError::fe_common_error_with_source(
                    "error-cadet-course-search",
                    &error.to_string(),
                );
                FeApplicationContext::show_global_error(error);
                SearchCadetCourseResponse::default()
            });
        pagination_signal.with_mut(|pagination| {
            pagination.set_number_of_pages(response.number_of_pages());
        });
        response.owned_page_entries()
    });

    rsx! {
        cadet_course_grid_view::CadetCourseGridView {
            cadet_course_columns: HashSet::from_iter([
                "full_name".to_string(),
                "birth_date".to_string(),
                "tax_number".to_string(),
                "specialty".to_string(),
                "training_location".to_string(),
                "start_date".to_string(),
                "end_date".to_string(),
            ]),
            cadet_course_create_button_visible: false,
            cadet_id_signal: Signal::new(None),
            cadet_course_entry_resource,
            on_cadet_course_pre_select: move |()| {
                cadet_course_edit_view_visible_signal.set(true);
            },
            on_cadet_course_pre_cancel: move |()| {
                cadet_course_edit_view_visible_signal.set(false);
            },
        }
    }
}

#[component]
fn CadetCourseStatisticGridView(
    pagination_signal: Signal<Pagination>,
    cadet_course_search_request_signal: Signal<SearchCadetCourseRequest>,
    cadet_course_list_update_signal: Signal<i64>,
) -> Element {
    let cadet_course_statistic_resource: Resource<SearchCadetCourseStatisticResponse> =
        use_resource(move || async move {
            cadet_course_list_update_signal.read();
            let mut request = cadet_course_search_request_signal();
            request.set_page_request(PageRequest::all());
            let user = FeApplicationContext::require_logged_in_user();
            let response = FeApplicationContext::backend_client()
                .get_cadet_course_statistic_entries_by_search_request(user, request)
                .await
                .unwrap_or_else(|error| {
                    let error = CadetHubFeError::fe_common_error_with_source(
                        "error-cadet-course-search",
                        &error.to_string(),
                    );
                    FeApplicationContext::show_global_error(error);
                    SearchCadetCourseStatisticResponse::default()
                });
            pagination_signal.with_mut(|pagination| {
                pagination.set_number_of_pages(0);
            });
            response
        });

    rsx! {
        cadet_course_statistic_grid_view::CadetCourseStatisticGridView { cadet_course_statistic_resource }
    }
}

#[component]
fn DeleteCadetCoursesButton(
    on_complete: EventHandler<()>,
    cadet_course_search_request_signal: Signal<SearchCadetCourseRequest>,
) -> Element {
    let delete = move |event: Event<MouseData>| {
        event.prevent_default();
        event.stop_propagation();

        spawn(async move {
            let user = FeApplicationContext::require_logged_in_user();
            let number_of_cadet_courses = match FeApplicationContext::backend_client()
                .count_cadet_course_entries_by_search_request(
                    user,
                    cadet_course_search_request_signal(),
                )
                .await
            {
                Ok(count) => count,
                Err(error) => {
                    error!("failed to count cadet courses: {error:?}");
                    FeApplicationContext::show_global_error(error);
                    return;
                }
            };
            let dialog = Dialog::new(
                FeApplicationContext::translate_with_context(
                    "dialog-bulk-delete-confirmation",
                    HashMap::from([("bulk-size", number_of_cadet_courses.to_string())]),
                ),
                EventHandler::new(move |_| {
                    spawn(async move {
                        FeApplicationContext::show_global_spinner();
                        let user = FeApplicationContext::require_logged_in_user();
                        let result = FeApplicationContext::backend_client()
                            .delete_cadet_courses_by_search_request(
                                user,
                                cadet_course_search_request_signal(),
                            )
                            .await;
                        match result {
                            Ok(()) => on_complete.call(()),
                            Err(error) => {
                                error!("failed to delete cadet courses: {error:?}");
                                FeApplicationContext::show_global_error(error);
                            }
                        }
                        FeApplicationContext::hide_global_spinner();
                    });
                }),
            );
            FeApplicationContext::show_global_dialog(dialog);
        });
    };

    rsx! {
        RegularButton {
            name: "delete_cadet_courses",
            title: FeApplicationContext::translate("delete"),
            symbol: DELETE,
            onclick: delete,
        }
    }
}
