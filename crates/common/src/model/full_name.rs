use crate::error::CadetHubError;
use crate::CadetHubResult;

pub fn parse_last_first_middle_names(
    full_name_str: &str,
) -> CadetHubResult<(String, String, String)> {
    let full_name_parts: Vec<&str> = full_name_str.trim().split_whitespace().collect();
    if let [last, first, middle] = full_name_parts.as_slice() {
        Ok((
            last.trim().to_string(),
            first.trim().to_string(),
            middle.trim().to_string(),
        ))
    } else {
        Err(CadetHubError::general_error_with_context(format!(
            "invalid full name: '{}'",
            full_name_str
        )))
    }
}

pub trait FullNameAware {
    fn last_name(&self) -> &str;
    fn first_name(&self) -> &str;
    fn middle_name(&self) -> &str;

    fn full_name(&self) -> String {
        format!(
            "{} {} {}",
            self.last_name().to_uppercase(),
            self.first_name(),
            self.middle_name()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn should_parse_last_first_middle_names() {
        // Given
        let str = "  Doe  John  Lee  ";

        // When
        let result = parse_last_first_middle_names(str);

        // Then
        assert_that(&result).is_ok().is_equal_to(&(
            String::from("Doe"),
            String::from("John"),
            String::from("Lee"),
        ));
    }

    #[test]
    fn should_parse_last_first_middle_names_case_invalid_full_name() {
        // Given
        let str = "  Doe  John  Lee II ";

        // When
        let result = parse_last_first_middle_names(str);

        // Then
        assert_that(&result).is_err();
    }
}
