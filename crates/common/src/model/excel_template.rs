use crate::cadet_hub_common_prelude::{
    Builder, Deserialize, Getters, Serialize, Setters, Validate,
};
use std::borrow::Cow;

#[derive(
    Default, Debug, Getters, Setters, Serialize, Deserialize, Builder, Validate, Clone, PartialEq,
)]
#[builder(default)]
#[builder(setter(into))]
#[getset(get = "pub", set = "pub")]
pub struct ExcelTemplate {
    sheet_number: u32,
    first_row: u32,
    first_column: u32,
    data: Cow<'static, [u8]>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum ExcelTemplateName {
    CadetCourseReport,
}
