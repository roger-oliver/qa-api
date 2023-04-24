use std::fmt::Display;

use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    WrongPassword,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::WrongPassword => {
                write!(f, "Wrong Password")
            }
        }
    }
}

impl Reject for Error {}