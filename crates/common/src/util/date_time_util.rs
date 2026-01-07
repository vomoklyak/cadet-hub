use crate::error::CadetHubError;
use crate::CadetHubResult;
use chrono::{DateTime, NaiveDate, Utc};

pub const SECONDS_IN_DAY: i64 = 86_400;
pub const DD_MM_YYYY_FORMAT: &str = "%d.%m.%Y";
pub const DD_MM_YYYY_FORMAT_REGEX: &str = r"^(0?[1-9]|[12][0-9]|3[01])\.(0?[1-9]|1[0-2])\.\d{4}$";

pub fn now_utc_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn dot_dd_mm_yyyy_str_as_utc_timestamp_next_day(date_str: &str) -> CadetHubResult<i64> {
    dot_dd_mm_yyyy_str_as_utc_timestamp(date_str).map(|timestamp| timestamp + SECONDS_IN_DAY)
}

pub fn dot_dd_mm_yyyy_str_as_utc_timestamp(date_str: &str) -> CadetHubResult<i64> {
    let timestamp = NaiveDate::parse_from_str(date_str, DD_MM_YYYY_FORMAT)
        .map(|date| {
            date.and_hms_opt(0, 0, 0)
                .unwrap_or_default()
                .and_utc()
                .timestamp()
        })
        .map_err(|error| {
            let message =
                format!("failed to parse date: pattern={DD_MM_YYYY_FORMAT}, date_str={date_str}",);
            CadetHubError::general_error(Some(error), Some(message), false)
        })?;
    Ok(timestamp)
}
pub fn utc_timestamp_as_dot_dd_mm_yyyy_str(timestamp: i64) -> CadetHubResult<String> {
    DateTime::from_timestamp(timestamp, 0)
        .map(|dt| dt.format(DD_MM_YYYY_FORMAT).to_string())
        .ok_or(CadetHubError::general_error_with_context(format!(
            "failed to parse timestamp: pattern={DD_MM_YYYY_FORMAT}, timestamp={timestamp}"
        )))
}

pub fn days_since_base_tax_number_date(date_str: &str) -> CadetHubResult<i64> {
    let base_tax_number_date =
        NaiveDate::from_ymd_opt(1899, 12, 31).expect("failed to create date");
    let date = NaiveDate::parse_from_str(date_str, DD_MM_YYYY_FORMAT).map_err(|error| {
        let message =
            format!("failed to parse date: pattern={DD_MM_YYYY_FORMAT}, date_str={date_str}");
        CadetHubError::general_error(Some(error), Some(message), false)
    })?;
    Ok(date.signed_duration_since(base_tax_number_date).num_days())
}

#[cfg(test)]
mod date_time_util_test {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn should_get_now_utc_timestamp() {
        // Given
        let current_utc_timestamp = Utc::now().timestamp();

        // When
        let result = now_utc_timestamp();

        // Then
        assert_that!(result).is_greater_than_or_equal_to(current_utc_timestamp);
    }

    #[test]
    fn should_parse_dot_dd_mm_yyyy_str_as_utc_timestamp_next_day() {
        // Given
        let date_str = "20.02.2020";

        // When
        let result = dot_dd_mm_yyyy_str_as_utc_timestamp_next_day(date_str).unwrap();

        // Then
        assert_that!(result).is_equal_to(1582243200);
    }

    #[test]
    fn should_parse_dot_dd_mm_yyyy_str_as_utc_timestamp_case_padded_date() {
        // Given
        let date_str = "20.02.2020";

        // When
        let result = dot_dd_mm_yyyy_str_as_utc_timestamp(date_str).unwrap();

        // Then
        assert_that!(result).is_equal_to(1582156800);
    }

    #[test]
    fn should_parse_dot_dd_mm_yyyy_str_as_utc_timestamp_case_unpadded_date() {
        // Given
        let padded_date_str = "20.2.2020";

        // When
        let result = dot_dd_mm_yyyy_str_as_utc_timestamp(padded_date_str).unwrap();

        // Then
        assert_that(&result).is_equal_to(1582156800);
    }

    #[test]
    fn should_parse_dot_dd_mm_yyyy_str_as_utc_timestamp_case_invalid_formats() {
        // Given
        let date_str = "20-02-2020";

        // When
        let result = dot_dd_mm_yyyy_str_as_utc_timestamp(date_str);

        // Then
        assert_that(&result).is_err();
    }

    #[test]
    fn should_parse_dot_dd_mm_yyyy_str_as_utc_timestamp_case_impossible_dates() {
        // Given
        let date_str = "30.02.2000";

        // When
        let result = dot_dd_mm_yyyy_str_as_utc_timestamp(date_str);

        // Then
        assert_that(&result).is_err();
    }

    #[test]
    fn should_parse_utc_timestamp_as_dot_dd_mm_yyyy_str() {
        // Given
        let utc_timestamp = 1582156800;

        // When
        let result = utc_timestamp_as_dot_dd_mm_yyyy_str(utc_timestamp).unwrap();

        // Then
        assert_that(&result).is_equal_to("20.02.2020".to_string());
    }

    #[test]
    fn should_parse_utc_timestamp_as_dot_dd_mm_yyyy_str_case_invalid_timestamp() {
        // Given
        let utc_timestamp = i64::MAX;

        // When
        let result = utc_timestamp_as_dot_dd_mm_yyyy_str(utc_timestamp);

        // Then
        assert_that(&result).is_err();
    }
}
