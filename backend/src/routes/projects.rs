use actix_web::{
    get,
    web::{Data, Json, Path, Query, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::projects::Project,
    utils::{auth::UserSession, error::{codestrands_error, CodestrandsError}, Result},
};

#[derive(Deserialize)]
struct ProjectQuery{
    name: String,
    description: String,
    url: String,
    image: String,
}

struct NameQuery{
    name: String,
}


#[get("/users/{user_id}/projects/")]
async fn get_by_user(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<Vec<Project>>> {
    Ok(Json(Project::get_by_user(path.into_inner(), &pool).await?))
}

#[get("/users/@me/projects/")]
async fn create(session: UserSession, query: Query<ProjectQuery>, pool: Data<PgPool>) -> Result<Json<Project>> {
    Ok(Json(Project::create(session.user_id, &query.name, &query.description, 
    &query.url, &query.image, &pool).await?))
}

#[get("/users/@me/projects/")]
async fn delete(session: UserSession, pool: Data<PgPool>) -> Result<Json<()>> {
    Ok(Json(Project::delete(session.user_id, &pool).await?))
}

// #[get("/users/@me/projects/")]
// async fn update_name(session: UserSession, query: Query<NameQuery>, pool: Data<PgPool>) -> Result<Project, CodestrandsError> {
//     Ok(Json(Project::update_name(session.user_id, query.name, &pool)))
// }