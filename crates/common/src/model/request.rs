use crate::cadet_hub_common_prelude::*;
use crate::model::{
    Cadet, CadetCourseEntry, CadetCourseStatisticEntry, ImpexCadetCourseEntry, UserRole,
};
use crate::util::string_util;
use std::collections::HashMap;

#[derive(Default, Debug, Getters, Setters, Builder, Serialize, Deserialize, Clone, PartialEq)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct PageRequest {
    page_size: i64,
    page_index: i64,
    order_by: Vec<OrderBy>,
}

impl PageRequest {
    pub fn all() -> Self {
        PageRequest {
            page_size: i64::MAX,
            ..Self::default()
        }
    }

    pub fn offset(&self) -> i64 {
        self.page_size * self.page_index
    }

    pub fn limit(&self) -> i64 {
        self.page_size
    }
}

#[derive(Default, Debug, Getters, Setters, Builder, Serialize, Deserialize, Clone, PartialEq)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct OrderBy {
    column: String,
    direction: SortDirection,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

// CADETS
#[derive(
    Default, Debug, Getters, Setters, Serialize, Deserialize, Builder, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct SearchCadetRequest {
    tax_numbers: Option<Vec<String>>,
    last_names: Option<Vec<String>>,
    birth_date_after: Option<i64>,
    birth_date_before: Option<i64>,
    page_request: PageRequest,
}

impl SearchCadetRequest {
    //noinspection DuplicatedCode
    pub fn normalize(&mut self) {
        self.tax_numbers = self.tax_numbers.clone().map(|list| {
            string_util::uppercase_list(list.iter().map(|string| string.as_str()).collect())
        });
        self.last_names = self.last_names.clone().map(|list| {
            string_util::capitalize_list(list.iter().map(|string| string.as_str()).collect())
        });
    }
}

#[derive(
    Default, Debug, Getters, Setters, Serialize, Deserialize, Builder, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct SearchCadetResponse {
    page_request: PageRequest,
    page_cadets: Vec<Cadet>,
    total_number_of_cadets: i64,
}

impl SearchCadetResponse {
    pub fn number_of_pages(&self) -> i64 {
        (self.total_number_of_cadets as f64 / self.page_request.page_size as f64).ceil() as i64
    }

    pub fn owned_page_cadets(self) -> Vec<Cadet> {
        self.page_cadets
    }
}

#[derive(
    Default, Debug, Getters, Setters, Serialize, Deserialize, Builder, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct SearchCadetCourseRequest {
    tax_numbers: Option<Vec<String>>,
    last_names: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    birth_date_after: Option<i64>,
    birth_date_before: Option<i64>,
    start_date_after: Option<i64>,
    start_date_before: Option<i64>,
    end_date_after: Option<i64>,
    end_date_before: Option<i64>,
    page_request: PageRequest,
}

impl SearchCadetCourseRequest {
    //noinspection DuplicatedCode
    pub fn normalize(&mut self) {
        self.tax_numbers = self.tax_numbers.clone().map(|list| {
            string_util::uppercase_list(list.iter().map(|string| string.as_str()).collect())
        });
        self.last_names = self.last_names.clone().map(|list| {
            string_util::capitalize_list(list.iter().map(|string| string.as_str()).collect())
        });
        self.categories = self.categories.clone().map(|list| {
            string_util::uppercase_list(list.iter().map(|string| string.as_str()).collect())
        });
    }
}

#[derive(
    Default, Debug, Getters, Setters, Serialize, Deserialize, Builder, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct SearchCadetCourseResponse {
    page_request: PageRequest,
    page_entries: Vec<CadetCourseEntry>,
    total_number_of_entries: i64,
}

impl SearchCadetCourseResponse {
    pub fn number_of_pages(&self) -> i64 {
        (self.total_number_of_entries as f64 / self.page_request.page_size as f64).ceil() as i64
    }

    pub fn owned_page_entries(self) -> Vec<CadetCourseEntry> {
        self.page_entries
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, Getters, Setters, Builder, Serialize, Deserialize, Validate,
)]
#[getset(get = "pub", set = "pub")]
#[builder(default)]
#[builder(setter(into))]
pub struct SearchCadetCourseStatisticResponse {
    entries: Vec<CadetCourseStatisticEntry>,
}

impl SearchCadetCourseStatisticResponse {
    pub fn owned_entries(self) -> Vec<CadetCourseStatisticEntry> {
        self.entries
    }

    pub fn group_by_training_location(self) -> HashMap<String, Vec<CadetCourseStatisticEntry>> {
        self.entries
            .into_iter()
            .map(|entry| (entry.training_location().clone(), entry))
            .fold(
                HashMap::new(),
                |mut accumulator, (training_location, entry)| {
                    accumulator
                        .entry(training_location)
                        .or_insert(vec![])
                        .push(entry);
                    accumulator
                },
            )
    }
}

// IMPEX
#[derive(
    Default, Debug, Getters, Setters, Serialize, Deserialize, Builder, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct ImportCadetCourseRequest {
    #[validate(nested)]
    entries: Vec<ImpexCadetCourseEntry>,
}

impl ImportCadetCourseRequest {
    pub fn owned_entries(self) -> Vec<ImpexCadetCourseEntry> {
        self.entries
    }
}

#[derive(Default, Debug, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct ImportCadetCourseResponse<T> {
    failed_entries: Vec<(ImpexCadetCourseEntry, T)>,
}

impl<T> ImportCadetCourseResponse<T> {
    pub fn new(failed_entries: Vec<(ImpexCadetCourseEntry, T)>) -> Self {
        Self { failed_entries }
    }

    pub fn owned_failed_entries(self) -> Vec<(ImpexCadetCourseEntry, T)> {
        self.failed_entries
    }
}

#[derive(
    Default, Debug, Getters, Setters, Serialize, Deserialize, Builder, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct ExportCadetCourseResponse {
    #[validate(nested)]
    entries: Vec<ImpexCadetCourseEntry>,
}

impl ExportCadetCourseResponse {
    pub fn owned_entries(self) -> Vec<ImpexCadetCourseEntry> {
        self.entries
    }
}

// USERS
#[derive(
    Default, Debug, Getters, Setters, Builder, Serialize, Deserialize, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct UpdateUserRequest {
    id: i64,
    password: Option<String>,
    role: Option<UserRole>,
}

#[derive(
    Default, Debug, Getters, Setters, Serialize, Deserialize, Builder, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct SearchUserRequest {
    logins: Option<Vec<String>>,
    roles: Option<Vec<UserRole>>,
    page_request: PageRequest,
}
