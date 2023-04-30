use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::{
    custom_errors::repository::Error,
    models::{
        account::{Account, AccountId},
        answer::{Answer, AnswerDTO, AnswerId},
        question::{NewQuestion, Question, QuestionId},
    },
};

#[derive(Debug, Clone)]
pub struct Repository {
    pub db_pool: PgPool,
}

impl Repository {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(db_url)
            .expect("Failed to connect to posgresql");

        Ok(Repository { db_pool })
    }

    pub async fn add_account(&self, account: Account) -> Result<bool, Error> {
        match sqlx::query(
            "INSERT INTO accounts (email, password) 
            VALUES ($1, $2)  returning id, email",
        )
        .bind(account.email)
        .bind(account.password)
        .execute(&self.db_pool)
        .await
        {
            Ok(_) => Ok(true),
            Err(error) => Err(Error::DatabaseQueryError(error)),
        }
    }

    pub async fn get_account(&self, email: &str) -> Result<Account, Error> {
        match sqlx::query("SELECT * FROM public.accounts where email = $1;")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(account) => Ok(account),
            Err(error) => Err(Error::DatabaseQueryError(error)),
        }
    }

    pub async fn create_question(
        &self,
        question: NewQuestion,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO public.questions
                             (title, content, tags, account_id)
                      VALUES ($1, $2, $3, $4)
                      RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(account_id.0)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.db_pool)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn get_question(&self, question_id: QuestionId) -> Result<Question, Error> {
        let result = sqlx::query(
            "SELECT id, title, content, tags, account_id, created_on
                   FROM public.questions
                  WHERE id = $1",
        )
        .bind(question_id.0)
        .map(|row: PgRow| Question {
            id: question_id,
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.db_pool)
        .await;
        match result {
            Ok(question) => Ok(question),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<i16>,
        offset: i16,
    ) -> Result<Vec<Question>, Error> {
        let result = sqlx::query(
            "SELECT id, title, content, tags, account_id, created_on
                   FROM public.questions
                  ORDER BY created_on desc
                  LIMIT $1
                 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_all(&self.db_pool)
        .await;

        match result {
            Ok(questions) => Ok(questions),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn update_question(
        &self,
        question: NewQuestion,
        question_id: QuestionId,
    ) -> Result<Question, Error> {
        // the update can only be applied when the user is the entry's owner.
        // checked by "is_question_owner"
        let result = sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3
        WHERE id = $4
        RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id.0)
        .map(|row: PgRow| Question {
            id: question_id,
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.db_pool)
        .await;

        match result {
            Ok(question) => Ok(question),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn delete_question(&self, question_id: QuestionId) -> Result<bool, Error> {
        // the update can only be applied when the user is the entry's owner.
        // checked by "is_question_owner"
        let result = sqlx::query(
            "DELETE FROM public.questions
            WHERE id = $1",
        )
        .bind(question_id.0)
        .execute(&self.db_pool)
        .await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn is_question_owner(
        &self,
        question_id: QuestionId,
        account_id: AccountId,
    ) -> Result<bool, Error> {
        let result =
            sqlx::query("SELECT * FROM public.questions where id = $1 and account_id = $2")
                .bind(question_id.0)
                .bind(account_id.0)
                .fetch_optional(&self.db_pool)
                .await;

        match result {
            Ok(question) => Ok(question.is_some()),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn is_answer_owner(&self, answer_id: AnswerId, account_id: AccountId) -> Result<bool, Error> {
        let result = sqlx::query("select * from public.answers where id = $1 and account_id = $2")
            .bind(answer_id.0)
            .bind(account_id.0)
            .fetch_optional(&self.db_pool)
            .await;

        match result {
            Ok(answer) => Ok(answer.is_some()),
            Err(e) => Err(Error::DatabaseQueryError(e))
        }
    }

    pub async fn create_answer(
        &self,
        answer: AnswerDTO,
        account_id: AccountId,
    ) -> Result<Answer, Error> {
        let result = sqlx::query("INSERT INTO public.answers
            (content, account_id, question_id)
            VALUES($1, $2, $3)
            returning id, content, account_id, question_id;")
            .bind(answer.content)
            .bind(account_id.0)
            .bind(answer.question_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id"))
            })
            .fetch_one(&self.db_pool)
            .await;

        match result {
            Ok(answer) => Ok(answer),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn update_answer(&self, answer: AnswerDTO, answer_id: AnswerId) -> Result<Answer, Error> {
        let result = sqlx::query("UPDATE public.answers
            SET content=$1
            WHERE id=$2
            returning id, content, account_id, question_id")
            .bind(answer.content)
            .bind(answer_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id"))
            })
            .fetch_one(&self.db_pool)
            .await;

        match result {
            Ok(answer) => Ok(answer),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }
}
