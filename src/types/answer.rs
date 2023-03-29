use serde::{Deserialize, Serialize};

use super::question::QuestionId;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct AnswerId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}
