use std::collections::HashMap;

use warp::hyper::StatusCode;

use crate::{
    store::Store,
    types::{
        answer::{Answer, AnswerId},
        question::QuestionId,
    },
};

pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        id: AnswerId(params.get("id").unwrap().to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("questionId").unwrap().to_string()),
    };

    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
