use std::sync::Arc;

use warp::{Filter, Reply, Rejection, path};

use crate::{store::Store, models::account::Account, controllers::authentication::AuthenticationController};

pub fn login_route(store: Arc<Store>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let registration_controller = AuthenticationController::new(store);
    warp::post()
        .and(path("login"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(move |account: Account| {
            let controller = registration_controller.clone();
            async move {
                controller.login(account).await
            }
        })
}

pub fn registration_route(store: Arc<Store>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let registration_controller = AuthenticationController::new(store);
    warp::post()
        .and(path("registration"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(move |account: Account| {
            let controller = registration_controller.clone();
            async move {
                controller.register_account(&account).await
            }
        })
}