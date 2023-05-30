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
struct Question {
    id: i32,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct AnswerDTO {
    content: String,
    question_id: i32
}

#[derive(Debug, Deserialize)]
struct Answer {
    id: i32,
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
    let _answer_returned: Answer;

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

    print!("Running add answer!!!");
    let answer = AnswerDTO {
        content: "Answer content".to_string(),
        question_id: question_returned.id
    };

    let result = std::panic::AssertUnwindSafe(
        create_answer(
            &token, 
            &answer
        )
    ).catch_unwind().await;

    match result {
        Ok(a) => {
            _answer_returned = a;
            println!(" ✓")
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
    let res: Question = post_entity(token, &question, "http://localhost:8080/question").await;
    assert_eq!(res.id, 1);
    assert_eq!(res.content, question.content);

    res
}

async fn create_answer(token: &Token, answer: &AnswerDTO) -> Answer {
    let res: Answer = post_entity(token, &answer, "http://localhost:8080/answer").await;
    assert_eq!(res.id, 1);
    assert_eq!(res.content, answer.content);
    assert_eq!(res.question_id, answer.question_id);

    res
}

// below, we use Higher-Ranked Trait Bounds (HRTB) for S generic types, but why?
// This is used when the trait implementation must be valid for any lifetime 'de,
// not just a specific one. In this case, it means that S must implement the 
// Deserialize trait for any lifetime, not just a specific lifetime 'a.
// When S: for<'de> Deserialize<'de> is used, it means S must be a type that can 
// be deserialized from any possible lifetime 'de. This is a requirement for the
// json method from the reqwest library, because it doesn't know the exact 
// lifetime of the data it will be receiving from the network, so it requires a 
// type that can handle any possible lifetime.
async fn post_entity<'a, T, S>(token: &'a Token, entity: &'a T, action_url: &'a str) -> S
    where T: Serialize + 'a, S: for<'b> Deserialize<'b> {
    let client = Client::new();
    let response = client.post(action_url)
        .header("Authorization", token.0.clone())
        .json(&entity)
        .send()
        .await
        .unwrap()
        .json::<S>()
        .await
        .unwrap();

    response
}