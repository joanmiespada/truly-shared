#[derive(Debug)]
pub struct AsyncOpError(pub String);

impl std::error::Error for AsyncOpError {}

impl std::fmt::Display for AsyncOpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "async work operation wrong: {}", self.0)
    }
}
