use std::env;

use actix_web::cookie::Key;

macro_rules! codestrands_config {
    ($config:ident) => {
        $config::default()
            .error_handler(|err, _| $crate::utils::error::CodestrandsError::new(err.into()).into())
    };
}

pub(crate) use codestrands_config;

#[derive(Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub oauth_url: String,
}

#[derive(Clone)]
pub struct Config {
    pub debug: bool,
    pub logging: String,
    pub database_url: String,
    pub redis_url: String,
    pub oauth_redirect_uri: String,
    pub oauth: OAuthConfig,
    pub secret_key: Key,
}

impl Default for Config {
    fn default() -> Self {
        let logging = match env::var("RUST_LOG") {
            Ok(rust_log) => rust_log,
            Err(_) => {
                let rust_log = "info,sqlx::query=warn".to_string();
                env::set_var("RUST_LOG", &rust_log);
                rust_log
            }
        };

        let debug = env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true";

        let database_url =
            env::var("DATABASE_URL").expect("missing `DATABASE_URL` environment variable");
        let redis_url = env::var("REDIS_URL").expect("missing `REDIS_URL` environment variable");

        let oauth_redirect_uri = env::var("OAUTH_REDIRECT_URI")
            .expect("missing `OAUTH_REDIRECT_URI` environment variable");

        let oauth = OAuthConfig {
            client_id: env::var("OAUTH_CLIENT_ID")
                .expect("missing `OAUTH_CLIENT_ID` environment variable"),
            client_secret: env::var("OAUTH_CLIENT_SECRET")
                .expect("missing `OAUTH_CLIENT_SECRET` environment variable"),
            oauth_url: env::var("OAUTH_URL").expect("missing `OAUTH_URL` environment variable"),
        };

        let secret_key = Key::from(
            env::var("SECRET_KEY")
                .expect("missing `SECRET_KEY` environment variable")
                .as_bytes(),
        );

        Self {
            debug,
            logging,
            database_url,
            redis_url,
            oauth_redirect_uri,
            oauth,
            secret_key,
        }
    }
}
