use actix_web::{
    delete, get, put,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::user_skills::UserSkill;
use crate::utils::{auth::UserSession, Result};

#[derive(Deserialize)]
struct UserIdPath {
    user_id: i32,
}

#[derive(Deserialize)]
struct PutDeleteUserSkill {
    user_id: Option<i32>,
    skill_id: Option<i32>,
}

#[get("/users/@me/skills/")]
async fn get_me_skills(session: UserSession, pool: Data<PgPool>) -> Result<Json<Vec<UserSkill>>> {
    Ok(Json(UserSkill::get(session.user_id, &pool).await?))
}

#[get("/users/{user_id}/skills/")]
async fn get_user_skills(
    path: Path<UserIdPath>,
    pool: Data<PgPool>,
) -> Result<Json<Vec<UserSkill>>> {
    Ok(Json(UserSkill::get(path.user_id, &pool).await?))
}

#[put("/users/@me/skills/")]
async fn put_me_skills(
    user_skill: Json<UserSkill>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<UserSkill>> {
    Ok(Json(
        UserSkill::create(session.user_id, user_skill.skill_id, &pool).await?,
    ))
}

#[delete("/users/@me/skills/")]
async fn delete_me_skills(
    user_skill: Json<UserSkill>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<HttpResponse> {
    UserSkill::delete(session.user_id, user_skill.skill_id, &pool).await?;
    Ok(HttpResponse::Ok().finish())
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_me_skills)
        .service(get_user_skills)
        .service(put_me_skills)
        .service(delete_me_skills);
}
