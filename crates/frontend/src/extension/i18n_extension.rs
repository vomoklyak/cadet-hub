use dioxus_i18n::fluent::FluentArgs;
use dioxus_i18n::prelude::I18n;
use std::collections::HashMap;

pub(crate) trait I18nExtension {
    fn try_translate_or_default<S>(&self, key: S) -> String
    where
        S: Into<String>;

    fn try_translate_with_context_or_default<S, N, V>(
        &self,
        key: S,
        context: HashMap<N, V>,
    ) -> String
    where
        S: Into<String>,
        N: Into<String>,
        V: Into<String>;
}

impl I18nExtension for I18n {
    fn try_translate_or_default<S: Into<String>>(&self, key: S) -> String {
        let key = key.into().replace("_", "-");
        self.try_translate(key.as_str()).ok().unwrap_or(key)
    }

    fn try_translate_with_context_or_default<S, N, V>(
        &self,
        key: S,
        context: HashMap<N, V>,
    ) -> String
    where
        S: Into<String>,
        N: Into<String>,
        V: Into<String>,
    {
        let key = key.into().replace("_", "-");
        let mut args = FluentArgs::new();
        context.into_iter().for_each(|(name, value)| {
            let name = name.into().replace("_", "-");
            let value = value.into().replace("_", "-");
            args.set(name, value);
        });
        self.try_translate_with_args(key.as_str(), Some(&args))
            .ok()
            .unwrap_or(key)
    }
}
