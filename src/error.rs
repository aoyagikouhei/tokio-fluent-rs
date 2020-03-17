#[derive(Debug)]
pub enum FluentError {
    IOError(std::io::Error),
    JSON(serde_json::error::Error),
}

impl From<std::io::Error> for FluentError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<serde_json::error::Error> for FluentError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::JSON(err)
    }
}