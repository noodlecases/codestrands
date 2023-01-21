use actix_web::{
    get, patch,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::auth::codestrands,
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[derive(Deserialize)]
struct UserPath {
    username: String,
}

#[derive(Deserialize)]
struct PatchedUser {
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<Option<String>>,
    username: Option<String>,
}

#[get("/users/@me/")]
async fn get_me(session: UserSession, pool: Data<PgPool>) -> Result<Json<codestrands::User>> {
    Ok(Json(
        codestrands::User::get_by_id(session.user_id, &pool).await?,
    ))
}

#[get("/users/{username}/")]
async fn get_user(
    path: Path<UserPath>,
    pool: Data<PgPool>,
) -> Result<Json<codestrands::PublicUser>> {
    Ok(Json(
        codestrands::User::get_by_username(&path.username, &pool)
            .await?
            .into_public(),
    ))
}

#[patch("/users/@me/")]
async fn patch_me(
    user: Json<PatchedUser>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<codestrands::User>> {
    let mut latest = None;

    if let Some(name) = &user.first_name {
        latest = Some(codestrands::User::update_first_name(session.user_id, name, &pool).await?);
    }
    if let Some(name) = &user.last_name {
        latest = Some(codestrands::User::update_last_name(session.user_id, name, &pool).await?);
    }
    if let Some(bio) = &user.bio {
        latest = Some(codestrands::User::update_bio(session.user_id, bio.clone(), &pool).await?);
    }
    if let Some(username) = &user.username {
        latest = Some(codestrands::User::update_username(session.user_id, username, &pool).await?);
    }

    match latest {
        Some(patched_user) => Ok(Json(patched_user)),
        None => codestrands_error!(400, "no updated fields specified"),
    }
}

pub fn config(config: &mut ServiceConfig) {
    config.service(get_me).service(get_user).service(patch_me);
}
