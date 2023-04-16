use warp::{Filter, Reply, Rejection, path};

use crate::{controllers::authentication::{login, register_account}, store::Store};

pub fn login_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("login"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(login)
}

pub fn registration_route(store: Store) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("registration"))
        .and(path::end())
        .and(warp::any().map(move || store.clone()))
        .and(warp::body::json())
        .and_then(register_account)
}