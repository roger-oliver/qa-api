use std::collections::HashMap;

use async_trait::async_trait;
use qa_api::{
    custom_errors::account::Error,
    models::{
        account::{Account, AccountId},
        answer::{Answer, AnswerId},
        question::{Question, QuestionDTO, QuestionId},
    },
    repository::database_repository::DatabaseRepository,
};

pub struct InMemoryRepository {
    accounts: HashMap<String, Account>, // Map email to account
    questions: HashMap<QuestionId, (Question, AccountId)>, // Map question id to (question, account id)
    answers: HashMap<AnswerId, (Answer, AccountId)>,       // Map answer id to (answer, account id)
}

#[async_trait]
impl DatabaseRepository for InMemoryRepository {
    async fn add_account(&self, account: Account) -> Result<bool, Error> {
        let email = account.email.clone();
        if self.accounts.contains_key(&email) {
            return Ok(false);
        }
        self.accounts.insert(email, account);
        Ok(true)
    }

    async fn get_account(&self, email: &str) -> Result<Account, Error> {
        match self.accounts.get(email) {
            Some(account) => Ok(account.clone()),
            None => Err(Error::NotFound),
        }
    }

    async fn create_question(
        &self,
        question: QuestionDTO,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        let question_id = QuestionId::generate();
        let question = Question::new(question_id, question.title, question.body);
        self.questions
            .insert(question_id, (question.clone(), account_id));
        Ok(question)
    }
}
