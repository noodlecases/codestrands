use actix_web::web::ServiceConfig;

mod auth;
mod chat_messages;
mod chat_users;
mod chats;
mod interests;
mod projects;
mod skills;
mod social_links;
mod swipes;
mod user;
mod user_interests;
mod user_relationships;
mod user_skills;

pub fn config(config: &mut ServiceConfig) {
    config
        .configure(auth::config)
        .configure(chat_messages::config)
        .configure(chat_users::config)
        .configure(chats::config)
        .configure(interests::config)
        .configure(projects::config)
        .configure(skills::config)
        .configure(social_links::config)
        .configure(swipes::config)
        .configure(user::config)
        .configure(user_interests::config)
        .configure(user_relationships::config)
        .configure(user_skills::config);
}
