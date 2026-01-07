use std::backtrace::Backtrace;

#[derive(thiserror::Error, Debug)]
pub enum CadetHubError {
    #[error("{reason}")]
    CommonError { reason: String },

    #[error("backend error occurred: {}",
        context.as_ref().map(|context| format!("context={}", context)).unwrap_or_default(),
    )]
    GeneralError {
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },
}

impl CadetHubError {
    pub fn pretty_debug_str(&self) -> String {
        match self {
            Self::GeneralError {
                source, backtrace, ..
            } => {
                let mut debug_string = format!("{}", self);
                if let Some(source) = source {
                    debug_string.push_str(&format!("\n{:?}", source));
                }
                if let Some(backtrace) = backtrace {
                    debug_string.push_str(&format!("\n{}", backtrace));
                }
                debug_string
            }
            _ => format!("{}", self),
        }
    }

    pub fn general_error<S, E>(source: Option<E>, context: Option<S>, trace: bool) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
        S: Into<String>,
    {
        let backtrace = if trace {
            Some(Backtrace::capture())
        } else {
            None
        };
        Self::GeneralError {
            source: source.map(|source| source.into()),
            context: context.map(|context| context.into()),
            backtrace,
        }
    }

    pub fn general_error_with_source<E>(source: E) -> CadetHubError
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        CadetHubError::general_error(Some(source), None::<String>, true)
    }

    pub fn general_error_with_context<S>(context: S) -> CadetHubError
    where
        S: Into<String>,
    {
        CadetHubError::general_error(None::<Self>, Some(context.into()), false)
    }
}
