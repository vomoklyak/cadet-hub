use crate::context::application_context::FeApplicationContext;
use crate::element::button::{DeleteButton, RegularButton};
use crate::util::frontend_util;
use crate::util::frontend_util::display_if;
use crate::view::cadet_course_edit_view::CadetCourseEditView;
use common::model::{BirthDateAware, CadetCourseEntry, FullNameAware};
use dioxus::prelude::*;
use std::collections::HashSet;

#[component]
pub(crate) fn CadetCourseGridView(
    cadet_course_columns: HashSet<String>,
    cadet_course_create_button_visible: bool,
    cadet_id_signal: Signal<Option<i64>>,
    cadet_course_entry_resource: Resource<Vec<CadetCourseEntry>>,
    on_cadet_course_pre_select: Option<EventHandler<()>>,
    on_cadet_course_pre_cancel: Option<EventHandler<()>>,
) -> Element {
    // HOOK
    let mut cadet_edit_id_signal = use_signal(|| None);
    let mut cadet_course_edit_id_signal = use_signal(|| None);
    let mut cadet_course_edit_view_visible_signal = use_signal(|| false);
    let visible_column = |column: &str| cadet_course_columns.contains(column);

    // EVENT HANDLER
    let on_cadet_course_select = move |cadet_id_to_course_id: (Option<i64>, Option<i64>)| {
        if let Some(on_cadet_course_pre_select) = on_cadet_course_pre_select {
            on_cadet_course_pre_select.call(());
        }
        cadet_edit_id_signal.set(cadet_id_to_course_id.0);
        cadet_course_edit_id_signal.set(cadet_id_to_course_id.1);
        cadet_course_edit_view_visible_signal.set(true);
    };
    let on_cadet_course_delete = move |cadet_course_id: i64| async move {
        let user = FeApplicationContext::require_logged_in_user();
        match FeApplicationContext::backend_client()
            .delete_cadet_course(user, cadet_course_id)
            .await
        {
            Ok(_) => {
                cadet_course_entry_resource.restart();
            }
            Err(error) => {
                FeApplicationContext::show_global_error(error);
            }
        }
    };
    let on_cadet_course_edit_cancel = move |restart_required: bool| {
        if let Some(on_cadet_course_pre_cancel) = on_cadet_course_pre_cancel {
            on_cadet_course_pre_cancel.call(());
        }
        if restart_required {
            cadet_course_entry_resource.restart();
        }
        cadet_edit_id_signal.set(None);
        cadet_course_edit_id_signal.set(None);
        cadet_course_edit_view_visible_signal.set(false);
    };

    rsx! {
        div { style: display_if(!cadet_course_edit_view_visible_signal()),
            div { class: "bg-zsu-green-dark-light border border-zsu-green-light rounded-lg overflow-hidden shadow-2xl",
                table { class: "zsu-table",
                    thead { class: "zsu-table-thead",
                        tr {
                            if visible_column("full_name") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("full-name")}
                                }
                            }
                            if visible_column("birth_date") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("birth-date")}
                                }
                            }
                            if visible_column("tax_number") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("tax-number")}
                                }
                            }
                            if visible_column("military_rank") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("military-rank")}
                                }
                            }
                            if visible_column("source_unit") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("source-unit")}
                                }
                            }
                            if visible_column("specialty") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("specialty-name")}
                                    br {}
                                    {FeApplicationContext::translate("specialty-code")}
                                }
                            }
                            if visible_column("category") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("category")}
                                }
                            }
                            if visible_column("training_location") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("training-location")}
                                }
                            }
                            if visible_column("start_date") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("start-date")}
                                }
                            }
                            if visible_column("end_date") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("end-date")}
                                }
                            }
                            if visible_column("completion_order_number") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("completion-order-number")}
                                }
                            }
                            if visible_column("completion_certificate_number") {
                                th { class: "zsu-table-th",
                                    {FeApplicationContext::translate("completion-certificate-number")}
                                }
                            }
                            th { class: "zsu-table-th", {FeApplicationContext::translate("actions")} }
                        }
                    }
                    tbody {
                        for cadet_course_entry in cadet_course_entry_resource.suspend()?.read().iter().cloned() {
                            CadetCourseView {
                                cadet_course_columns: cadet_course_columns.clone(),
                                cadet_course_entry,
                                on_cadet_course_select,
                                on_cadet_course_delete,
                            }
                        }
                    }
                }
            }

            if cadet_course_create_button_visible
                && FeApplicationContext::require_logged_in_user().has_write_permission()
            {
                div { class: "flex justify-start w-full",
                    div { class: "p-2 flex justify-center gap-4 w-full",
                        RegularButton {
                            name: "add_button",
                            title: FeApplicationContext::translate("create"),
                            symbol: "+",
                            onclick: move |event: Event<MouseData>| {
                                event.prevent_default();
                                event.stop_propagation();

                                if let Some(on_cadet_course_pre_select) = on_cadet_course_pre_select {
                                    on_cadet_course_pre_select.call(());
                                }
                                cadet_edit_id_signal.set(cadet_id_signal());
                                cadet_course_edit_id_signal.set(None);
                                cadet_course_edit_view_visible_signal.set(true);
                            },
                        }
                    }
                }
            }
        }

        div { style: display_if(cadet_course_edit_view_visible_signal()),
            CadetCourseEditView {
                cadet_edit_id_signal,
                cadet_course_edit_id_signal,
                on_cadet_course_edit_cancel,
            }
        }
    }
}

#[component]
pub(crate) fn CadetCourseView(
    cadet_course_columns: HashSet<String>,
    cadet_course_entry: CadetCourseEntry,
    on_cadet_course_select: EventHandler<(Option<i64>, Option<i64>)>,
    on_cadet_course_delete: EventHandler<i64>,
) -> Element {
    let edit_cadet_id = cadet_course_entry.cadet_id().clone();
    let edit_cadet_course_id = cadet_course_entry.id().clone();
    let delete_cadet_course_id = cadet_course_entry.id().clone();
    let visible_column = |column: &str| cadet_course_columns.contains(column);
    rsx! {
        tr {
            class: "zsu-table-tr group hover:bg-zsu-accent-gold-dark/20",
            onclick: move |event: Event<MouseData>| {
                event.prevent_default();
                event.stop_propagation();

                on_cadet_course_select.call((Some(edit_cadet_id), Some(edit_cadet_course_id)));
            },
            if visible_column("full_name") {
                td { class: "zsu-table-td",
                    div { class: "font-medium text-grey-light",
                        {format!("{}", cadet_course_entry.full_name())}
                    }
                }
            }
            if visible_column("birth_date") {
                td { class: "zsu-table-td",
                    {format!("{}", cadet_course_entry.birth_date_as_forward_slash_m_d_y_str())}
                }
            }
            if visible_column("tax_number") {
                td { class: "zsu-table-td", {format!("{}", cadet_course_entry.tax_number())} }
            }
            if visible_column("military_rank") {
                td { class: "zsu-table-td", {format!("{}", cadet_course_entry.military_rank())} }
            }
            if visible_column("source_unit") {
                td { class: "zsu-table-td", {format!("{}", cadet_course_entry.source_unit())} }
            }
            if visible_column("specialty") {
                td { class: "zsu-table-td",
                    {cadet_course_entry.specialty_name().clone()}
                    br {}
                    {cadet_course_entry.specialty_code().clone()}
                }
            }
            if visible_column("category") {
                td { class: "zsu-table-td", {format!("{}", cadet_course_entry.category())} }
            }
            if visible_column("training_location") {
                td { class: "zsu-table-td", {format!("{}", cadet_course_entry.training_location())} }
            }
            if visible_column("start_date") {
                td { class: "zsu-table-td",
                    {frontend_util::to_date_str(cadet_course_entry.start_date().clone())}
                }
            }
            if visible_column("end_date") {
                td { class: "zsu-table-td",
                    {frontend_util::to_date_str(cadet_course_entry.end_date().clone())}
                }
            }
            if visible_column("completion_order_number") {
                td { class: "zsu-table-td",
                    {format!("{}", cadet_course_entry.completion_order_number())}
                }
            }
            if visible_column("completion_certificate_number") {
                td { class: "zsu-table-td",
                    {format!("{}", cadet_course_entry.completion_certificate_number())}
                }
            }
            td { class: "px-6 py-4 text-right",
                div { class: "flex justify-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity",
                    if FeApplicationContext::require_logged_in_user().has_write_permission() {
                        DeleteButton {
                            name: "delete_cadet_course",
                            onclick: move |event: Event<MouseData>| {
                                event.prevent_default();
                                event.stop_propagation();

                                on_cadet_course_delete.call(delete_cadet_course_id);
                            },
                        }
                    }
                }
            }
        }

    }
}
