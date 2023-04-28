use std::fmt::Display;
use warp::reject::Reject;
use argon2::Error as ArgonError;

#[derive(Debug)]
pub enum Error {
    WrongCredentials,
    ArgonLibraryError(ArgonError),
    CannotDecryptToken,
    Unauthorized,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ArgonLibraryError(_) => {
                write!(f, "Can't verify password")
            },
            Error::WrongCredentials => {
                write!(f, "Wrong Credentials")
            },
            Error::CannotDecryptToken => {
                write!(f, "Not possible to decrypt informed token")
            },
            Error::Unauthorized => {
                write!(f, "you are not authorized to use this resource")
            }
        }
    }
}

impl Reject for Error {}