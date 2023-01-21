use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum Status {
    Online = 0,
    Offline = 1,
}
