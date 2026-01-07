use crate::context::application_context::FeApplicationContext;
use crate::element::button::RegularButton;
use crate::element::form_data_extension::FormDataExtension;
use crate::element::input_field::{SelectInputField, TextInputField};
use crate::error::frontend_error::CadetHubFeError;
use crate::view::modal_view::{Dialog, InputModalView};
use crate::CadetHubFeResult;
use common::model::{
    ExcelTemplate, ExcelTemplateName, ImpexCadetCourseEntry, ReadExcelFileRequestBuilder,
};
use common::model::{
    ExportCadetCourseResponse, ImportCadetCourseRequestBuilder, PageRequest, ReadExcelFileRequest,
    SearchCadetCourseRequest, WriteExcelFileRequest,
};
use dioxus::prelude::*;
use std::collections::HashMap;
use crate::symbol::{ADD, LEFT, EXPORT, IMPORT};

const IMPORT_FILE_FORMAT: &str = "xlsx";

// IMPORT
#[component]
pub(crate) fn CadetCourseImportButton(cadet_course_import_view_visible: Signal<bool>) -> Element {
    rsx! {
        RegularButton {
            name: "import_cadet_courses",
            title: FeApplicationContext::translate("import"),
            symbol: IMPORT,
            onclick: move |event: MouseEvent| {
                event.prevent_default();
                event.stop_propagation();

                cadet_course_import_view_visible.set(true);
            },
        }
    }
}

#[component]
pub(crate) fn CadetCourseImportView(
    cadet_course_import_view_visible: Signal<bool>,
    on_complete: EventHandler<()>,
) -> Element {
    if !cadet_course_import_view_visible() {
        return rsx! {};
    }

    let excel_template = use_resource(move || async move {
        let user = FeApplicationContext::require_logged_in_user();
        FeApplicationContext::backend_client()
            .get_excel_template(user, ExcelTemplateName::CadetCourseReport)
            .await
            .unwrap_or_else(|error| {
                let error = CadetHubFeError::fe_common_error_with_source(
                    "error-cadet-search",
                    &error.to_string(),
                );
                FeApplicationContext::show_global_error(error);
                ExcelTemplate::default()
            })
    })
    .suspend()?
    .read()
    .clone();

    let on_submit = move |event: FormEvent| {
        spawn(async move {
            event.prevent_default();
            event.stop_propagation();

            let Some(file_handle) = rfd::AsyncFileDialog::new()
                .add_filter(IMPORT_FILE_FORMAT, &[IMPORT_FILE_FORMAT])
                .pick_file()
                .await
            else {
                // file dialog was closed
                return;
            };

            FeApplicationContext::show_global_spinner();
            let skip_optional_validation: bool = event
                .get_str("skip_optional_validation")
                .and_then(|value| value.parse().ok())
                .unwrap_or_default();
            let request = ReadExcelFileRequestBuilder::default()
                .path(file_handle.path().to_str().unwrap_or_default().to_string())
                .sheet_number(event.get_u32("sheet_number"))
                .first_column(event.get_u32("first_column"))
                .first_row(event.get_u32("first_row"))
                .last_row(event.get_u32("last_row"))
                .skip_optional_validation(skip_optional_validation)
                .build()
                .expect("failed build ReadExcelFileRequest");
            match import_entries(request).await {
                Ok((total_number_of_entries, failed_entries)) => {
                    if failed_entries.is_empty() {
                        show_import_succeeded_info(total_number_of_entries);
                    } else {
                        show_export_failed_entry_dialog(total_number_of_entries, failed_entries);
                    }
                    on_complete.call(());
                }
                Err(error) => {
                    FeApplicationContext::show_global_error(error);
                }
            }
            FeApplicationContext::hide_global_spinner();
        });
    };

    rsx! {
        InputModalView {
            content: rsx! {
                div {
                    span { class: "block text-justify",
                        {
                            let context = HashMap::from([
                                ("sheet_number", excel_template.sheet_number().to_string()),
                                ("first_column", excel_template.first_column().to_string()),
                                ("first_row", excel_template.first_row().to_string()),
                            ]);
                            {
                                FeApplicationContext::translate_with_context(
                                    "excel-import-description",
                                    context,
                                )
                            }
                        }
                    }
                    form { onsubmit: on_submit,
                        div { class: "zsu-modal-input-grid, py-4",
                            SelectInputField {
                                name: "skip_optional_validation",
                                title: FeApplicationContext::translate("validation"),
                                selected: false.to_string(),
                                required: true,
                                items: localized_validation_modes(),
                            }

                            TextInputField {
                                name: "sheet_number",
                                title: FeApplicationContext::translate("sheet-number"),
                                placeholder: FeApplicationContext::translate("enter-sheet-number"),
                                value: excel_template.sheet_number().to_string(),
                            }

                            TextInputField {
                                name: "first_column",
                                title: FeApplicationContext::translate("first-column"),
                                placeholder: FeApplicationContext::translate("enter-first-column"),
                                value: excel_template.first_column().to_string(),
                            }

                            TextInputField {
                                name: "first_row",
                                title: FeApplicationContext::translate("first-row"),
                                placeholder: FeApplicationContext::translate("enter-first-row"),
                                value: excel_template.first_row().to_string(),
                            }

                            TextInputField {
                                name: "last_row",
                                title: FeApplicationContext::translate("last-row"),
                                placeholder: FeApplicationContext::translate("enter-last-row"),
                            }
                        }

                        div { class: "zsu-button-cell",
                            RegularButton {
                                name: "back_button",
                                title: FeApplicationContext::translate("back"),
                                symbol: LEFT,
                                onclick: move |event: Event<MouseData>| {
                                    event.prevent_default();
                                    event.stop_propagation();

                                    cadet_course_import_view_visible.set(false);
                                },
                            }

                            if FeApplicationContext::require_logged_in_user().has_write_permission() {
                                RegularButton {
                                    r#type: "submit",
                                    name: "choose_file_button",
                                    title: FeApplicationContext::translate("choose-file"),
                                    symbol: ADD,
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}

fn localized_validation_modes() -> Vec<(String, String)> {
    let localized_skip_optional_validation_false = FeApplicationContext::translate_with_context(
        "skip-optional-validation",
        HashMap::from([("value", false.to_string())]),
    );
    let localized_skip_optional_validation_true = FeApplicationContext::translate_with_context(
        "skip-optional-validation",
        HashMap::from([("value", true.to_string())]),
    );
    vec![
        (false.to_string(), localized_skip_optional_validation_false),
        (true.to_string(), localized_skip_optional_validation_true),
    ]
}

async fn import_entries(
    request: ReadExcelFileRequest,
) -> CadetHubFeResult<(usize, Vec<ImpexCadetCourseEntry>)> {
    let i18n = FeApplicationContext::i18n();
    let user = FeApplicationContext::require_logged_in_user();

    let entries: Vec<ImpexCadetCourseEntry> = FeApplicationContext::backend_client()
        .read_excel_file(user.clone(), request)
        .await
        .map_err(|error| {
            CadetHubFeError::fe_common_error_with_source(
                "error-cadet-course-file-read",
                error.to_string(),
            )
        })?;
    let total_number_of_entries = entries.len();
    let request = ImportCadetCourseRequestBuilder::default()
        .entries(entries)
        .build()
        .expect("failed to build ImportCadetCourseRequestBuilder");
    let localized_failed_entries = FeApplicationContext::backend_client()
        .import_cadet_courses(user, request)
        .await?
        .owned_failed_entries()
        .into_iter()
        .map(|(mut entry, error)| {
            entry.set_error(Some(error.localized_message(&i18n)));
            entry
        })
        .collect::<Vec<_>>();
    Ok((total_number_of_entries, localized_failed_entries))
}

fn show_import_succeeded_info(total_number_of_entries: usize) {
    let context = HashMap::from([("number_of_succeeded", total_number_of_entries.to_string())]);
    let info = FeApplicationContext::translate_with_context(
        "info-import-cadet-course-entry-succeeded",
        context,
    );
    FeApplicationContext::show_global_info(info);
}

fn show_export_failed_entry_dialog(
    total_number_of_entries: usize,
    failed_entries: Vec<ImpexCadetCourseEntry>,
) {
    let number_of_succeeded = (total_number_of_entries - failed_entries.len()).to_string();
    let number_of_failed = failed_entries.len().to_string();
    let context = HashMap::from([
        ("number_of_succeeded", number_of_succeeded),
        ("number_of_failed", number_of_failed),
    ]);
    let dialog = Dialog::new(
        FeApplicationContext::translate_with_context(
            "dialog-import-export-failed-cadet-course-entry",
            context,
        ),
        EventHandler::new(move |_| {
            let failed_entries = failed_entries.clone();
            async move {
                let file_name =
                    localized_file_name("export-import-failed-cadet-course-entry-file-name");
                if let Err(error) = export_entries(&file_name, failed_entries).await {
                    FeApplicationContext::show_global_error(error);
                }
            }
        }),
    );
    FeApplicationContext::show_global_dialog(dialog);
}
//

// EXPORT
#[component]
pub(crate) fn CadetCourseExportButton(
    cadet_course_search_request_signal: Signal<SearchCadetCourseRequest>,
) -> Element {
    let export_csv = move |event: Event<MouseData>| {
        spawn(async move {
            event.prevent_default();
            event.stop_propagation();

            FeApplicationContext::show_global_spinner();
            let entries = match search_export_entries(cadet_course_search_request_signal()).await {
                Ok(response) => response.owned_entries(),
                Err(error) => {
                    FeApplicationContext::show_global_error(error);
                    return;
                }
            };

            let file_name = localized_file_name("export-cadet-course-entry-file-name");
            if let Err(error) = export_entries(&file_name, entries).await {
                FeApplicationContext::show_global_error(error);
            };
            FeApplicationContext::hide_global_spinner();
        });
    };

    rsx! {
        RegularButton {
            name: "export_cadet_courses",
            title: FeApplicationContext::translate("export"),
            symbol: EXPORT,
            onclick: export_csv,
        }
    }
}

async fn search_export_entries(
    request: SearchCadetCourseRequest,
) -> CadetHubFeResult<ExportCadetCourseResponse> {
    let user = FeApplicationContext::require_logged_in_user();
    let backend_client = FeApplicationContext::backend_client();
    let mut request = request;
    request.set_page_request(PageRequest::all());
    backend_client
        .export_cadet_courses(user, request)
        .await
        .map_err(|error| {
            CadetHubFeError::fe_common_error_with_source(
                "error-cadet-course-search",
                &error.to_string(),
            )
        })
}

async fn export_entries(
    file_name: &str,
    entries: Vec<ImpexCadetCourseEntry>,
) -> CadetHubFeResult<()> {
    let Some(file_handle) = rfd::AsyncFileDialog::new()
        .set_file_name(file_name)
        .add_filter(IMPORT_FILE_FORMAT, &[IMPORT_FILE_FORMAT])
        .save_file()
        .await
    else {
        // file dialog was closed
        return Ok(());
    };

    let user = FeApplicationContext::require_logged_in_user();
    let write_request =
        WriteExcelFileRequest::new(file_handle.path().to_str().unwrap_or_default().to_string());
    FeApplicationContext::backend_client()
        .write_excel_file(user, write_request, entries)
        .await
        .map_err(|error| {
            CadetHubFeError::fe_common_error_with_source(
                "error-cadet-course-export",
                error.to_string(),
            )
        })
}

fn localized_file_name(file_name: &str) -> String {
    format!(
        "{}.{IMPORT_FILE_FORMAT}",
        FeApplicationContext::translate(file_name)
    )
}
