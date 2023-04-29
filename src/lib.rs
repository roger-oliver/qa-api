use std::sync::Arc;

use config::Config;
use controllers::{authentication::AuthenticationController, question::QuestionController};
use custom_errors::custom_error_recover::return_custom_error;
use routes::{authentication::{login_route, registration_route}, question::{add_question_route, get_question_route, get_questions_route, update_question_route, delete_question_route}};
use store::Store;
use warp::{Filter, Reply};

mod routes;
mod models;
mod controllers {
    pub mod authentication;
    pub mod question;
}
mod store;
mod custom_errors;

pub mod config;

pub async fn run(config: &Config, store: Arc<Store>) {

    let routes = build_routes(Arc::clone(&store)).await;
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], config.api_port))
        .await;
}

pub async fn setup_store(config: &Config) -> Arc<Store> {
    let store = Store::new(&format!(
            "postgres://{}:{}@{}:{}/{}",
            config.db_user,
            config.db_password,
            config.db_host,
            config.db_port,
            config.db_name))
    .await
    .unwrap();

    sqlx::migrate!()
        .run(&store.clone().db_pool)
        .await.unwrap();

    return Arc::new(store);
}

async fn build_routes(store: Arc<Store>) -> impl Filter<Extract = (impl Reply,)> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST"]);

    let auth_controller = Arc::new(AuthenticationController::new(Arc::clone(&store)));
    let question_controller = Arc::new(QuestionController::new(Arc::clone(&store)));

    login_route(Arc::clone(&auth_controller))
    .or(registration_route(Arc::clone(&auth_controller)))
    .or(add_question_route(Arc::clone(&question_controller)))
    .or(get_question_route(Arc::clone(&question_controller)))
    .or(get_questions_route(Arc::clone(&question_controller)))
    .or(update_question_route(Arc::clone(&question_controller)))
    .or(delete_question_route(Arc::clone(&question_controller)))
    .with(cors)
    .recover(return_custom_error)
}
