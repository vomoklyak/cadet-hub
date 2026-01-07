use common::util::{date_time_util, string_util};

pub(crate) fn split_by_comma(string: &str) -> Option<Vec<String>> {
    let trimmed_string = string.trim();
    if trimmed_string.is_empty() {
        None
    } else {
        Some(string_util::split_if_not_blank(trimmed_string, ","))
    }
}

pub(crate) fn to_utc_timestamp(date: &str) -> Option<i64> {
    let trimmed_date = date.trim();
    if trimmed_date.is_empty() {
        None
    } else {
        date_time_util::forward_slash_m_d_y_str_as_utc_timestamp(trimmed_date.trim()).ok()
    }
}

pub(crate) fn to_date_str(timestamp: i64) -> Option<String> {
    date_time_util::utc_timestamp_as_forward_slash_m_d_y_str(timestamp).ok()
}

pub(crate) fn display_if(condition: bool) -> String {
    format!("display:{}", if condition { "block" } else { "none" })
}
