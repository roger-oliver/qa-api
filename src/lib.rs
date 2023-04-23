use std::sync::Arc;

use routes::authentication::{login_route, registration_route};
use store::Store;
use warp::{Filter, Reply};

mod routes;
mod models;
mod controllers;
mod store;
mod custom_errors;

pub async fn run(store: Arc<Store>) {

    let routes = build_routes(Arc::clone(&store)).await;
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8081))
        .await;
}

pub async fn setup_store() -> Arc<Store> {
    let store = Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}", "postgres", "postgres", "localhost", 5432, "qa_api_db"))
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
