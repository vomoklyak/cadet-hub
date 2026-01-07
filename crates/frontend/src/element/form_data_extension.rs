use dioxus::prelude::*;

pub(crate) trait FormDataExtension {
    fn get_str(&self, field: &str) -> Option<String>;
    fn get_u32(&self, field: &str) -> Option<u32>;
}

impl FormDataExtension for Event<FormData> {
    fn get_str(&self, field: &str) -> Option<String> {
        if let Some(FormValue::Text(value)) = self.get_first(field) {
            Some(value)
        } else {
            None
        }
    }

    fn get_u32(&self, field: &str) -> Option<u32> {
        self.get_str(field).and_then(|value| value.parse().ok())
    }
}
