use std::sync::Arc;

use warp::{Filter, Reply, Rejection, path};

use crate::{controllers::{question::QuestionController, authentication::AuthenticationController}, models::{question::NewQuestion, account::Session}};

pub fn add_question_route(question_controller: Arc<QuestionController>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::post()
        .and(path("question"))
        .and(path::end())
        .and(AuthenticationController::auth())
        .and(warp::body::json())
        .and_then(move |session: Session, new_question: NewQuestion| {
            let controller = question_controller.clone();
            async move { controller.create_question(session, new_question).await }
        })
}