use common::cadet_hub_common_prelude::*;
use common::model::User;
use common::model::{Cadet, CadetCourse};
use derive_builder::Builder;
use sqlx::FromRow;

// USER
#[derive(Default, Debug, Getters, Setters, Serialize, Deserialize, Clone, PartialEq, FromRow)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub(crate) struct UserEntity {
    id: Option<i64>,
    login: String,
    password: String,
    role: String,
}

impl From<&User> for UserEntity {
    fn from(value: &User) -> Self {
        Self {
            id: value.id().clone(),
            login: value.login().clone(),
            password: value.password().clone(),
            role: value.role().to_str(),
        }
    }
}

// CADET
#[derive(
    Default, Debug, Getters, Setters, Builder, Serialize, Deserialize, Clone, PartialEq, FromRow,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub(crate) struct CadetEntity {
    id: Option<i64>,
    tax_number: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    birth_date: i64,
}

impl From<&Cadet> for CadetEntity {
    fn from(value: &Cadet) -> Self {
        Self {
            id: value.id().clone(),
            tax_number: value.tax_number().clone(),
            first_name: value.first_name().clone(),
            middle_name: value.middle_name().clone(),
            last_name: value.last_name().clone(),
            birth_date: value.birth_date().clone(),
        }
    }
}

#[derive(
    Default, Debug, Getters, Setters, Serialize, Builder, Deserialize, Clone, PartialEq, FromRow,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub(crate) struct CadetCourseEntity {
    id: Option<i64>,
    cadet_id: i64,
    military_rank: String,
    source_unit: String,
    specialty_name: String,
    specialty_code: String,
    specialty_mos_code: String,
    category: String,
    training_location: String,
    start_date: i64,
    end_date: i64,
    completion_order_number: String,
    completion_certificate_number: String,
    notes: Option<String>,
}

impl From<&CadetCourse> for CadetCourseEntity {
    fn from(value: &CadetCourse) -> Self {
        Self {
            id: value.id().clone(),
            cadet_id: value.cadet_id().unwrap_or_default(),
            military_rank: value.military_rank().clone(),
            source_unit: value.source_unit().clone(),
            specialty_name: value.specialty_name().clone(),
            specialty_code: value.specialty_code().clone(),
            specialty_mos_code: value.specialty_mos_code().clone(),
            category: value.category().clone(),
            training_location: value.training_location().clone(),
            start_date: value.start_date().clone(),
            end_date: value.end_date().clone(),
            completion_order_number: value.completion_order_number().clone(),
            completion_certificate_number: value.completion_certificate_number().clone(),
            notes: value.notes().clone(),
        }
    }
}

#[derive(Default, Debug, Getters, Setters, Serialize, Deserialize, Clone, PartialEq, FromRow)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub(crate) struct CadetCourseEntryEntity {
    id: i64,
    cadet_id: i64,
    first_name: String,
    middle_name: String,
    last_name: String,
    birth_date: i64,
    military_rank: String,
    tax_number: String,
    source_unit: String,
    specialty_name: String,
    specialty_code: String,
    specialty_mos_code: String,
    category: String,
    training_location: String,
    start_date: i64,
    end_date: i64,
    completion_order_number: String,
    completion_certificate_number: String,
    notes: Option<String>,
}

#[derive(Default, Debug, Getters, Setters, Serialize, Deserialize, Clone, PartialEq, FromRow)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub(crate) struct CadetCourseStatisticEntryEntity {
    specialty_name: String,
    specialty_code: String,
    training_location: String,
    number_of_cadet_courses: i64,
}
