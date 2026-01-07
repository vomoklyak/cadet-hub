use crate::context::application_context::FeApplicationContext;
use crate::element::button::RegularButton;
use crate::error::frontend_error::CadetHubFeError;
use crate::view::modal_view::Dialog;
use crate::CadetHubFeResult;
use common::cadet_hub_common_prelude::*;
use common::model::ImpexCadetCourseEntry;
use common::model::{
    ExportCadetCourseResponse, ImportCadetCourseRequestBuilder, PageRequest,
    SearchCadetCourseRequest,
};
use dioxus::prelude::*;
use rfd::FileHandle;
use std::collections::HashMap;

const IMPORT_FILE_FORMAT: &str = "csv";

// IMPORT
#[component]
pub(crate) fn CadetCourseImportButton(on_complete: EventHandler<()>) -> Element {
    let import_csv = move |event: Event<MouseData>| {
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
            match import_entries(file_handle).await {
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
        RegularButton {
            name: "import_cadet_courses",
            title: FeApplicationContext::translate("import"),
            symbol: "↑",
            onclick: import_csv,
        }
    }
}

async fn import_entries(
    file_handle: FileHandle,
) -> CadetHubFeResult<(usize, Vec<ImpexCadetCourseEntry>)> {
    let i18n = FeApplicationContext::i18n();
    let user = FeApplicationContext::require_logged_in_user();

    let entries: Vec<ImpexCadetCourseEntry> = FeApplicationContext::backend_client()
        .read_csv_file(file_handle.path())
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
            let csv = failed_entries.clone();
            async move {
                let file_name =
                    localized_file_name("export-import-failed-cadet-course-entry-file-name");
                if let Err(error) = export_entries(&file_name, csv).await {
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
            symbol: "↓",
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

async fn export_entries<T: 'static + Send + Serialize>(
    file_name: &str,
    entries: Vec<T>,
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

    let csv_string = FeApplicationContext::backend_client()
        .write_to_csv_string(entries)
        .await?;
    file_handle
        .write(csv_string.as_bytes())
        .await
        .map_err(|error| {
            CadetHubFeError::fe_common_error_with_source(
                "error-cadet-course-export",
                &error.to_string(),
            )
        })
}

fn localized_file_name(file_name: &str) -> String {
    format!(
        "{}.{IMPORT_FILE_FORMAT}",
        FeApplicationContext::translate(file_name)
    )
}
