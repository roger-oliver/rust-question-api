use std::future;

use crate::{store::Store, types::account::{Account, AccountId, Session}};
use argon2::Config;
use chrono::Utc;
use handle_errors::Error;
use rand::Rng;
use warp::{http::StatusCode, Filter};

pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };
    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountId) -> String {
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);
 
    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(
            &Vec::from("RANDOM WORDS WINTER MACINTOSH PC".as_bytes()
            ))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

pub fn verify_token(token: String) -> Result<Session, Error> {
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
    &"RANDOM WORDS WINTER MACINTOSH PC".as_bytes(),
    &paseto::tokens::TimeBackend::Chrono,
  )
    .map_err(|_| Error::CannotDecryptToken)?;
 
    serde_json::from_value::<Session>(token).map_err(|_| {  
        Error::CannotDecryptToken
    })
}

pub fn auth() ->
    impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization")
        .and_then(|token: String| {
            match verify_token(token) {
                Ok(t) => return future::ready(Ok(t)),
                Err(_) => return future::ready(Err(warp::reject::custom(Error::Unauthorized))),
            };
        })
}