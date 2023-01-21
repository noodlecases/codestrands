use actix_web::{
    delete, get, put,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use sqlx::PgPool;

use crate::models::user_interests::UserInterest;
use crate::{
    utils::{auth::UserSession, Result},
};

#[get("/users/@me/interests/")]
async fn get_me_interests(
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<Vec<UserInterest>>> {
    Ok(Json(UserInterest::get(session.user_id, &pool).await?))
}

#[get("/users/{user_id}/interests/")]
async fn get_user_interests(
    path: Path<i32>,
    pool: Data<PgPool>,
) -> Result<Json<Vec<UserInterest>>> {
    Ok(Json(UserInterest::get(path.into_inner(), &pool).await?))
}

#[put("/users/@me/interests/{interest_id}/")]
async fn put_me_interest(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<UserInterest>> {
    Ok(Json(
        UserInterest::create(session.user_id, path.into_inner(), &pool).await?,
    ))
}

#[delete("/users/@me/interests/{interest_id}/")]
async fn delete_me_interest(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<HttpResponse> {
    UserInterest::delete(session.user_id, path.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().finish())
}

pub fn config(config: &mut ServiceConfig) {
    config
        .service(get_me_interests)
        .service(get_user_interests)
        .service(put_me_interest)
        .service(delete_me_interest);
}
