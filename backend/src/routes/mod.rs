use actix_web::web::ServiceConfig;

mod auth;
mod interests;
mod skills;
mod social_links;
mod user;
mod user_interests;
mod user_skills;

pub fn config(config: &mut ServiceConfig) {
    config
        .configure(auth::config)
        .configure(user::config)
        .configure(social_links::config);
}
