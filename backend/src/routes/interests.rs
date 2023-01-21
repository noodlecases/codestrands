use actix_web::{
    get,
    web::{Data, Json, ServiceConfig},
};
use sqlx::PgPool;

use crate::{models::interests::Interest, utils::Result};

#[get("/interests/")]
async fn get_interests(pool: Data<PgPool>) -> Result<Json<Vec<Interest>>> {
    Ok(Json(Interest::all(&pool).await?))
}

pub fn config(config: &mut ServiceConfig) {
    config.service(get_interests);
}
