use crate::CadetHubBeResult;
use common::error::CadetHubError;
use common::cadet_hub_common_prelude::Serialize;
use csv::Writer;
use serde::de::DeserializeOwned;
use std::path::Path;

pub(crate) struct CsvService {}

impl CsvService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn read_csv_file<T: DeserializeOwned>(
        &self,
        path: &Path,
    ) -> CadetHubBeResult<Vec<T>> {
        let mut csv_reader =
            csv::Reader::from_path(path).map_err(CadetHubError::general_error_with_source)?;
        let mut entries = vec![];
        for csv_entry_read_result in csv_reader.deserialize::<T>() {
            let entry = csv_entry_read_result
                .map_err(|error| CadetHubError::general_error_with_context(error.to_string()))?;
            entries.push(entry);
        }
        Ok(entries)
    }

    pub(crate) fn write_to_csv_string<T: Serialize>(
        &self,
        entities: Vec<T>,
    ) -> CadetHubBeResult<String> {
        let mut writer = Writer::from_writer(vec![]);
        for entity in entities.iter() {
            writer
                .serialize(entity)
                .map_err(CadetHubError::general_error_with_source)?;
        }
        let csv_bytes = writer
            .into_inner()
            .map_err(CadetHubError::general_error_with_source)?;
        let csv_string =
            String::from_utf8(csv_bytes).map_err(CadetHubError::general_error_with_source)?;
        Ok(csv_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::cadet_hub_common_prelude::{Deserialize, Validate};
    use spectral::assert_that;
    use spectral::prelude::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[derive(Debug, Serialize, Deserialize, Validate, PartialEq)]
    struct TestStruct {
        string: String,
        number: u32,
    }

    impl TestStruct {
        fn new(string: String, number: u32) -> Self {
            Self { string, number }
        }
    }

    #[test]
    fn should_read_from_csv_file() {
        // Given
        let test_struct_one = TestStruct::new("AA".to_string(), 1);
        let test_struct_two = TestStruct::new("BB".to_string(), 2);
        let file = csv_temp_file(vec![&test_struct_one, &test_struct_two]);
        let sut = CsvService::new();

        // When
        let result = sut
            .read_csv_file::<TestStruct>(file.path())
            .expect("failed read from csv file");

        // Then
        assert_that(&result).has_length(2);
        assert_that(&result.get(0)).is_equal_to(Some(&test_struct_one));
        assert_that(&result.get(1)).is_equal_to(Some(&test_struct_two));
    }

    fn csv_temp_file(test_structs: Vec<&TestStruct>) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("failed create temp file");
        let file_content = test_structs.iter().fold(
            "string,number".to_string(),
            |mut accumulator, test_struct| {
                accumulator.push_str(&format!("\n{},{}", test_struct.string, test_struct.number));
                accumulator
            },
        );
        file.write_all(file_content.as_bytes())
            .expect("failed write to temp file");
        file
    }

    #[test]
    fn should_write_to_csv_string() {
        // Given
        let test_structs = vec![
            TestStruct {
                string: "AA".into(),
                number: 1,
            },
            TestStruct {
                string: "BB".into(),
                number: 2,
            },
        ];
        let sut = CsvService::new();

        // When
        let result = sut
            .write_to_csv_string(test_structs)
            .expect("failed write to csv string");

        // Then
        assert!(result.contains("AA,1"));
        assert!(result.contains("BB,2"));
    }
}
