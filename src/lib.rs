use std::sync::Arc;

use config::Config;
use routes::authentication::{login_route, registration_route};
use store::Store;
use warp::{Filter, Reply};

mod routes;
mod models;
mod controllers;
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

    login_route()
    .or(registration_route(Arc::clone(&store)))
    .with(cors)
}
