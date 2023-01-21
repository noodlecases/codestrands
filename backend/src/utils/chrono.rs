use chrono::Duration;
use serde::{Deserialize, Deserializer};

pub fn duration_from_secs<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let secs: i64 = Deserialize::deserialize(deserializer)?;
    Ok(Duration::seconds(secs))
}
