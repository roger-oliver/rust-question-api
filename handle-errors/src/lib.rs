use warp::{
    body::BodyDeserializeError, cors::CorsForbidden, hyper::StatusCode, reject::Reject,
    reply::with_status, Rejection, Reply,
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "cannot parse parameter: {}", err)
            }
            Error::MissingParameters => {
                write!(f, "missing parameter")
            }
            Error::QuestionNotFound => {
                write!(f, "question not found")
            }
        }
    }
}

impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(with_status(error.to_string(), StatusCode::FORBIDDEN))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(with_status(
            "Route not Found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
