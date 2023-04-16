use routes::authentication::{login_route, registration_route};
use store::Store;
use warp::{Filter, Reply};

mod routes;
mod models;
mod controllers;
mod store;
mod custom_errors;

pub async fn run(store: Store) {

    let routes = build_routes(store).await;
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8081))
        .await;
}

pub async fn setup_store() -> Store {
    let store = Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}", "postgres", "postgres", "localhost", 5432, "qa_api_db"))
    .await
    .unwrap();

    sqlx::migrate!()
        .run(&store.clone().db_pool)
        .await.unwrap();

    return store;
}

async fn build_routes(store: Store) -> impl Filter<Extract = (impl Reply,)> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST"]);

    login_route()
    .or(registration_route(store.clone()))
    .with(cors)
}
