use std::sync::Arc;

use warp::{path, Filter, Rejection, Reply, query};

use crate::{
    controllers::{authentication::AuthenticationController, question::QuestionController},
    models::{account::Session, question::NewQuestion},
};

pub fn add_question_route(
    question_controller: Arc<QuestionController>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
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

pub fn get_question_route(
    question_controller: Arc<QuestionController>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(path("question"))
        .and(path::param::<i32>())
        .and(path::end())
        .and_then(move |id| {
            let controller = question_controller.clone();
            async move { controller.get_question(id).await }
        })
}

pub fn get_questions_route(
    question_controller: Arc<QuestionController>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get()
        .and(path("questions"))
        .and(path::end())
        .and(query())
        .and_then(move |params| {
            let controller = question_controller.clone();
            async move { controller.get_questions(params).await }
        })
}
