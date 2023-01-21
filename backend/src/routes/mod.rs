use actix_web::web::ServiceConfig;

mod auth;
mod social_links;
mod user;

pub fn config(config: &mut ServiceConfig) {
    config
        .configure(auth::config)
        .configure(user::config)
        .configure(social_links::config);
}
