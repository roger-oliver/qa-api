use std::sync::Arc;

use warp::{body::json, path, Filter, Rejection, Reply};

use crate::{
    controllers::{answer::AnswerController, authentication::AuthenticationController},
    models::{account::Session, answer::{AnswerDTO, AnswerId}},
};

pub fn create_answer_route(
    answer_controller: Arc<AnswerController>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("answer"))
        .and(path::end())
        .and(AuthenticationController::auth())
        .and(json())
        .and_then(move |session: Session, answer: AnswerDTO| {
            let controller = answer_controller.clone();
            async move { controller.create_answer(session, answer).await }
        })
}

pub fn update_answer_route(
    answer_controller: Arc<AnswerController>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::put()
        .and(path("answer"))
        .and(path::param::<i32>())
        .and(path::end())
        .and(AuthenticationController::auth())
        .and(json())
        .and_then(move |id, session: Session, answer: AnswerDTO| {
            let controller = answer_controller.clone();
            async move { controller.update_answer(session, AnswerId(id), answer).await }
        })
}
