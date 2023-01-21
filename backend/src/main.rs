use actix::Actor;
use actix_cors::Cors;
use actix_session::{
    config::{CookieContentSecurity, PersistentSession, SessionLifecycle, TtlExtensionPolicy},
    storage::RedisSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration, SameSite},
    get,
    middleware::{NormalizePath, TrailingSlash},
    web::{scope, Data, JsonConfig, PathConfig, QueryConfig},
    App, HttpServer, Responder,
};
use dotenv::dotenv;
use log::info;
use sqlx::postgres::PgPoolOptions;

mod config;
mod messaging;
mod models;
mod routes;
mod utils;

use config::{codestrands_config, Config};
use messaging::Lobby;
use utils::Result;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = Config::default();
    env_logger::init();

    let http_client = reqwest::Client::default();

    let db_pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&config.database_url)
        .await?;

    let redis_client = redis::Client::open(config.redis_url.clone())?;
    let redis_session_store = RedisSessionStore::new(&config.redis_url).await?;

    info!("Running database migrations..");
    sqlx::migrate!().run(&db_pool).await?;

    let messaging_lobby = Lobby::new(db_pool.clone()).start();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(codestrands_config!(JsonConfig))
            .app_data(codestrands_config!(PathConfig))
            .app_data(codestrands_config!(QueryConfig))
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(redis_client.clone()))
            .app_data(Data::new(http_client.clone()))
            .app_data(Data::new(messaging_lobby.clone()))
            .wrap(if config.debug {
                Cors::default()
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://127.0.0.1:8000")
                    .allowed_origin("http://localhost:8000")
                    .allowed_origin("http://codestrands.local:8000")
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
            } else {
                Cors::default()
                    .allowed_origin("https://codestrands.tech")
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
            })
            .wrap(
                SessionMiddleware::builder(redis_session_store.clone(), config.secret_key.clone())
                    .cookie_content_security(CookieContentSecurity::Signed)
                    .cookie_http_only(true)
                    .cookie_name("token".into())
                    .cookie_same_site(SameSite::Strict)
                    .cookie_secure(!config.debug)
                    .session_lifecycle(SessionLifecycle::PersistentSession(
                        PersistentSession::default()
                            .session_ttl(Duration::weeks(1))
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                    ))
                    .build(),
            )
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .service(scope("/api/v1").service(index).configure(routes::config))
    });

    info!("Starting server..");
    server.bind("0.0.0.0:8000")?.run().await?;

    Ok(())
}
