#[derive(PartialEq, Clone, Copy)]
pub enum RuntimeError {
    ErrorSocketConnect,
    ErrorSocketRead,
    ErrorSocketWrite,
    ErrorExpectedNumber,
    ErrorEnvironmentVariableNotSet,
    ErrorLyricNotFound,
    None,
}
