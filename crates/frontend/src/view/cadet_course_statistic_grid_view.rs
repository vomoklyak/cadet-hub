use crate::context::application_context::FeApplicationContext;
use common::model::{CadetCourseStatisticEntry, SearchCadetCourseStatisticResponse};
use dioxus::prelude::*;

#[component]
pub(crate) fn CadetCourseStatisticGridView(
    cadet_course_statistic_resource: Resource<SearchCadetCourseStatisticResponse>,
) -> Element {
    rsx! {
        div { class: "bg-zsu-green-dark-light border border-zsu-green-light rounded-lg overflow-hidden shadow-2xl",
            table { class: "zsu-table",
                thead { class: "zsu-table-thead",
                    tr {
                        th { class: "zsu-table-th",
                            {FeApplicationContext::translate("training-location")}
                        }
                        th { class: "zsu-table-th",
                            {FeApplicationContext::translate("number-of-completed-cadet-courses")}
                        }
                        th { class: "zsu-table-th" }
                    }
                }
                tbody {
                    for (training_location , entries) in cadet_course_statistic_resource
                        .suspend()?
                        .read()
                        .cloned()
                        .group_by_training_location()
                    {
                        CadetCourseTrainingLocationStatisticView { training_location, entries }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn CadetCourseTrainingLocationStatisticView(
    training_location: String,
    entries: Vec<CadetCourseStatisticEntry>,
) -> Element {
    let mut expanded = use_signal(|| false);
    let number_of_cadet_courses = entries.iter().fold(0, |accumulator, entry| {
        accumulator + entry.number_of_cadet_courses()
    });
    rsx! {
        tr {
            class: "zsu-table-tr group hover:bg-zsu-accent-gold-dark/20",
            onclick: move |event: Event<MouseData>| {
                event.prevent_default();
                event.stop_propagation();

                expanded.toggle();
            },
            td { class: "zsu-table-td", {training_location} }
            td { class: "zsu-table-td", {number_of_cadet_courses.to_string()} }
            td { class: "zsu-table-td",
                if *expanded.read() {
                    for entry in entries.iter() {
                        {
                            format!(
                                "{} {} ({}): {}",
                                FeApplicationContext::translate("specialty-code"),
                                entry.specialty_code(),
                                entry.specialty_name(),
                                entry.number_of_cadet_courses(),
                            )
                        }
                        br {}
                    }
                }
            }
        }
    }
}
