use super::question::QuestionId;

pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

pub struct AnswerId(pub i32);

pub struct AnswerDTO {
    pub content: String,
    pub question_id: QuestionId,
}