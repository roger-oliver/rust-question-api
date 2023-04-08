use std::env;

use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BadWord {
    pub original: String,
    pub word: String,
    pub deviations: i64,
    pub info: i64,
    #[serde(rename = "replacedLen")]
    pub replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BadWordsResponse {
    pub content: String,
    pub bad_words_total: i64,
    pub bad_words_list: Vec<BadWord>,
    pub censored_content: String,
}

pub async fn check_profanity(content: String) -> Result<String, handle_errors::Error> {
    // We are already checking if the ENV VARIABLE is set inside main.rs, 
    // so safe to unwrap here
    let api_key = env::var("BAD_WORDS_API_KEY").expect("API KEY NOT SET");
    let api_service_url = env::var("API_SERVICE_URL").expect("API SERVICE URL NOT SET");

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let res = client
        .post(format!("{}/bad_words?censor_character=*", api_service_url))
        .header("apikey", api_key)
        .body(content)
        .send()
        .await
        .map_err(|e| handle_errors::Error::MiddlewareReqwestApiError(e))?;

    if !res.status().is_success() {
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            return Err(handle_errors::Error::ClientError(err));
        } else {
            let err = transform_error(res).await;
            return Err(handle_errors::Error::ServerError(err));
        }
    }

    match res.json::<BadWordsResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(e) => Err(handle_errors::Error::ReqwestApiError(e)),
    }
}

async fn transform_error(res: reqwest::Response) -> handle_errors::ApiLayerError {
    handle_errors::ApiLayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}

#[cfg(test)]
mod profanity_tests {
    use super::{check_profanity, env};

    use mock_server::{MockServer, OneshotHandler};

    #[tokio::test]
    async fn run() {
        let handler = run_mock();
        censor_profane_words().await;
        no_profane_words().await;
        let _ = handler.sender.send(1);
    }

    fn run_mock() -> OneshotHandler {
        env::set_var("API_SERVICE_URL", "http://localhost:8081");
        env::set_var("BAD_WORDS_API_KEY", "YES");

        let socket = "127.0.0.1:8081"
            .to_string()
            .parse()
            .expect("Not a valid address");
        let mock = MockServer::new(socket);

        mock.oneshot()
    }

    async fn censor_profane_words() {
        let content = "quite a dick!".to_string();
        let censored_content = check_profanity(content).await;
        assert_eq!(censored_content.unwrap(), "quite a ****!");
    }

    async fn no_profane_words() {
        let content = "some sentence".to_string();
        let censored_content = check_profanity(content).await;
        assert_eq!(censored_content.unwrap(), "");
    }
}
