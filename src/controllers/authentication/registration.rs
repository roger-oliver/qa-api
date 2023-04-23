use std::sync::Arc;

use argon2::Config;
use rand::Rng;
use warp::{Reply, Rejection, hyper::StatusCode};

use crate::{models::account::Account, store::Store};

#[derive(Debug, Clone)]
pub struct RegistrationController {
    repository: Arc<Store>,
}

impl RegistrationController {
    pub fn new(store: Arc<Store>) -> Self {
        Self { repository: store }
    }

    // The Future type itself requires a lifetime parameter to be specified, which is used to specify the
    // lifetime of the returned Future.
    // Note that naming the lifetime parameter explicitly can make the code easier to read and understand,
    // but it is not necessary in this case because the anonymous lifetime '_ can be used to elide the 
    // lifetime parameter and let the Rust compiler infer the lifetime automatically.
    pub fn register_account(&self, account: &Account)
        -> impl warp::Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
    
        let hashed_password = self.hash_password(account.password.as_bytes());
    
        let account = Account {
            id: account.id.to_owned(),
            email: account.email.to_owned(),
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
