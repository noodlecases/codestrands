use actix_web::{
    get,
    web::{Data, Json, Path, Query, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::projects::Project,
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[derive(Deserialize)]
struct ProjectPath{
    user_id: i32,
    name: String,
    description: String,
    url: String,
    image: String,
}


#[get("/users/{user_id}/projects/")]
async fn get_by_user(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<Vec<Project>>> {
    Ok(Json(Project::get_by_user(path.into_inner(), &pool).await?))
}

#[get("/users/@me/projects/")]
async fn create(session: UserSession, pool: Data<PgPool>) -> Result<Json<Project>> {
    Ok(Json(Project::create(session.user_id, &path.name, &path.description, 
    &path.url, &path.image, &pool).await?))
}

#[get("/users/user_id/projects/")]
async fn delete(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<()>> {
    Ok(Json(Project::delete(path.into_inner(), &pool).await?))
}

#[get]