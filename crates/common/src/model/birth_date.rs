use crate::util::date_time_util::utc_timestamp_as_forward_slash_m_d_y_str;

pub trait BirthDateAware {
    fn birth_date(&self) -> &i64;

    fn birth_date_as_forward_slash_m_d_y_str(&self) -> String {
        utc_timestamp_as_forward_slash_m_d_y_str(self.birth_date().clone())
            .expect("failed convert timestamp to birth date")
    }
}
