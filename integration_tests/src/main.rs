use std::{process::Command, io::{stdout, Write}};

use futures_util::FutureExt;
use qa_api::{config, oneshot};
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserDTO {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct QuestionDTO {
    title: String,
    content: String,
    tags: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

struct AnswerDTO {
    content: String,
    question_id: i32
}

#[derive(Debug, Deserialize, Clone)]
struct Token(String);

#[tokio::main]
async fn main() -> Result<(), warp::Rejection> {
    // need to call the env variables
    dotenv::dotenv().ok();
    // set the configuration
    let config = config::Config::new().expect("Config can't be set");

    println!("{}", format!("postgres://{}:{}@{}:{}/{}", config.db_user, config.db_password, config.db_host, config.db_port, config.db_name));

    // to start integration test, we need to remove any db test instance
    let s = Command::new("sqlx")
        .arg("database")
        .arg("drop")
        .arg("--database-url")
        .arg(format!("postgres://{}:{}@{}:{}/{}", config.db_user, config.db_password, config.db_host, config.db_port, config.db_name))
        .arg("-y")
        .output()
        .expect("sqlx command failed to start");

    stdout().write_all(&s.stderr).unwrap();

    // now, we need to create a database to start our tests
    let s = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(format!("postgres://{}:{}@{}:{}/{}", config.db_user, config.db_password, config.db_host, config.db_port, config.db_name))
        .output()
        .expect("sqlx command failed to start");

    stdout().write_all(&s.stderr).unwrap();

    let repository = qa_api::setup_repository(&config).await?;

    let handler = oneshot(repository).await;

    let token: Token;
    let question_returned: Question;

    // create a test user to use throughout the tests
    let user = UserDTO {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
    };

    print!("Running register new user!!!");

    let result = std::panic::AssertUnwindSafe(register_new_user(&user)).catch_unwind().await;
    match result {
        Ok(_) => println!(" ✓"),
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running user login!!!");
    let result = std::panic::AssertUnwindSafe(login_user(&user)).catch_unwind().await;

    match result {
        Ok(t) => {
            token = t;
            println!(" ✓");
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    print!("Running add question!!!");
    let question = QuestionDTO {
        title: "New question".to_string(),
        content: "Question content".to_string(),
        tags: Some(vec!["general".to_string()])
    };
    let result = std::panic::AssertUnwindSafe(create_question(&token, &question)).catch_unwind().await;

    match result {
        Ok(q) => {
            question_returned = q;
            println!(" ✓");
        },
        Err(_) => {
            let _ = handler.sender.send(1);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn register_new_user(user: &UserDTO) {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8080/registration")
        .json(&user)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    assert_eq!(res.unwrap(), "Account added".to_string());
}

async fn login_user(user: &UserDTO) -> Token {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8080/login")
        .json(&user)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 200);

    res.json::<Token>()
        .await
        .unwrap()
}

async fn create_question(token: &Token, question: &QuestionDTO) -> Question {
    let client = Client::new();
    let res = client.post("http://localhost:8080/question")
        .header("Authorization", token.0.clone())
        .json(&question)
        .send()
        .await
        .unwrap()
        .json::<Question>()
        .await
        .unwrap();

    assert_eq!(res.id, 1);
    assert_eq!(res.content, question.content);

    res
}
