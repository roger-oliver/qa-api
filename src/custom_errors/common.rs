use std::fmt::Display;

use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    MissingParameters,
    ParseError(std::num::ParseIntError),
}

impl Reject for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::MissingParameters => write!(f, "missing parameter on query string"),
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
        }
    }
}