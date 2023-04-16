use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{models::account::Account, custom_errors::store::Error};

#[derive(Debug, Clone)]
pub struct Store {
    pub db_pool: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(db_url)
            .expect("Failed to connect to posgresql");

        Ok(Store { db_pool })
    }

    pub async fn add_account(&self, account: Account) -> Result<bool, Error> {
        match sqlx::query("INSERT INTO accounts (email, password) 
            VALUES ($1, $2)  returning id, email",)
            .bind(account.email)
            .bind(account.password)
            .execute(&self.db_pool)
            .await {
                Ok(_) => Ok(true),
                Err(error) => {
                    Err(Error::DatabaseQueryError(error))
                }
            }
    }
}