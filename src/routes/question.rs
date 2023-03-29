#![warn(clippy::all)]

use std::collections::HashMap;

use warp::{hyper::StatusCode, Rejection, Reply};

use handle_errors::Error;

use crate::{
    store::Store,
    types::{
        pagination::extract_pagination,
        question::{Question, QuestionId},
    },
};

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

// pay attention!!! the signature need to follow this order!!! param, store, item to be updated
pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("question updated", StatusCode::OK))
}

pub async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
