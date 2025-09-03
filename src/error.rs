#[derive(PartialEq)]
pub enum RuntimeError {
    ErrorSocketConnect,
    ErrorSocketRead,
    ErrorSocketWrite,
    ErrorExpectedNumber,
    ErrorEnvironmentVariableNotSet,
    ErrorFileNotFound,
}
