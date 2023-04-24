use sqlx::{PgPool, postgres::{PgPoolOptions, PgRow}, Row};

use crate::{models::account::{Account, AccountId}, custom_errors::store::Error};

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

    pub async fn get_account(&self, email: &str) -> Result<Account, Error> {
        match sqlx::query("SELECT * FROM public.accounts where email = $1;")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password")
            })
            .fetch_one(&self.db_pool)
            .await {
                Ok(account) => Ok(account),
                Err(error) => {
                    Err(Error::DatabaseQueryError(error))
                }
            }
    }
}