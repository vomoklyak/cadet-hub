use crate::error::CadetHubError;
use crate::CadetHubResult;
use chrono::{DateTime, NaiveDate};

static M_D_Y_FORMAT: &str = "%m/%d/%Y";

pub fn forward_slash_m_d_y_str_as_utc_timestamp(date_str: &str) -> CadetHubResult<i64> {
    let timestamp = NaiveDate::parse_from_str(date_str, M_D_Y_FORMAT)
        .map(|date| {
            date.and_hms_opt(0, 0, 0)
                .unwrap_or_default()
                .and_utc()
                .timestamp()
        })
        .map_err(|error| {
            let message =
                format!("failed to parse date: pattern={M_D_Y_FORMAT}, date_str={date_str}",);
            CadetHubError::general_error(Some(error), Some(message), false)
        })?;
    Ok(timestamp)
}

pub fn utc_timestamp_as_forward_slash_m_d_y_str(timestamp: i64) -> CadetHubResult<String> {
    DateTime::from_timestamp(timestamp, 0)
        .map(|dt| dt.format(M_D_Y_FORMAT).to_string())
        .ok_or(CadetHubError::general_error_with_context(format!(
            "failed to parse timestamp: pattern={M_D_Y_FORMAT}, timestamp={timestamp}"
        )))
}

pub fn days_since_base_tax_number_date(date_str: &str) -> CadetHubResult<i64> {
    let base_tax_number_date =
        NaiveDate::from_ymd_opt(1899, 12, 31).expect("failed to create date");
    let date = NaiveDate::parse_from_str(date_str, M_D_Y_FORMAT).map_err(|error| {
        let message = format!("failed to parse date: pattern={M_D_Y_FORMAT}, date_str={date_str}");
        CadetHubError::general_error(Some(error), Some(message), false)
    })?;
    Ok(date.signed_duration_since(base_tax_number_date).num_days())
}

#[cfg(test)]
mod date_time_util_test {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn should_parse_m_d_y_str_as_utc_timestamp_case_padded_date() {
        // Given
        let date_str = "01/02/2000";

        // When
        let result = forward_slash_m_d_y_str_as_utc_timestamp(date_str).unwrap();

        // Then
        assert_eq!(result, 946771200);
    }

    #[test]
    fn should_parse_m_d_y_str_as_utc_timestamp_case_unpadded_date() {
        // Given
        let padded_date_str = "1/2/2000";

        // When
        let result = forward_slash_m_d_y_str_as_utc_timestamp(padded_date_str).unwrap();

        // Then
        assert_that(&result).is_equal_to(946771200);
    }

    #[test]
    fn should_parse_m_d_y_str_as_utc_timestamp_case_invalid_formats() {
        // Given
        let date_str = "01-02-2000";

        // When
        let result = forward_slash_m_d_y_str_as_utc_timestamp(date_str);

        // Then
        assert_that(&result).is_err();
    }

    #[test]
    fn should_parse_m_d_y_str_as_utc_timestamp_case_impossible_dates() {
        // Given
        let date_str = "02/30/2000";

        // When
        let result = forward_slash_m_d_y_str_as_utc_timestamp(date_str);

        // Then
        assert_that(&result).is_err();
    }

    #[test]
    fn should_parse_utc_timestamp_as_forward_slash_m_d_y_str() {
        // Given
        let utc_timestamp = 946771200;

        // When
        let result = utc_timestamp_as_forward_slash_m_d_y_str(utc_timestamp).unwrap();

        // Then
        assert_that(&result).is_equal_to("01/02/2000".to_string());
    }

    #[test]
    fn should_parse_utc_timestamp_as_forward_slash_m_d_y_str_case_invalid_timestamp() {
        // Given
        let utc_timestamp = i64::MAX;

        // When
        let result = utc_timestamp_as_forward_slash_m_d_y_str(utc_timestamp);

        // Then
        assert_that(&result).is_err();
    }
}
