use dioxus::prelude::*;

pub(crate) trait FormDataExtension {
    fn get_str(&self, field: &str) -> Option<String>;
}

impl FormDataExtension for Event<FormData> {
    fn get_str(&self, field: &str) -> Option<String> {
        if let Some(FormValue::Text(value)) = self.get_first(field) {
            Some(value)
        } else {
            None
        }
    }
}
