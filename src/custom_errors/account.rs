use std::fmt::Display;
use warp::reject::Reject;
use argon2::Error as ArgonError;

#[derive(Debug)]
pub enum Error {
    WrongCredentials,
    ArgonLibraryError(ArgonError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ArgonLibraryError(_) => {
                write!(f, "Can't verify password")
            },
            Error::WrongCredentials => {
                write!(f, "Wrong Credentials")
            }
        }
    }
}

impl Reject for Error {}