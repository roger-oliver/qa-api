use std::sync::Arc;

use warp::{Filter, Reply, Rejection, path};

use crate::{controllers::authentication::{login, RegistrationController}, store::Store, models::account::Account};

pub fn login_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("login"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(login)
}

pub fn registration_route(store: Arc<Store>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let registration_controller = RegistrationController::new(store);
    warp::post()
        .and(path("registration"))
        .and(path::end())
        .and(warp::body::json())
        .and_then(move |account: Account| {
            let controller = registration_controller.clone();
            async move {
                controller.register_account(account.clone()).await
            }
        })
}