use crate::cadet_hub_common_prelude::*;
use crate::model;
use crate::model::TaxNumberFormat::{PassportTaxNumber, RegularTaxNumber};
use crate::model::{ImpexCadetCourseEntry, TaxNumberFormat};
use crate::util::date_time_util;

pub fn not_blank_str(str: &str) -> Result<(), ValidationError> {
    match str.trim().len() {
        0 => Err(ValidationError::new("error-validation-blank-string")),
        _ => Ok(()),
    }
}

pub fn full_name(full_name_str: &str) -> Result<(), ValidationError> {
    match model::parse_last_first_middle_names(full_name_str) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("error-validation-invalid-full-name")),
    }
}

pub fn forward_slash_m_d_y_date(date_str: &str) -> Result<(), ValidationError> {
    match date_time_util::forward_slash_m_d_y_str_as_utc_timestamp(date_str) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("error-validation-invalid-m-d-y-date")),
    }
}

pub fn consistent_tax_number_with_birth_date_or_passport_number_entry(
    entry: &ImpexCadetCourseEntry,
) -> Result<(), ValidationError> {
    internal_consistent_tax_number_with_birth_date_or_passport_number(
        entry.tax_number(),
        entry.birth_date(),
    )
}

fn internal_consistent_tax_number_with_birth_date_or_passport_number(
    tax_number: &str,
    birth_date: &str,
) -> Result<(), ValidationError> {
    match TaxNumberFormat::from(tax_number, birth_date) {
        RegularTaxNumber { .. } | PassportTaxNumber { .. } => Ok(()),
        _ => Err(ValidationError::new(
            "error-validation-inconsistent-tax-number-birth-date-or-passport-number",
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::ImpexCadetCourseEntryBuilder;
    use spectral::prelude::*;

    #[test]
    fn should_validate_not_blank_str() {
        // Given
        let str = "  string  ";

        // When
        let result = not_blank_str(str);

        // Then
        assert_that(&result).is_ok();
    }

    #[test]
    fn should_validate_not_blank_str_case_empty() {
        // Given
        let str = "";

        // When
        let result = not_blank_str(str);

        // Then
        assert_that(&result)
            .is_err()
            .is_equal_to(ValidationError::new("error-validation-blank-string"));
    }

    #[test]
    fn should_validate_not_blank_str_case_blank() {
        // Given
        let str = "   ";

        // When
        let result = not_blank_str(str);

        // Then
        assert_that(&result)
            .is_err()
            .is_equal_to(ValidationError::new("error-validation-blank-string"));
    }

    #[test]
    fn should_validate_full_name() {
        // Given
        let str = "  Doe  John  Lee  ";

        // When
        let result = full_name(str);

        // Then
        assert_that(&result).is_ok();
    }

    #[test]
    fn should_validate_full_name_case_invalid_full_name() {
        // Given
        let str = "Doe John";

        // When
        let result = full_name(str);

        // Then
        assert_that(&result)
            .is_err()
            .is_equal_to(ValidationError::new(
                "error-validation-invalid-full-name",
            ));
    }

    #[test]
    fn should_validate_forward_slash_m_d_y_date() {
        // Given
        let date_str = "02/20/2000";

        // When
        let result = forward_slash_m_d_y_date(date_str);

        // Then
        assert_that(&result).is_ok();
    }

    #[test]
    fn should_validate_forward_slash_m_d_y_date_case_another_format() {
        // Given
        let date_str = "02-20-2000";

        // When
        let result = forward_slash_m_d_y_date(date_str);

        // Then
        assert_that(&result)
            .is_err()
            .is_equal_to(ValidationError::new("error-validation-invalid-m-d-y-date"));
    }

    #[test]
    fn should_validate_consistent_tax_number_with_birth_date_or_passport_number_entry() {
        // Given
        let entry = ImpexCadetCourseEntryBuilder::default()
            .tax_number("4388000013")
            .birth_date("02/20/2020")
            .build()
            .expect("failed build ImpexCadetCourseEntry");

        // When
        let result = consistent_tax_number_with_birth_date_or_passport_number_entry(&entry);

        // Then
        assert_that(&result).is_ok();
    }

    #[test]
    fn should_validate_consistent_tax_number_with_birth_date_or_passport_number_entry_case_passport(
    ) {
        // Given
        let entry = ImpexCadetCourseEntryBuilder::default()
            .tax_number("ВМ933877")
            .birth_date("02/20/2020")
            .build()
            .expect("failed build ImpexCadetCourseEntry");

        // When
        let result = consistent_tax_number_with_birth_date_or_passport_number_entry(&entry);

        // Then
        assert_that(&result).is_ok();
    }

    #[test]
    fn should_validate_consistent_tax_number_with_birth_date_or_passport_number_entry_case_card_id()
    {
        // Given
        let entry = ImpexCadetCourseEntryBuilder::default()
            .tax_number("123456789")
            .birth_date("02/20/2020")
            .build()
            .expect("failed build ImpexCadetCourseEntry");

        // When
        let result = consistent_tax_number_with_birth_date_or_passport_number_entry(&entry);

        // Then
        assert_that(&result)
            .is_err()
            .is_equal_to(ValidationError::new(
                "error-validation-inconsistent-tax-number-birth-date-or-passport-number",
            ));
    }

    #[test]
    fn should_validate_consistent_tax_number_with_birth_date_or_passport_number_entry_case_invalid_tax_number(
    ) {
        // Given
        let entry = ImpexCadetCourseEntryBuilder::default()
            .tax_number("1234567890")
            .birth_date("02/20/2020")
            .build()
            .expect("failed build ImpexCadetCourseEntry");

        // When
        let result = consistent_tax_number_with_birth_date_or_passport_number_entry(&entry);

        // Then
        assert_that(&result)
            .is_err()
            .is_equal_to(ValidationError::new(
                "error-validation-inconsistent-tax-number-birth-date-or-passport-number",
            ));
    }
}
