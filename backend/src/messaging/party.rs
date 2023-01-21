use std::collections::HashSet;

use actix::SpawnHandle;
use uuid::Uuid;

pub struct Party {
    pub id: Uuid,
    pub anime_aid: Option<String>,
    pub episode: Option<i32>,
    pub duration_watched: f32,
    pub completed: bool,
    pub members: HashSet<Uuid>,
    pub pending_removal: Option<SpawnHandle>,
}

impl Party {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            anime_aid: None,
            episode: None,
            duration_watched: 0.0,
            completed: false,
            members: HashSet::new(),
            pending_removal: None,
        }
    }
}
