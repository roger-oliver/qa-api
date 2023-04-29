use std::{collections::HashMap, sync::Arc};

use warp::{
    hyper::StatusCode,
    reject::custom,
    reply::{json, with_status},
    Future, Rejection, Reply,
};

use crate::{
    custom_errors::account,
    models::{
        account::Session,
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, QuestionId},
    },
    store::Store,
};

pub struct QuestionController {
    pub repository: Arc<Store>,
}

impl QuestionController {
    pub fn new(store: Arc<Store>) -> Self {
        Self { repository: store }
    }

    pub fn create_question(
        &self,
        session: Session,
        new_question: NewQuestion,
    ) -> impl Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
        async move {
            match self
                .repository
                .create_question(new_question, session.account_id)
                .await
            {
                Ok(question) => Ok(json(&question)),
                Err(e) => Err(custom(e)),
            }
        }
    }

    pub fn get_question(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
        async move {
            let result = self.repository.get_question(QuestionId(id)).await;
            match result {
                Ok(question) => Ok(json(&question)),
                Err(e) => Err(custom(e)),
            }
        }
    }

    pub fn get_questions(
        &self,
        params: HashMap<String, String>,
    ) -> impl Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
        async move {
            let mut pagination = Pagination::default();

            if !params.is_empty() {
                pagination = extract_pagination(params)?;
            }

            let result = self
                .repository
                .get_questions(pagination.limit, pagination.offset)
                .await;
            match result {
                Ok(questions) => Ok(json(&questions)),
                Err(e) => Err(custom(e)),
            }
        }
    }

    pub fn update_question(
        &self,
        session: Session,
        question: NewQuestion,
        question_id: QuestionId,
    ) -> impl Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
        async move {
            if self
                .repository
                .is_question_owner(question_id, session.account_id)
                .await?
            {
                let result = self.repository.update_question(question, question_id).await;
                match result {
                    Ok(question) => Ok(json(&question)),
                    Err(e) => Err(custom(e)),
                }
            } else {
                Err(custom(account::Error::Unauthorized))
            }
        }
    }

    pub fn delete_question(
        &self,
        session: Session,
        question_id: QuestionId,
    ) -> impl Future<Output = Result<impl Reply, Rejection>> + Send + '_ {
        async move {
            if self.repository.is_question_owner(question_id, session.account_id).await? {
                let result = self.repository.delete_question(question_id).await;
                match result {
                    Ok(_) => Ok(with_status(
                        format!("question id: {} deleted", question_id.0),
                        StatusCode::OK
                    )),
                    Err(e) => Err(custom(e))
                }
            } else {
                Err(custom(account::Error::Unauthorized))
            }
        }
    }

}
