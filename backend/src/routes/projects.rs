use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    models::projects::Project,
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewProjectData {
    name: String,
    description: String,
    url: String,
    image: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PatchedProject {
    name: Option<String>,
    description: Option<String>,
    url: Option<String>,
    image: Option<String>,
}

#[get("/users/{user_id}/projects/")]
async fn get_user_projects(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<Vec<Project>>> {
    Ok(Json(Project::get_by_user(path.into_inner(), &pool).await?))
}

#[get("/users/@me/projects/")]
async fn get_me_projects(session: UserSession, pool: Data<PgPool>) -> Result<Json<Vec<Project>>> {
    Ok(Json(Project::get_by_user(session.user_id, &pool).await?))
}

#[post("/users/@me/projects/")]
async fn create_project(
    session: UserSession,
    project: Json<NewProjectData>,
    pool: Data<PgPool>,
) -> Result<Json<Project>> {
    Ok(Json(
        Project::create(
            session.user_id,
            &project.name,
            &project.description,
            &project.url,
            project.image.clone(),
            &pool,
        )
        .await?,
    ))
}

#[delete("/users/@me/projects/{project_id}/")]
async fn delete_project(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<()>> {
    Ok(Json(
        Project::delete(session.user_id, path.into_inner(), &pool).await?,
    ))
}

#[patch("/users/@me/projects/{project_id}/")]
async fn update_project(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
    project: Json<PatchedProject>,
) -> Result<Json<Project>> {
    let project_id = path.into_inner();
    let mut latest = None;

    if let Some(name) = &project.name {
        latest = Some(Project::update_name(session.user_id, project_id, name, &pool).await?);
    }
    if let Some(description) = &project.description {
        latest = Some(
            Project::update_description(session.user_id, project_id, description, &pool).await?,
        );
    }
    if let Some(url) = &project.url {
        latest = Some(Project::update_url(session.user_id, project_id, url, &pool).await?);
    }
    if let Some(image) = &project.image {
        latest = Some(Project::update_image(session.user_id, project_id, image, &pool).await?);
    }

    match latest {
        Some(patched_project) => Ok(Json(patched_project)),
        None => codestrands_error!(400, "no updated fields specified"),
    }
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_me_projects)
        .service(get_user_projects)
        .service(create_project)
        .service(update_project)
        .service(delete_project);
}
