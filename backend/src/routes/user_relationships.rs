use actix_web::{
    get, put,
    web::{Data, Json, Path, ServiceConfig},
};
use sqlx::PgPool;

use crate::{
    models::user_relationships::{UserRelationship, UserRelationshipType},
    utils::{auth::UserSession, Result},
};

#[get("/users/@me/relationships/{other_user_id}/")]
async fn get_relationship_with_user(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<UserRelationship>> {
    Ok(Json(
        UserRelationship::get_by_pair(session.user_id, path.into_inner(), &pool).await?,
    ))
}

#[put("/users/@me/relationships/{other_user_id}/like/")]
async fn like_user(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<UserRelationship>> {
    let me = session.user_id;
    let other = path.into_inner();

    let new_rel_type = if me < other {
        UserRelationshipType::UserOneInitiated
    } else {
        UserRelationshipType::UserTwoInitiated
    };

    let rel_type = if let Ok(current_rel_type) =
        UserRelationship::get_by_pair(me, other, &pool).await
    {
        // TODO: account for when user turns their reject into a like
        //       although, that shouldn't be possible via the UI
        match (current_rel_type.rel_type.into(), &new_rel_type) {
            (UserRelationshipType::UserOneInitiated, UserRelationshipType::UserTwoInitiated)
            | (UserRelationshipType::UserTwoInitiated, UserRelationshipType::UserOneInitiated)
            | (UserRelationshipType::Matched, _) => UserRelationshipType::Matched,
            (UserRelationshipType::Rejected, _) | (_, UserRelationshipType::Rejected) => {
                UserRelationshipType::Rejected
            }
            _ => new_rel_type,
        }
    } else {
        new_rel_type
    };

    Ok(Json(
        UserRelationship::upsert(me, other, rel_type, &pool).await?,
    ))
}

#[put("/users/@me/relationships/{other_user_id}/reject/")]
async fn reject_user(
    path: Path<i32>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<UserRelationship>> {
    let me = session.user_id;
    let other = path.into_inner();

    Ok(Json(
        UserRelationship::upsert(me, other, UserRelationshipType::Rejected, &pool).await?,
    ))
}

pub fn config(config: &mut ServiceConfig) {
    config.service(get_relationship_with_user);
}
