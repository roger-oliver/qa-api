use std::sync::Arc;

use argon2::Config;
use rand::Rng;
use warp::{Reply, Rejection, hyper::StatusCode};

use crate::{models::account::Account, store::Store};

#[derive(Debug, Clone)]
pub struct RegistrationController {
    repository: Arc<Store>,
}

// #[derive(Debug)]
// enum RegistrationError {
//     UserExists,
// }

// impl Reject for RegistrationError {}

impl RegistrationController {
    pub fn new(store: Arc<Store>) -> Self {
        Self { repository: store }
    }

    pub fn register_account(&self, account: Account) -> impl warp::Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
    
        let hashed_password = self.hash_password(account.password.as_bytes());
    
        let account = Account {
            id: account.id,
            email: account.email,
            password: hashed_password,
        };

        async move {
            match self.repository.add_account(account).await {
                Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        }
    }
    
    fn hash_password(&self, password: &[u8]) -> String {
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        let config = Config::default();
        argon2::hash_encoded(password, &salt, &config).unwrap()
    }
}
