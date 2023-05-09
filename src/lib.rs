use std::sync::Arc;

use config::Config;
use controllers::{
    answer::AnswerController, authentication::AuthenticationController,
    question::QuestionController,
};
use custom_errors::{custom_error_recover::return_custom_error, repository::Error};
use repository::Repository;
use routes::{
    answer::{create_answer_route, update_answer_route},
    authentication::{login_route, registration_route},
    question::{
        add_question_route, delete_question_route, get_question_route, get_questions_route,
        update_question_route,
    },
};
use warp::{Filter, Reply};

mod models;
mod routes;
mod controllers {
    pub mod answer;
    pub mod authentication;
    pub mod question;
}

mod repository {
    pub mod database_repository;
    pub mod repository;
    pub use repository::Repository;
}

mod custom_errors;

pub mod config;

pub async fn run(config: &Config, repository: Arc<Repository>) {
    let routes = build_routes(Arc::clone(&repository)).await;

    warp::serve(routes)
        .run(([127, 0, 0, 1], config.api_port))
        .await;
}

pub async fn setup_repository(config: &Config) -> Result<Arc<Repository>, Error> {
    let repository = Repository::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
    ))
    .await
    .map_err(|e| Error::DatabaseQueryError(e))?;

    sqlx::migrate!()
        .run(
            &repository.clone().db_pool
        )
        .await
        .map_err(|e| Error::MigrationError(e))?;

    Ok(Arc::new(repository))
}

async fn build_routes(repository: Arc<Repository>) -> impl Filter<Extract = (impl Reply,)> + Clone {
    let cors = warp::cors().allow_any_origin().allow_methods(vec!["POST"]);

    let auth_controller = Arc::new(AuthenticationController::new(Arc::clone(&repository)));
    let question_controller = Arc::new(QuestionController::new(Arc::clone(&repository)));
    let answer_controller = Arc::new(AnswerController::new(Arc::clone(&repository)));

    login_route(Arc::clone(&auth_controller))
        .or(registration_route(Arc::clone(&auth_controller)))
        .or(add_question_route(Arc::clone(&question_controller)))
        .or(get_question_route(Arc::clone(&question_controller)))
        .or(get_questions_route(Arc::clone(&question_controller)))
        .or(update_question_route(Arc::clone(&question_controller)))
        .or(delete_question_route(Arc::clone(&question_controller)))
        .or(create_answer_route(Arc::clone(&answer_controller)))
        .or(update_answer_route(Arc::clone(&answer_controller)))
        .with(cors)
        .recover(return_custom_error)
}
