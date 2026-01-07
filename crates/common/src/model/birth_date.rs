use crate::util::date_time_util::utc_timestamp_as_dot_dd_mm_yyyy_str;

pub trait BirthDateAware {
    fn birth_date(&self) -> &i64;

    fn birth_date_as_dot_dd_mm_yyyy_str(&self) -> String {
        utc_timestamp_as_dot_dd_mm_yyyy_str(self.birth_date().clone())
            .expect("failed convert timestamp to birth date")
    }
}
