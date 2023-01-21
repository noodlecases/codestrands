use actix_web::{
    delete, get, put,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::interests::Interest;
use crate::models::user_interests::UserInterest;
use crate::{
    models::auth::codestrands,
    utils::{auth::UserSession, error::codestrands_error, Result},
};

#[derive(Deserialize)]
struct UserIdPath {
    user_id: i32,
}

#[derive(Deserialize)]
struct PutDeleteUserInterest {
    user_id: Option<i32>,
    interest_id: Option<i32>,
}

#[get("/users/@me/interests/")]
async fn get_me_interests(
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<Vec<UserInterest>>> {
    Ok(Json(UserInterest::get(session.user_id, &pool).await?))
}

#[get("/users/{user_id}/interests/")]
async fn get_user_interests(
    path: Path<UserIdPath>,
    pool: Data<PgPool>,
) -> Result<Json<Vec<UserInterest>>> {
    Ok(Json(UserInterest::get(path.user_id, &pool).await?))
}

#[put("/users/@me/interests/")]
async fn put_me_interest(
    user_interest: Json<UserInterest>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<UserInterest>> {
    Ok(Json(
        UserInterest::create(session.user_id, user_interest.interest_id, &pool).await?,
    ))
}

#[delete("/users/@me/interests/")]
async fn delete_me_interest(
    user_interest: Json<UserInterest>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<HttpResponse> {
    UserInterest::delete(session.user_id, user_interest.interest_id, &pool).await?;
    Ok(HttpResponse::Ok().finish())
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_me_interests)
        .service(get_user_interests)
        .service(put_me_interest)
        .service(delete_me_interest);
}
