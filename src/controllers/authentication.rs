use argon2::Config;
use chrono::Utc;
use paseto::PasetoBuilder;
use rand::Rng;
use std::{env, sync::Arc, future};
use warp::{
    header, hyper::StatusCode, reject::custom, reply::json, Filter, Future, Rejection, Reply,
};

use crate::{
    custom_errors::account::Error,
    models::account::{Account, AccountId, Session}, repository::Repository,
};

#[derive(Debug, Clone)]
pub struct AuthenticationController {
    repository: Arc<Repository>,
}

impl AuthenticationController {
    pub fn new(store: Arc<Repository>) -> Self {
        Self { repository: store }
    }

    // The Future type itself requires a lifetime parameter to be specified, which is used to specify the
    // lifetime of the returned Future.
    // Note that naming the lifetime parameter explicitly can make the code easier to read and understand,
    // but it is not necessary in this case because the anonymous lifetime '_ can be used to elide the
    // lifetime parameter and let the Rust compiler infer the lifetime automatically.
    pub fn register_account(
        &self,
        account: Account,
    ) -> impl warp::Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
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

    pub fn login(
        &self,
        login: Account,
    ) -> impl Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
        async move {
            match self.repository.get_account(&login.email).await {
                Ok(account) => {
                    match self.verify_password(&account.password, login.password.as_bytes()) {
                        Ok(verified) => {
                            if verified {
                                Ok(json(
                                    &self.issue_token(account.id.expect("Account Id not found")),
                                ))
                            } else {
                                Err(custom(Error::WrongCredentials))
                            }
                        }
                        Err(e) => Err(custom(Error::ArgonLibraryError(e))),
                    }
                }
                Err(e) => Err(custom(e)),
            }
        }
    }

    pub fn auth() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
        header::<String>("Authorization").and_then(|token: String| {
            match Self::verify_token(token) {
                Ok(t) => return future::ready(Ok(t)),
                Err(_) => return future::ready(Err(warp::reject::custom(Error::Unauthorized))),
            }
        })
    }

    fn hash_password(&self, password: &[u8]) -> String {
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        let config = Config::default();
        argon2::hash_encoded(password, &salt, &config).unwrap()
    }

    fn verify_password(&self, hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
        argon2::verify_encoded(hash, password)
    }

    fn issue_token(&self, account_id: AccountId) -> String {
        let key = env::var("PASETO_KEY").unwrap();
        let current_date_time = Utc::now();
        let dt = current_date_time + chrono::Duration::days(1);

        PasetoBuilder::new()
            .set_encryption_key(&Vec::from(key.as_bytes()))
            .set_expiration(&dt)
            .set_not_before(&Utc::now())
            .set_claim("account_id", serde_json::json!(account_id))
            .build()
            .expect("Failed to construct paseto token w/ builder!")
    }

    fn verify_token(token: String) -> Result<Session, Error> {
        let key = env::var("PASETO_KEY").expect("please define the PASETO_KEY env variable");
        let token = paseto::tokens::validate_local_token(
            &token,
            None,
            key.as_bytes(),
            &paseto::tokens::TimeBackend::Chrono,
        )
        .map_err(|_| crate::custom_errors::account::Error::CannotDecryptToken)?;

        serde_json::from_value::<Session>(token)
            .map_err(|_| crate::custom_errors::account::Error::CannotDecryptToken)
    }
}
