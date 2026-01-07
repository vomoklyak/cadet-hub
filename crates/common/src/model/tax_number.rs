use crate::cadet_hub_common_prelude::*;
use crate::error::CadetHubError;
use crate::model::tax_number::TaxNumberFormat::RegularTaxNumber;
use crate::util::date_time_util;
use crate::CadetHubResult;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaxNumberFormat {
    RegularTaxNumber(String),
    PassportTaxNumber(String),
    CardIdTaxNumber(String),
    UnknownTaxNumber(String),
}

impl TaxNumberFormat {
    pub fn from(tax_number: &str, birth_date: &str) -> Self {
        let tax_number = tax_number.trim();
        let tax_number_len = tax_number.chars().count();
        let birth_date = birth_date.trim();

        if tax_number_len == 10
            && tax_number.chars().all(|char| char.is_ascii_digit())
            && check_tax_number_match_birth_date(tax_number, birth_date).is_ok()
        {
            return RegularTaxNumber(tax_number.to_string());
        }

        if tax_number_len == 8
            && tax_number.chars().take(2).all(|char| char.is_alphabetic())
            && tax_number.chars().skip(2).all(|char| char.is_ascii_digit())
        {
            return Self::PassportTaxNumber(tax_number.to_string());
        }

        if tax_number_len == 9 && tax_number.chars().all(|char| char.is_ascii_digit()) {
            return Self::CardIdTaxNumber(tax_number.to_string());
        }

        Self::UnknownTaxNumber(tax_number.to_string())
    }
}

fn check_tax_number_match_birth_date(tax_number: &str, birth_date: &str) -> CadetHubResult<()> {
    date_time_util::days_since_base_tax_number_date(birth_date).and_then(|days| {
        if tax_number.starts_with(&days.to_string()) {
            Ok(())
        } else {
            Err(CadetHubError::general_error_with_context(
                "tax_number does not match birth_date",
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::tax_number::TaxNumberFormat::UnknownTaxNumber;
    use spectral::prelude::*;

    #[test]
    fn should_build_regular_tax_number_type() {
        // Given
        let tax_number = "4386298718";
        let birth_date = "02/02/2020";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(RegularTaxNumber(tax_number.to_string()));
    }

    #[test]
    fn should_build_regular_tax_number_type_case_trim() {
        // Given
        let tax_number = " 4386298718 ";
        let birth_date = " 02/02/2020 ";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(RegularTaxNumber(tax_number.trim().to_string()));
    }

    #[test]
    fn should_build_regular_tax_number_type_case_invalid_length() {
        // Given
        let tax_number = "43862987";
        let birth_date = "02/02/2020";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(UnknownTaxNumber(tax_number.to_string()));
    }

    #[test]
    fn should_build_regular_tax_number_type_case_invalid_date() {
        // Given
        let tax_number = "4386298718";
        let birth_date = "02/03/2020";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(UnknownTaxNumber(tax_number.to_string()));
    }

    #[test]
    fn should_build_passport_tax_number_type() {
        // Given
        let tax_number = "АН123456";
        let birth_date = "any_date";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result)
            .is_equal_to(TaxNumberFormat::PassportTaxNumber(tax_number.to_string()));
    }

    #[test]
    fn should_build_passport_tax_number_type_case_invalid_number_of_alphabetic_chars() {
        // Given
        let tax_number = "А123456";
        let birth_date = "any_date";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(UnknownTaxNumber(tax_number.to_string()));
    }

    #[test]
    fn should_build_passport_tax_number_type_case_invalid_number_of_numeric_chars() {
        // Given
        let tax_number = "АН1234567";
        let birth_date = "any_date";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(UnknownTaxNumber(tax_number.to_string()));
    }

    #[test]
    fn should_build_card_id_tax_number_type() {
        // Given
        let tax_number = "123456789";
        let birth_date = "any_date";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(TaxNumberFormat::CardIdTaxNumber(tax_number.to_string()));
    }

    #[test]
    fn should_build_card_id_tax_number_type_case_invalid_number_of_numeric_chars() {
        // Given
        let tax_number = "1234567890";
        let birth_date = "any_date";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(UnknownTaxNumber(tax_number.to_string()));
    }

    #[test]
    fn should_build_unknown_number_type() {
        // Given
        let tax_number = "any_tax_number";
        let birth_date = "any_date";

        // When
        let result = TaxNumberFormat::from(tax_number, birth_date);

        // Then
        assert_that(&result).is_equal_to(UnknownTaxNumber(tax_number.to_string()));
    }
}
