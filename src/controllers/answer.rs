use std::sync::Arc;

use warp::{reject::custom, reply::json, Future, Rejection, Reply};

use crate::{
    custom_errors::account,
    models::{
        account::Session,
        answer::{AnswerDTO, AnswerId},
    },
    repository::{database_repository::DatabaseRepository, Repository},
};

pub struct AnswerController {
    pub repository: Arc<Repository>,
}

impl AnswerController {
    pub fn new(store: Arc<Repository>) -> Self {
        Self { repository: store }
    }

    pub fn create_answer(
        &self,
        session: Session,
        answer: AnswerDTO,
    ) -> impl Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
        async move {
            let result = self
                .repository
                .create_answer(answer, session.account_id)
                .await;
            match result {
                Ok(answer) => Ok(json(&answer)),
                Err(e) => Err(custom(e)),
            }
        }
    }

    pub fn update_answer(
        &self,
        session: Session,
        answer_id: AnswerId,
        answer: AnswerDTO,
    ) -> impl Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
        async move {
            if self
                .repository
                .is_answer_owner(answer_id, session.account_id)
                .await?
            {
                let result = self.repository.update_answer(answer, answer_id).await;

                match result {
                    Ok(answer) => Ok(json(&answer)),
                    Err(e) => Err(custom(e)),
                }
            } else {
                Err(custom(account::Error::Unauthorized))
            }
        }
    }
}
