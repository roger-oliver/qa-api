use std::fmt::Display;

use warp::{Reply, Rejection, reply, hyper::StatusCode, reject::{self, Reject}};

use crate::models::account::Account;

#[derive(Debug)]
enum AccountError {
    WrongCredentials,
}

impl Reject for AccountError {}

impl Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            AccountError::WrongCredentials => write!(f, "Wrong Credentials"),
        }
    }
}

pub async fn login(account: Account) -> Result<impl Reply, Rejection> {
    if account.email == "admin" && account.password == "password" {
        Ok(reply::with_status("you're logged", StatusCode::OK))
    } else {
        Err(reject::custom(AccountError::WrongCredentials))
    }
}
