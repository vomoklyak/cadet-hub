use crate::context::application_context::FeApplicationContext;
use crate::element::button::RegularButton;
use crate::element::form_data_extension::FormDataExtension;
use crate::element::input_field::{RegularDateInputField, TextInputField};
use crate::error::frontend_error::CadetHubFeError;
use crate::util::frontend_util;
use crate::util::frontend_util::display_if;
use crate::view::cadet_course_grid_view::CadetCourseGridView;
use crate::view::modal_view::Dialog;
use common::model::TaxNumberFormat;
use common::model::TaxNumberFormat::{CardIdTaxNumber, PassportTaxNumber, RegularTaxNumber};
use common::model::{BirthDateAware, Cadet, CadetBuilder};
use common::model::{PageRequest, SearchCadetCourseRequestBuilder, SearchCadetCourseResponse};
use dioxus::prelude::*;
use std::collections::HashSet;

#[component]
pub(crate) fn CadetEditView(
    cadet_edit_id_signal: Signal<Option<i64>>,
    cadet_edit_tax_number_signal: Signal<Option<String>>,
    on_cadet_edit_cancel: EventHandler<bool>,
) -> Element {
    let cadet_course_edit_view_visible = use_signal(|| false);

    let cadet_resource = use_resource(move || async move {
        let user = FeApplicationContext::require_logged_in_user();
        match cadet_edit_id_signal() {
            Some(id) => FeApplicationContext::backend_client()
                .get_cadet(user, id)
                .await
                .unwrap_or_else(|error| {
                    FeApplicationContext::show_global_error(error);
                    Cadet::default()
                }),
            None => Cadet::default(),
        }
    });
    let cadet = cadet_resource.suspend()?.read().cloned();
    let new_cadet = cadet_edit_id_signal().is_none();
    let has_write_permission =
        FeApplicationContext::require_logged_in_user().has_write_permission();

    let on_submit = move |event: FormEvent| async move {
        event.prevent_default();
        event.stop_propagation();

        let tax_number_str = event.get_str("tax_number").unwrap_or_default();
        let birth_date_str = event.get_str("birth_date").unwrap_or_default();
        let tax_number_format = TaxNumberFormat::from(&tax_number_str, &birth_date_str);

        let perform_save = {
            let event = event.clone();
            move || {
                let event = event.clone();
                let actor_user = FeApplicationContext::require_logged_in_user();
                spawn(async move {
                    let birth_date = event
                        .get_str("birth_date")
                        .and_then(|date| frontend_util::to_utc_timestamp(&date))
                        .unwrap_or_default();

                    let mut cadet_builder = CadetBuilder::default()
                        .last_name(event.get_str("last_name").unwrap_or_default())
                        .first_name(event.get_str("first_name").unwrap_or_default())
                        .middle_name(event.get_str("middle_name").unwrap_or_default())
                        .tax_number(event.get_str("tax_number").unwrap_or_default())
                        .birth_date(birth_date)
                        .clone();

                    let result = if let Some(id) = cadet_edit_id_signal() {
                        let cadet = cadet_builder.id(id).build().expect("failed build Cadet");
                        FeApplicationContext::backend_client()
                            .update_cadet(actor_user, cadet)
                            .await
                    } else {
                        let cadet = cadet_builder.build().expect("failed build Cadet");
                        FeApplicationContext::backend_client()
                            .create_cadet(actor_user, cadet)
                            .await
                    };

                    match result {
                        Ok(_) => on_cadet_edit_cancel.call(true),
                        Err(error) => {
                            error!("failed to save cadet: {error:?}");
                            FeApplicationContext::show_global_error(error);
                        }
                    };
                });
            }
        };

        match tax_number_format {
            RegularTaxNumber(_) => {
                perform_save();
            }
            _ => {
                let message = match tax_number_format {
                    PassportTaxNumber(_) => "dialog-tax-number-passport-format",
                    CardIdTaxNumber(_) => "dialog-tax-number-card-id-format",
                    _ => "dialog-tax-number-unknown-format",
                };

                let dialog = Dialog::new(
                    FeApplicationContext::translate(message),
                    EventHandler::new(move |_| perform_save()),
                );
                FeApplicationContext::show_global_dialog(dialog);
            }
        }
    };

    rsx! {
        div { style: display_if(!cadet_course_edit_view_visible()),
            div {
                form { onsubmit: on_submit,
                    div { class: "zsu-input-grid",
                        TextInputField {
                            name: "last_name",
                            title: FeApplicationContext::translate("last-name"),
                            placeholder: FeApplicationContext::translate("enter-last-name"),
                            required: true,
                            disabled: !has_write_permission,
                            value: cadet.last_name(),
                        }

                        TextInputField {
                            name: "first_name",
                            title: FeApplicationContext::translate("first-name"),
                            placeholder: FeApplicationContext::translate("enter-first-name"),
                            required: true,
                            disabled: !has_write_permission,
                            value: cadet.first_name(),
                        }

                        TextInputField {
                            name: "middle_name",
                            title: FeApplicationContext::translate("middle-name"),
                            placeholder: FeApplicationContext::translate("enter-middle-name"),
                            required: true,
                            disabled: !has_write_permission,
                            value: cadet.middle_name(),
                        }

                        TextInputField {
                            name: "tax_number",
                            title: FeApplicationContext::translate("tax-number"),
                            placeholder: FeApplicationContext::translate("enter-tax-number"),
                            required: true,
                            disabled: !has_write_permission,
                            value: cadet.tax_number(),
                        }

                        RegularDateInputField {
                            name: "birth_date",
                            title: FeApplicationContext::translate("birth-date"),
                            placeholder: FeApplicationContext::translate("enter-date"),
                            required: true,
                            disabled: !has_write_permission,
                            value: if new_cadet { "" } else { cadet.birth_date_as_forward_slash_m_d_y_str() },
                        }
                    }

                    div { class: "zsu-button-cell",
                        RegularButton {
                            name: "back_button",
                            title: FeApplicationContext::translate("back"),
                            symbol: "←",
                            onclick: move |event: Event<MouseData>| {
                                event.prevent_default();
                                event.stop_propagation();

                                on_cadet_edit_cancel.call(false);
                            },
                        }

                        if FeApplicationContext::require_logged_in_user().has_write_permission() {
                            RegularButton {
                                r#type: "submit",
                                name: "save_button",
                                title: FeApplicationContext::translate("save"),
                                symbol: "✓",
                                onclick: move |_| {},
                            }
                        }
                    }
                }
            }
        }

        CadetCourseListView {
            cadet_edit_id_signal,
            cadet_edit_tax_number_signal,
            cadet_course_edit_view_visible,
        }
    }
}

#[component]
//noinspection DuplicatedCode
fn CadetCourseListView(
    cadet_edit_id_signal: Signal<Option<i64>>,
    cadet_edit_tax_number_signal: Signal<Option<String>>,
    cadet_course_edit_view_visible: Signal<bool>,
) -> Element {
    let cadet_course_entry_resource = use_resource(move || async move {
        if let Some(cadet_tax_number) = cadet_edit_tax_number_signal() {
            let user = FeApplicationContext::require_logged_in_user();
            let request = SearchCadetCourseRequestBuilder::default()
                .tax_numbers(vec![cadet_tax_number])
                .page_request(PageRequest::all())
                .build()
                .expect("failed build SearchCadetCourseRequest");
            FeApplicationContext::backend_client()
                .get_cadet_course_entries_by_search_request(user, request)
                .await
                .unwrap_or_else(|error| {
                    let error = CadetHubFeError::fe_common_error_with_source(
                        "error-cadet-course-search",
                        &error.to_string(),
                    );
                    FeApplicationContext::show_global_error(error);
                    SearchCadetCourseResponse::default()
                })
                .owned_page_entries()
        } else {
            vec![]
        }
    });

    rsx! {
        CadetCourseGridView {
            cadet_course_columns: HashSet::from_iter([
                "military_rank".to_string(),
                "source_unit".to_string(),
                "specialty".to_string(),
                "category".to_string(),
                "training_location".to_string(),
                "start_date".to_string(),
                "end_date".to_string(),
                "completion_order_number".to_string(),
                "completion_certificate_number".to_string(),
            ]),
            cadet_course_create_button_visible: true,
            cadet_id_signal: cadet_edit_id_signal,
            cadet_course_entry_resource,
            on_cadet_course_pre_select: move |()| {
                cadet_course_edit_view_visible.set(true);
            },
            on_cadet_course_pre_cancel: move |()| {
                cadet_course_edit_view_visible.set(false);
            },
        }
    }
}
