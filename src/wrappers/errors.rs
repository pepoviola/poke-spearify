#[derive(thiserror::Error, Debug, PartialEq)]
pub enum WrapperError {
    #[error("Not Found")]
    NotFound,

    #[error("Pokemon without en description")]
    NoDescription,

    #[error("Parsing error")]
    ParsingError,

    #[error("Too many requests")]
    TooManyRequests,

    #[error("Unexpected API Error")]
    UnexpectedError,
}
