use async_trait::async_trait;

use crate::{
    custom_errors::repository::Error,
    models::{
        account::{Account, AccountId},
        answer::{Answer, AnswerDTO, AnswerId},
        question::{Question, QuestionDTO, QuestionId},
    },
};

#[async_trait]
pub trait DatabaseRepository {
    async fn add_account(&self, account: Account) -> Result<bool, Error>;
    async fn get_account(&self, email: &str) -> Result<Account, Error>;

    async fn create_question(
        &self,
        question: QuestionDTO,
        account_id: AccountId,
    ) -> Result<Question, Error>;

    async fn get_question(&self, question_id: QuestionId) -> Result<Question, Error>;
    async fn get_questions(&self, limit: Option<i16>, offset: i16) -> Result<Vec<Question>, Error>;

    async fn update_question(
        &self,
        question: QuestionDTO,
        question_id: QuestionId,
    ) -> Result<Question, Error>;

    async fn delete_question(&self, question_id: QuestionId) -> Result<bool, Error>;
    async fn is_question_owner(&self, question_id: QuestionId, account_id: AccountId) -> Result<bool, Error>;

    async fn is_answer_owner(&self, answer_id: AnswerId, account_id: AccountId) -> Result<bool, Error>;
    async fn create_answer(&self, answer: AnswerDTO, account_id: AccountId) -> Result<Answer, Error>;
    async fn update_answer(&self, answer: AnswerDTO, answer_id: AnswerId) -> Result<Answer, Error>;
}