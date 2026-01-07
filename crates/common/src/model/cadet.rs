use crate::cadet_hub_common_prelude::*;
use crate::model::{BirthDateAware, FullNameAware};
use crate::util::string_util;
use crate::validator::validator::{
    consistent_tax_number_with_birth_date_or_passport_number_entry, forward_slash_m_d_y_date,
    full_name, not_blank_str,
};

// CADET
pub const CADET_STRUCT_NAME: &str = "Cadet";

#[derive(
    Default, Debug, Clone, PartialEq, Getters, Setters, Builder, Serialize, Deserialize, Validate,
)]
#[getset(get = "pub", set = "pub")]
#[builder(default)]
#[builder(setter(into))]
pub struct Cadet {
    id: Option<i64>,
    #[validate(custom(function = "not_blank_str"))]
    tax_number: String,
    #[validate(custom(function = "not_blank_str"))]
    first_name: String,
    #[validate(custom(function = "not_blank_str"))]
    middle_name: String,
    #[validate(custom(function = "not_blank_str"))]
    last_name: String,
    birth_date: i64,
}

impl Cadet {
    pub fn require_id(&self) -> i64 {
        self.id.unwrap()
    }

    pub fn normalize(&mut self) {
        self.set_first_name(string_util::capitalize(self.first_name()));
        self.set_middle_name(string_util::capitalize(self.middle_name()));
        self.set_last_name(string_util::capitalize(self.last_name()));
        self.set_tax_number(string_util::uppercase(self.tax_number()));
    }
}

impl FullNameAware for Cadet {
    fn last_name(&self) -> &str {
        self.last_name()
    }

    fn first_name(&self) -> &str {
        self.first_name()
    }

    fn middle_name(&self) -> &str {
        self.middle_name()
    }
}

impl BirthDateAware for Cadet {
    fn birth_date(&self) -> &i64 {
        &self.birth_date
    }
}

// CADET COURSE
pub const CADET_COURSE_STRUCT_NAME: &str = "Cadet_Course";
#[derive(
    Default, Debug, Clone, PartialEq, Getters, Setters, Builder, Serialize, Deserialize, Validate,
)]
#[getset(get = "pub", set = "pub")]
#[builder(default)]
#[builder(setter(into))]
pub struct CadetCourse {
    id: Option<i64>,
    cadet_id: Option<i64>,
    #[validate(custom(function = "not_blank_str"))]
    military_rank: String,
    #[validate(custom(function = "not_blank_str"))]
    source_unit: String,
    #[validate(custom(function = "not_blank_str"))]
    specialty_name: String,
    #[validate(custom(function = "not_blank_str"))]
    specialty_code: String,
    specialty_mos_code: String,
    #[validate(custom(function = "not_blank_str"))]
    category: String,
    #[validate(custom(function = "not_blank_str"))]
    training_location: String,
    start_date: i64,
    end_date: i64,
    #[validate(custom(function = "not_blank_str"))]
    completion_order_number: String,
    #[validate(custom(function = "not_blank_str"))]
    completion_certificate_number: String,
    notes: Option<String>,
}

impl CadetCourse {
    pub fn require_id(&self) -> i64 {
        self.id.expect("id not set")
    }

    pub fn require_cadet_id(&self) -> i64 {
        self.cadet_id.expect("cadet id not set")
    }

    pub fn normalize(&mut self) {
        self.set_military_rank(string_util::lowercase(self.military_rank()));
        self.set_category(string_util::uppercase(self.category()));
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, Getters, Setters, Builder, Serialize, Deserialize, Validate,
)]
#[getset(get = "pub", set = "pub")]
#[builder(default)]
#[builder(setter(into))]
pub struct CadetCourseEntry {
    id: i64,
    cadet_id: i64,
    #[validate(custom(function = "not_blank_str"))]
    first_name: String,
    #[validate(custom(function = "not_blank_str"))]
    middle_name: String,
    #[validate(custom(function = "not_blank_str"))]
    last_name: String,
    birth_date: i64,
    #[validate(custom(function = "not_blank_str"))]
    military_rank: String,
    tax_number: String,
    #[validate(custom(function = "not_blank_str"))]
    source_unit: String,
    #[validate(custom(function = "not_blank_str"))]
    specialty_name: String,
    #[validate(custom(function = "not_blank_str"))]
    specialty_code: String,
    specialty_mos_code: String,
    #[validate(custom(function = "not_blank_str"))]
    category: String,
    #[validate(custom(function = "not_blank_str"))]
    training_location: String,
    start_date: i64,
    end_date: i64,
    #[validate(custom(function = "not_blank_str"))]
    completion_order_number: String,
    #[validate(custom(function = "not_blank_str"))]
    completion_certificate_number: String,
    notes: Option<String>,
}

impl FullNameAware for CadetCourseEntry {
    fn last_name(&self) -> &str {
        self.last_name()
    }

    fn first_name(&self) -> &str {
        self.first_name()
    }

    fn middle_name(&self) -> &str {
        self.middle_name()
    }
}

impl BirthDateAware for CadetCourseEntry {
    fn birth_date(&self) -> &i64 {
        &self.birth_date
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, Getters, Setters, Builder, Serialize, Deserialize, Validate,
)]
#[getset(get = "pub", set = "pub")]
#[builder(default)]
#[builder(setter(into))]
pub struct CadetCourseStatisticEntry {
    #[validate(custom(function = "not_blank_str"))]
    training_location: String,
    #[validate(custom(function = "not_blank_str"))]
    specialty_name: String,
    #[validate(custom(function = "not_blank_str"))]
    specialty_code: String,
    number_of_cadet_courses: i64,
}

#[derive(
    Default, Debug, Clone, PartialEq, Getters, Setters, Builder, Serialize, Deserialize, Validate,
)]
#[getset(get = "pub", set = "pub")]
#[builder(default)]
#[builder(setter(into))]
#[validate(schema(function = "consistent_tax_number_with_birth_date_or_passport_number_entry"))]
pub struct ImpexCadetCourseEntry {
    #[validate(custom(function = "not_blank_str"))]
    military_rank: String,
    #[validate(custom(function = "not_blank_str"))]
    #[validate(custom(function = "full_name"))]
    full_name: String,
    #[validate(custom(function = "forward_slash_m_d_y_date"))]
    birth_date: String,
    #[validate(custom(function = "not_blank_str"))]
    tax_number: String,
    #[validate(custom(function = "not_blank_str"))]
    source_unit: String,
    #[validate(custom(function = "not_blank_str"))]
    specialty_name: String,
    #[validate(custom(function = "not_blank_str"))]
    specialty_code: String,
    specialty_mos_code: String,
    #[validate(custom(function = "not_blank_str"))]
    category: String,
    #[validate(custom(function = "not_blank_str"))]
    training_location: String,
    #[validate(custom(function = "forward_slash_m_d_y_date"))]
    start_date: String,
    #[validate(custom(function = "forward_slash_m_d_y_date"))]
    end_date: String,
    #[validate(custom(function = "not_blank_str"))]
    completion_order_number: String,
    #[validate(custom(function = "not_blank_str"))]
    completion_certificate_number: String,
    notes: Option<String>,
    error: Option<String>,
}
