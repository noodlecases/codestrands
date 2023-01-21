use actix_web::web::ServiceConfig;

mod auth;

pub fn config(config: &mut ServiceConfig) {
    config.configure(auth::config);
}
