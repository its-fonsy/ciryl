use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    LyricDirEnvNotSet,
    LyricNotFound,
    GuiError(String),
    ParseError(String),
    EnvVarError(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "error non yet implemented"),
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        Self::GuiError(error.to_string())
    }
}

impl From<std::num::ParseIntError> for RuntimeError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseError(error.to_string())
    }
}

impl From<std::env::VarError> for RuntimeError {
    fn from(error: std::env::VarError) -> Self {
        Self::EnvVarError(error.to_string())
    }
}

impl Error for RuntimeError {}
