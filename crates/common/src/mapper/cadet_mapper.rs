use crate::model::{
    parse_last_first_middle_names, Cadet, CadetBuilder, CadetCourse, CadetCourseBuilder,
    CadetCourseEntry, FullNameAware, ImpexCadetCourseEntry, ImpexCadetCourseEntryBuilder,
};
use crate::util::date_time_util::{
    forward_slash_m_d_y_str_as_utc_timestamp, utc_timestamp_as_forward_slash_m_d_y_str,
};

impl From<&ImpexCadetCourseEntry> for Cadet {
    fn from(value: &ImpexCadetCourseEntry) -> Self {
        let (last_name, first_name, middle_name) =
            parse_last_first_middle_names(value.full_name()).expect("failed parse full name");
        let birth_date = forward_slash_m_d_y_str_as_utc_timestamp(value.birth_date())
            .expect("failed parse birth_date");
        CadetBuilder::default()
            .tax_number(value.tax_number())
            .last_name(last_name)
            .first_name(first_name)
            .middle_name(middle_name)
            .birth_date(birth_date)
            .build()
            .expect("failed build CadetBuilder")
    }
}

impl From<&ImpexCadetCourseEntry> for CadetCourse {
    fn from(value: &ImpexCadetCourseEntry) -> Self {
        let start_date = forward_slash_m_d_y_str_as_utc_timestamp(value.start_date())
            .expect("failed parse start_date");
        let end_date = forward_slash_m_d_y_str_as_utc_timestamp(value.end_date())
            .expect("failed parse end_date");
        CadetCourseBuilder::default()
            .military_rank(value.military_rank())
            .source_unit(value.source_unit())
            .specialty_name(value.specialty_name())
            .specialty_code(value.specialty_code())
            .specialty_mos_code(value.specialty_mos_code())
            .category(value.category())
            .training_location(value.training_location())
            .start_date(start_date)
            .end_date(end_date)
            .completion_order_number(value.completion_order_number())
            .completion_certificate_number(value.completion_certificate_number())
            .notes(value.notes().clone())
            .build()
            .expect("failed build CadetCourseBuilder")
    }
}

impl From<&CadetCourseEntry> for ImpexCadetCourseEntry {
    fn from(value: &CadetCourseEntry) -> Self {
        let birth_date = utc_timestamp_as_forward_slash_m_d_y_str(value.birth_date().clone())
            .expect("failed to convert birth");
        let start_date = utc_timestamp_as_forward_slash_m_d_y_str(value.start_date().clone())
            .expect("failed to convert start_date");
        let end_date = utc_timestamp_as_forward_slash_m_d_y_str(value.end_date().clone())
            .expect("failed to convert parse end_date");
        ImpexCadetCourseEntryBuilder::default()
            .military_rank(value.military_rank())
            .tax_number(value.tax_number())
            .full_name(value.full_name())
            .birth_date(birth_date)
            .source_unit(value.source_unit())
            .specialty_name(value.specialty_name())
            .specialty_code(value.specialty_code())
            .specialty_mos_code(value.specialty_mos_code())
            .category(value.category().clone())
            .training_location(value.training_location())
            .start_date(start_date)
            .end_date(end_date)
            .completion_order_number(value.completion_order_number())
            .completion_certificate_number(value.completion_certificate_number())
            .notes(value.notes().clone())
            .build()
            .expect("failed build ImpexCadetCourseEntryBuilder")
    }
}
