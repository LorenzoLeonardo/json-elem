#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::SerdeJson(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}
