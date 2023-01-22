use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::social_links::SocialLink,
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialLinkData {
    name: String,
    url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PatchedSocialLink {
    name: Option<String>,
    url: Option<String>,
}

#[get("/users/{user_id}/social-links/")]
async fn get_user_social_links(
    path: Path<i32>,
    pool: Data<PgPool>,
) -> Result<Json<Vec<SocialLink>>> {
    Ok(Json(SocialLink::get(path.into_inner(), &pool).await?))
}

#[post("/users/@me/social-links/")]
async fn create_social_link(
    session: UserSession,
    pool: Data<PgPool>,
    sld: Json<SocialLinkData>,
) -> Result<Json<SocialLink>> {
    Ok(Json(
        SocialLink::create(&sld.name, &sld.url, session.user_id, &pool).await?,
    ))
}

#[delete("/users/@me/social-links/{id}/")]
async fn delete_social_link(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<HttpResponse> {
    SocialLink::delete(path.into_inner(), session.user_id, &pool).await?;
    Ok(HttpResponse::Ok().finish())
}

#[patch("/users/@me/social-links/{id}/")]
async fn update_social_link(
    social_link: Json<PatchedSocialLink>,
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<SocialLink>> {
    let sl_id = path.into_inner();
    let mut latest = None;

    if let Some(name) = &social_link.name {
        latest = Some(SocialLink::update_name(sl_id, session.user_id, name, &pool).await?);
    }
    if let Some(url) = &social_link.url {
        latest = Some(SocialLink::update_url(sl_id, session.user_id, url, &pool).await?);
    }

    match latest {
        Some(patched_social_link) => Ok(Json(patched_social_link)),
        None => codestrands_error!(400, "no updated fields specified"),
    }
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_user_social_links)
        .service(create_social_link)
        .service(delete_social_link)
        .service(update_social_link);
}
