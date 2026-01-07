use crate::repository::entity::{
    CadetCourseEntity, CadetCourseEntryEntity, CadetCourseStatisticEntryEntity, CadetEntity,
};
use common::model::{
    Cadet, CadetBuilder, CadetCourse, CadetCourseBuilder, CadetCourseEntry,
    CadetCourseEntryBuilder, CadetCourseStatisticEntry, CadetCourseStatisticEntryBuilder,
};

impl From<CadetEntity> for Cadet {
    fn from(value: CadetEntity) -> Self {
        CadetBuilder::default()
            .id(value.id().clone())
            .tax_number(value.tax_number().clone())
            .first_name(value.first_name().clone())
            .middle_name(value.middle_name().clone())
            .last_name(value.last_name().clone())
            .birth_date(value.birth_date().clone())
            .build()
            .expect("failed build Cadet")
    }
}

impl From<CadetCourseEntity> for CadetCourse {
    fn from(value: CadetCourseEntity) -> Self {
        CadetCourseBuilder::default()
            .id(value.id().clone())
            .cadet_id(Some(value.cadet_id().clone()))
            .military_rank(value.military_rank().clone())
            .source_unit(value.source_unit().clone())
            .specialty_name(value.specialty_name().clone())
            .specialty_code(value.specialty_code().clone())
            .specialty_mos_code(value.specialty_mos_code().clone())
            .category(value.category().clone())
            .training_location(value.training_location().clone())
            .start_date(value.start_date().clone())
            .end_date(value.end_date().clone())
            .completion_order_number(value.completion_order_number().clone())
            .completion_certificate_number(value.completion_certificate_number().clone())
            .notes(value.notes().clone())
            .build()
            .expect("failed build CadetCourse")
    }
}

impl From<CadetCourseEntryEntity> for CadetCourseEntry {
    fn from(value: CadetCourseEntryEntity) -> Self {
        CadetCourseEntryBuilder::default()
            .id(value.id().clone())
            .cadet_id(value.cadet_id().clone())
            .first_name(value.first_name().clone())
            .middle_name(value.middle_name().clone())
            .last_name(value.last_name().clone())
            .birth_date(value.birth_date().clone())
            .military_rank(value.military_rank().clone())
            .tax_number(value.tax_number().clone())
            .source_unit(value.source_unit().clone())
            .specialty_name(value.specialty_name().clone())
            .specialty_code(value.specialty_code().clone())
            .specialty_mos_code(value.specialty_mos_code().clone())
            .category(value.category().clone())
            .training_location(value.training_location().clone())
            .start_date(value.start_date().clone())
            .end_date(value.end_date().clone())
            .completion_order_number(value.completion_order_number().clone())
            .completion_certificate_number(value.completion_certificate_number().clone())
            .notes(value.notes().clone())
            .build()
            .expect("failed build CadetCourseEntry")
    }
}

impl From<CadetCourseStatisticEntryEntity> for CadetCourseStatisticEntry {
    fn from(value: CadetCourseStatisticEntryEntity) -> Self {
        CadetCourseStatisticEntryBuilder::default()
            .specialty_name(value.specialty_name().clone())
            .specialty_code(value.specialty_code().clone())
            .training_location(value.training_location().clone())
            .number_of_cadet_courses(value.number_of_cadet_courses().clone())
            .build()
            .expect("failed build CadetCourseStatisticEntry")
    }
}
