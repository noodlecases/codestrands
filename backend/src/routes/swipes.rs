use std::collections::HashSet;

use actix_session::Session;
use actix_web::{
    get, post,
    web::{Data, Json, Query, ServiceConfig},
    HttpResponse,
};
use openssl::base64;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::PgPool;
use url::Url;

use crate::{
    models::{auth::codestrands, skills::Skill},
    utils::{auth::UserSession, Result},
};

#[derive(Deserialize)]
struct TopSwipeIgnoresQuery {
    pub ignores: String,
}

/// this is probably one of the most horrific pieces of code i've written to date.
/// i've been awake for almost 3 days straight..
#[get("/users/@me/swipes/")]
async fn get_top_swipe(
    query: Query<TopSwipeIgnoresQuery>,
    session: UserSession,
    pool: Data<PgPool>,
) -> Result<Json<codestrands::PublicUser>> {
    let ignores = query
        .ignores
        .split(",")
        .map(|ig| ig.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let skills = sqlx::query_as::<_, Skill>("SELECT * FROM skills WHERE user_id = $1")
        .bind(session.user_id)
        .fetch_all(pool.as_ref())
        .await?
        .iter()
        .map(|s| s.id)
        .collect::<Vec<i32>>();
    let hskills: HashSet<i32> = HashSet::from_iter(skills);

    let interests = sqlx::query_as::<_, Skill>("SELECT * FROM interests WHERE user_id = $1")
        .bind(session.user_id)
        .fetch_all(pool.as_ref())
        .await?
        .iter()
        .map(|s| s.id)
        .collect::<Vec<i32>>();
    let hinterests: HashSet<i32> = HashSet::from_iter(interests);

    let users = sqlx::query_as::<_, codestrands::User>("SELECT * FROM users WHERE user_id != $1")
        .bind(session.user_id)
        .fetch_all(pool.as_ref())
        .await?;

    let mut top = None;
    let mut top_score: i32 = -1;
    for user in users.iter() {
        if ignores.contains(&user.id) {
            continue;
        }

        let uskills = sqlx::query_as::<_, Skill>("SELECT * FROM skills WHERE user_id = $1")
            .bind(user.id)
            .fetch_all(pool.as_ref())
            .await?
            .iter()
            .map(|s| s.id)
            .collect::<Vec<i32>>();

        let uinterests = sqlx::query_as::<_, Skill>("SELECT * FROM interests WHERE user_id = $1")
            .bind(user.id)
            .fetch_all(pool.as_ref())
            .await?
            .iter()
            .map(|s| s.id)
            .collect::<Vec<i32>>();

        let possible = hskills.intersection(&HashSet::from_iter(uskills)).count()
            + hinterests
                .intersection(&HashSet::from_iter(uinterests))
                .count();

        if possible as i32 > top_score {
            top_score = possible as i32;
            top = Some(user);
        }
    }

    Ok(Json(
        sqlx::query_as::<_, codestrands::User>("SELECT * FROM users WHERE user_id != $1")
            .bind(top.unwrap().id)
            .fetch_one(pool.as_ref())
            .await?
            .into_public(),
    ))
}

pub fn config(config: &mut ServiceConfig) {
    config.service(get_top_swipe);
}
