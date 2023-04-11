use warp::{Reply, Rejection, reject::Reject, reply, hyper::StatusCode};

use crate::models::account::Account;

#[derive(Debug)]
enum RegistrationError {
    UserExists,
}

impl Reject for RegistrationError {}

pub async fn registration(account: Account) -> Result<impl Reply, Rejection> {
    if account.email == "admin" {
        Err(warp::reject::custom(RegistrationError::UserExists))
    } else {
        Ok(reply::with_status("user created", StatusCode::CREATED))
    }
}