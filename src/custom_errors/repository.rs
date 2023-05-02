use std::fmt::Display;

use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    DatabaseQueryError(sqlx::Error),
}

impl Reject for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::DatabaseQueryError(err) => write!(f, "Database query error: {}", err),
        }
    }
}