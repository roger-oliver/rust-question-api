use clap::Parser;
use std::{env};

/// Q&A web service API
#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// Database user
    #[clap(long, default_value = "user")]
    pub db_user: String,
    /// Database user
    #[clap(long, default_value = "password")]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "rustwebdev")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        let config = Config::parse();

        if let Err(_) = env::var("BAD_WORDS_API_KEY") {
            panic!("BadWords API key not set");
        }

        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO_KEY not set");
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(|e| handle_errors::Error::ParseError(e))?;

        let db_user = env::var("POSTGRES_USER").unwrap_or(config.db_user.to_owned());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap_or(config.db_password.to_owned());
        let db_host = env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());
        let db_port = env::var("POSTGRES_PORT").unwrap_or(config.db_port.to_string());
        let db_name = env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());
        let log_level = env::var("LOG_LEVEL").unwrap_or(config.log_level.to_owned());

        Ok(Config {
            log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(|e| handle_errors::Error::ParseError(e))?,
            db_name,
        })
    }
}

#[cfg(test)]
mod config_tests {
    use super::Config;
    use std::{env::{set_var, remove_var}, panic::catch_unwind};

    fn set_env() {
        set_var("BAD_WORDS_API_KEY", "api_key");
        set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        set_var("POSTGRES_USER", "user");
        set_var("POSTGRES_PASSWORD", "password");
        set_var("POSTGRES_HOST", "localhost");
        set_var("POSTGRES_PORT", "5432");
        set_var("POSTGRES_DB", "rustwebdev");
    }

    // fn unset_env() {
    //     remove_var("BAD_WORDS_API_KEY");
    //     remove_var("PASETO_KEY");
    //     remove_var("POSTGRES_USER");
    //     remove_var("POSTGRES_PASSWORD");
    //     remove_var("POSTGRES_HOST");
    //     remove_var("POSTGRES_PORT");
    //     remove_var("POSTGRES_DB");
    // }

    #[test]
    fn unset_and_set_api_key() {

        // catch_unwind avoid panicking and captures the error to be tested;
        let result = catch_unwind(|| Config::new());
        assert!(result.is_err());

        set_env();
        let expected = Config {
            log_level: "warn".to_string(),
            port: 8080,
            db_user: "user".to_string(),
            db_password: "password".to_string(),
            db_host: "localhost".to_string(),
            db_port: 5432,
            db_name: "rustwebdev".to_string(),
        };

        let config = Config::new().unwrap();

        assert_eq!(config, expected);
    }
}
