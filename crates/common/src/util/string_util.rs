pub fn split_if_not_blank(string: &str, separator: &str) -> Vec<String> {
    let string = string.trim();
    if string.is_empty() {
        Vec::new()
    } else {
        string
            .split(separator)
            .map(|part| part.trim().to_string())
            .collect()
    }
}

pub fn capitalize_list(strings: Vec<&str>) -> Vec<String> {
    strings
        .iter()
        .map(|tax_number| capitalize(tax_number))
        .collect::<Vec<_>>()
}

pub fn capitalize(string: &str) -> String {
    let string = string.trim().to_lowercase();
    let mut chars = string.chars();
    match chars.next() {
        None => "".to_string(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn uppercase_list(strings: Vec<&str>) -> Vec<String> {
    strings
        .iter()
        .map(|tax_number| uppercase(tax_number))
        .collect::<Vec<_>>()
}

pub fn uppercase(string: &str) -> String {
    string.trim().to_uppercase()
}

pub fn lowercase(string: &str) -> String {
    string.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn should_split_if_not_blank() {
        // Given
        let string = "foo, bar, buz";

        // When
        let result = split_if_not_blank(&string, ",");

        // Then
        assert_that(&result).is_equal_to(&vec![
            "foo".to_string(),
            "bar".to_string(),
            "buz".to_string(),
        ]);
    }

    #[test]
    fn should_split_if_not_blank_case_blank() {
        // Given
        let string = "   ";

        // When
        let result = split_if_not_blank(&string, ",");

        // Then
        assert_that(&result).is_equal_to(&vec![]);
    }

    #[test]
    fn should_capitalize_list() {
        // Given
        let strings = vec!["  STRING  ", "  string  "];

        // When
        let result = capitalize_list(strings);

        // Then
        assert_that(&result).is_equal_to(vec!["String".to_string(), "String".to_string()]);
    }

    #[test]
    fn should_capitalize_case_uppercase() {
        // Given
        let string = "STRING";

        // When
        let result = capitalize(string);

        // Then
        assert_that(&result).is_equal_to("String".to_string());
    }

    #[test]
    fn should_capitalize_case_lowercase() {
        // Given
        let string = "string";

        // When
        let result = capitalize(string);

        // Then
        assert_that(&result).is_equal_to("String".to_string());
    }

    #[test]
    fn should_capitalize_case_blank() {
        // Given
        let string = "  ";

        // When
        let result = capitalize(string);

        // Then
        assert_that(&result).is_equal_to("".to_string());
    }

    #[test]
    fn should_uppercase_list() {
        // Given
        let strings = vec!["  STRING  ", "  string  "];

        // When
        let result = uppercase_list(strings);

        // Then
        assert_that(&result).is_equal_to(vec!["STRING".to_string(), "STRING".to_string()]);
    }

    #[test]
    fn should_uppercase() {
        // Given
        let string = "string";

        // When
        let result = uppercase(string);

        // Then
        assert_that(&result).is_equal_to("STRING".to_string());
    }

    #[test]
    fn should_uppercase_case_blank() {
        // Given
        let string = "  ";

        // When
        let result = lowercase(string);

        // Then
        assert_that(&result).is_equal_to("".to_string());
    }

    #[test]
    fn should_lowercase() {
        // Given
        let string = "STRING";

        // When
        let result = lowercase(string);

        // Then
        assert_that(&result).is_equal_to("string".to_string());
    }

    #[test]
    fn should_lowercase_case_blank() {
        // Given
        let string = "  ";

        // When
        let result = lowercase(string);

        // Then
        assert_that(&result).is_equal_to("".to_string());
    }
}
