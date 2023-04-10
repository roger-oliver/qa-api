use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub password: String,
}

impl Account {
    pub fn new(email: String, password: String) -> Self {
        Account { email, password }
    }
}