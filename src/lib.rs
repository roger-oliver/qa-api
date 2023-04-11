use routes::authentication::{login_route, registration_route};
use warp::Filter;

mod routes;
mod models;
mod controllers;

pub async fn run() {

    let routes = login_route()
        .or(registration_route());
    
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST"]);
    
    warp::serve(routes.with(cors))
        .run(([127, 0, 0, 1], 8081))
        .await;
}   