use warp::{Filter, Reply, Rejection, path};

use crate::controllers::authentication::{login, registration};

pub fn login_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("login"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(login)
}

pub fn registration_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("registration"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(registration)
}