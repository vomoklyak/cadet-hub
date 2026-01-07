use crate::context::application_context::FeApplicationContext;
use crate::element::button::RegularButton;
use crate::element::form_data_extension::FormDataExtension;
use crate::element::input_field::{RegularDateInputField, TextInputField};
use crate::util::frontend_util;
use common::model::{CadetCourse, CadetCourseBuilder};
use dioxus::prelude::*;

#[component]
pub(crate) fn CadetCourseEditView(
    cadet_edit_id_signal: Signal<Option<i64>>,
    cadet_course_edit_id_signal: Signal<Option<i64>>,
    on_cadet_course_edit_cancel: EventHandler<bool>,
) -> Element {
    let cadet_course_resource = use_resource(move || async move {
        let user = FeApplicationContext::require_logged_in_user();
        match cadet_course_edit_id_signal() {
            Some(id) => FeApplicationContext::backend_client()
                .get_cadet_course(user, id)
                .await
                .unwrap_or_else(|error| {
                    FeApplicationContext::show_global_error(error);
                    CadetCourse::default()
                }),
            None => CadetCourse::default(),
        }
    });
    let cadet_course = cadet_course_resource.suspend()?.read().cloned();
    let new_cadet_course = cadet_course_edit_id_signal().is_none();
    let has_write_permission =
        FeApplicationContext::require_logged_in_user().has_write_permission();

    let on_submit = move |event: FormEvent| async move {
        event.prevent_default();
        event.stop_propagation();

        let actor_user = FeApplicationContext::require_logged_in_user();
        let start_date = event
            .get_str("start_date")
            .and_then(|start_date| frontend_util::to_utc_timestamp(&start_date))
            .unwrap_or_default();
        let end_date = event
            .get_str("end_date")
            .and_then(|end_date| frontend_util::to_utc_timestamp(&end_date))
            .unwrap_or_default();
        let result = if let Some(id) = cadet_course_edit_id_signal() {
            let cadet_course = CadetCourseBuilder::default()
                .id(id)
                .cadet_id(cadet_edit_id_signal())
                .military_rank(event.get_str("military_rank").unwrap_or_default())
                .source_unit(event.get_str("source_unit").unwrap_or_default())
                .specialty_name(event.get_str("specialty_name").unwrap_or_default())
                .specialty_code(event.get_str("specialty_code").unwrap_or_default())
                .specialty_mos_code(event.get_str("specialty_mos_code").unwrap_or_default())
                .category(event.get_str("category").unwrap_or_default())
                .training_location(event.get_str("training_location").unwrap_or_default())
                .start_date(start_date)
                .end_date(end_date)
                .completion_order_number(
                    event.get_str("completion_order_number").unwrap_or_default(),
                )
                .completion_certificate_number(
                    event
                        .get_str("completion_certificate_number")
                        .unwrap_or_default(),
                )
                .notes(event.get_str("notes").unwrap_or_default())
                .build()
                .expect("failed build Cadet");
            FeApplicationContext::backend_client()
                .update_cadet_course(actor_user, cadet_course)
                .await
        } else {
            let cadet_course = CadetCourseBuilder::default()
                .cadet_id(cadet_edit_id_signal())
                .military_rank(event.get_str("military_rank").unwrap_or_default())
                .source_unit(event.get_str("source_unit").unwrap_or_default())
                .specialty_name(event.get_str("specialty_name").unwrap_or_default())
                .specialty_code(event.get_str("specialty_code").unwrap_or_default())
                .specialty_mos_code(event.get_str("specialty_mos_code").unwrap_or_default())
                .category(event.get_str("category").unwrap_or_default())
                .training_location(event.get_str("training_location").unwrap_or_default())
                .start_date(start_date)
                .end_date(end_date)
                .completion_order_number(
                    event.get_str("completion_order_number").unwrap_or_default(),
                )
                .completion_certificate_number(
                    event
                        .get_str("completion_certificate_number")
                        .unwrap_or_default(),
                )
                .notes(event.get_str("notes").unwrap_or_default())
                .build()
                .expect("failed build Cadet");
            FeApplicationContext::backend_client()
                .create_cadet_course(actor_user, cadet_course)
                .await
        };
        match result {
            Ok(_) => on_cadet_course_edit_cancel.call(true),
            Err(error) => {
                error!("failed to create user: {error:?}");
                FeApplicationContext::show_global_error(error);
            }
        };
    };

    rsx! {
        form { onsubmit: on_submit,
            div { class: "zsu-input-grid",
                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "military_rank",
                        title: FeApplicationContext::translate("military-rank"),
                        placeholder: FeApplicationContext::translate("enter-military-rank"),
                        required: true,
                        disabled: !has_write_permission,
                        value: cadet_course.military_rank(),
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "source_unit",
                        title: FeApplicationContext::translate("source-unit"),
                        placeholder: FeApplicationContext::translate("enter-source-unit"),
                        required: true,
                        disabled: !has_write_permission,
                        value: cadet_course.source_unit(),
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "specialty_name",
                        title: FeApplicationContext::translate("specialty-name"),
                        placeholder: FeApplicationContext::translate("enter-specialty-name"),
                        required: true,
                        disabled: !has_write_permission,
                        value: cadet_course.specialty_name(),
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "specialty_code",
                        title: FeApplicationContext::translate("specialty-code"),
                        placeholder: FeApplicationContext::translate("enter-specialty-code"),
                        required: true,
                        disabled: !has_write_permission,
                        value: cadet_course.specialty_code(),
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "specialty_mos_code",
                        title: FeApplicationContext::translate("specialty-mos-code"),
                        placeholder: FeApplicationContext::translate("enter-specialty-mos-code"),
                        disabled: !has_write_permission,
                        value: cadet_course.specialty_mos_code(),
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "category",
                        title: FeApplicationContext::translate("category"),
                        placeholder: FeApplicationContext::translate("enter-category"),
                        required: true,
                        disabled: !has_write_permission,
                        value: cadet_course.category(),
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "training_location",
                        title: FeApplicationContext::translate("training-location"),
                        placeholder: FeApplicationContext::translate("enter-training-location"),
                        required: true,
                        disabled: !has_write_permission,
                        value: cadet_course.training_location(),
                    }
                }

                div { class: "zsu-input-cell",
                    RegularDateInputField {
                        name: "start_date",
                        title: FeApplicationContext::translate("start-date"),
                        placeholder: FeApplicationContext::translate("enter-date"),
                        required: true,
                        disabled: !has_write_permission,
                        value: if new_cadet_course { "" } else { frontend_util::to_date_str(cadet_course.start_date().clone()).unwrap_or_default() },
                    }
                }

                div { class: "zsu-input-cell",
                    RegularDateInputField {
                        name: "end_date",
                        title: FeApplicationContext::translate("end-date"),
                        placeholder: FeApplicationContext::translate("enter-date"),
                        required: true,
                        disabled: !has_write_permission,
                        value: if new_cadet_course { "" } else { frontend_util::to_date_str(cadet_course.end_date().clone()).unwrap_or_default() },
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "completion_order_number",
                        title: FeApplicationContext::translate("completion-order-number"),
                        placeholder: FeApplicationContext::translate("enter-completion-order-number"),
                        required: true,
                        disabled: !has_write_permission,
                        value: cadet_course.completion_order_number(),
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "completion_certificate_number",
                        title: FeApplicationContext::translate("completion-certificate-number"),
                        placeholder: FeApplicationContext::translate("enter-completion-certificate-number"),
                        required: true,
                        disabled: !has_write_permission,
                        value: cadet_course.completion_certificate_number(),
                    }
                }

                div { class: "zsu-input-cell",
                    TextInputField {
                        name: "notes",
                        title: FeApplicationContext::translate("notes"),
                        placeholder: FeApplicationContext::translate("enter-notes"),
                        required: false,
                        disabled: !has_write_permission,
                        value: cadet_course.notes().clone().unwrap_or_default(),
                    }
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

                        on_cadet_course_edit_cancel.call(false);
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
