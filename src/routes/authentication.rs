use std::sync::Arc;

use warp::{path, Filter, Rejection, Reply};

use crate::{controllers::authentication::AuthenticationController, models::account::Account};

pub fn login_route(
    auth_controller: Arc<AuthenticationController>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("login"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(move |account: Account| {
            let controller = auth_controller.clone();
            async move { controller.login(account).await }
        })
}

pub fn registration_route(
    auth_controller: Arc<AuthenticationController>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("registration"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(move |account: Account| {
            let controller = auth_controller.clone();
            async move { controller.register_account(account).await }
        })
}
