use actix_web::{
    delete, get, put,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use sqlx::PgPool;

use crate::models::user_skills::UserSkill;
use crate::utils::{auth::UserSession, Result};

#[get("/users/@me/skills/")]
async fn get_me_skills(session: UserSession, pool: Data<PgPool>) -> Result<Json<Vec<UserSkill>>> {
    Ok(Json(UserSkill::get(session.user_id, &pool).await?))
}

#[get("/users/{user_id}/skills/")]
async fn get_user_skills(path: Path<i32>, pool: Data<PgPool>) -> Result<Json<Vec<UserSkill>>> {
    Ok(Json(UserSkill::get(path.into_inner(), &pool).await?))
}

#[put("/users/@me/skills/{skill_id}/")]
async fn put_me_skills(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<UserSkill>> {
    Ok(Json(
        UserSkill::create(session.user_id, path.into_inner(), &pool).await?,
    ))
}

#[delete("/users/@me/skills/{skill_id}/")]
async fn delete_me_skills(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<HttpResponse> {
    UserSkill::delete(session.user_id, path.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().finish())
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_me_skills)
        .service(get_user_skills)
        .service(put_me_skills)
        .service(delete_me_skills);
}
