use argon2::Config;
use rand::Rng;
use warp::{Reply, Rejection, hyper::StatusCode};

use crate::{models::account::Account, store::Store};

// #[derive(Debug)]
// enum RegistrationError {
//     UserExists,
// }

// impl Reject for RegistrationError {}

pub async fn register_account(store: Store, account: Account) -> Result<impl Reply, Rejection> {

    let hashed_password = hash_password(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}