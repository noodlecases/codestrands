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
#[serde(rename_all = "camelCase")]
struct PatchedUser {
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<Option<String>>,
}

#[get("/users/@me/")]
async fn get_me(session: UserSession, pool: Data<PgPool>) -> Result<Json<codestrands::User>> {
    Ok(Json(
        codestrands::User::get_by_id(session.user_id, &pool).await?,
    ))
}

#[get("/users/{user_id}/")]
async fn get_user(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<codestrands::PublicUser>> {
    Ok(Json(
        codestrands::User::get_by_id(path.into_inner(), &pool)
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

    if let Some(first_name) = &user.first_name {
        latest =
            Some(codestrands::User::update_first_name(session.user_id, first_name, &pool).await?);
    }
    if let Some(last_name) = &user.last_name {
        latest =
            Some(codestrands::User::update_last_name(session.user_id, last_name, &pool).await?);
    }
    if let Some(bio) = &user.bio {
        latest = Some(codestrands::User::update_bio(session.user_id, bio.clone(), &pool).await?);
    }

    match latest {
        Some(patched_user) => Ok(Json(patched_user)),
        None => codestrands_error!(400, "no updated fields specified"),
    }
}

pub fn config(config: &mut ServiceConfig) {
    config.service(get_me).service(get_user).service(patch_me);
}
