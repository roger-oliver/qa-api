use serde::{Serialize, Deserialize};

use super::question::QuestionId;

#[derive(Debug, Serialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct AnswerId(pub i32);

#[derive(Debug, Deserialize)]
pub struct AnswerDTO {
    pub content: String,
    pub question_id: QuestionId,
}