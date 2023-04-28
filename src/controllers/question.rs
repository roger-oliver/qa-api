use std::{collections::HashMap, sync::Arc};

use warp::{reject::custom, reply::json, Future, Rejection, Reply};

use crate::{
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
}
