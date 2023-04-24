use std::{fmt::Error, env, num::ParseIntError};

pub struct Config {
    pub api_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Self, Error> {

        let api_port = env::var("PORT").unwrap();
        let db_user = env::var("POSTGRES_USER").unwrap();
        let db_password = env::var("POSTGRES_PASSWORD").unwrap();
        let db_host = env::var("POSTGRES_HOST").unwrap();
        let db_port = env::var("POSTGRES_PORT").unwrap();
        let db_name = env::var("POSTGRES_DB").unwrap();

        Ok(Config {
            api_port: api_port
                .parse::<u16>()
                .map_err(|e| -> Result<u16, ParseIntError> { Err(e) }).unwrap(),
            db_user,
            db_password,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(|e| -> Result<u16, ParseIntError> { Err(e) }).unwrap(),
            db_name,
        })
    }
}
