use actix_web::web::ServiceConfig;

mod auth;
mod social_links;
mod user;
mod user_relationships;

pub fn config(config: &mut ServiceConfig) {
    config
        .configure(auth::config)
        .configure(user::config)
        .configure(user_relationships::config)
        .configure(social_links::config);
}
