mod character;
mod dialog_parser;
mod verbs;

use std::fmt::Display;

// TODO: expose API

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    UnknownVerbKey,
    UndefinedVerbCase,
    MissingMacroData,
    UnknownCharacterIdentifier,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Serde(serde_error) => Display::fmt(serde_error, f),
            Error::UnknownVerbKey => f.write_str("unknown verb key"),
            Error::UndefinedVerbCase => f.write_str("undefined verb case"),
            Error::MissingMacroData => f.write_str("macro misses data attribute"),
            Error::UnknownCharacterIdentifier => f.write_str("unknown character identifier"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Serde(serde_error) => Some(serde_error),
            _ => None,
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Self::Serde(source)
    }
}
