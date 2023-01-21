use actix_web::{
    get,
    web::{Data, Json, ServiceConfig},
};
use sqlx::PgPool;

use crate::{models::skills::Skill, utils::Result};

#[get("/skills/")]
async fn get_skills(pool: Data<PgPool>) -> Result<Json<Vec<Skill>>> {
    Ok(Json(Skill::all(&pool).await?))
}

pub fn config(config: &mut ServiceConfig) {
    config.service(get_skills);
}
