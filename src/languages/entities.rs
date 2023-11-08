use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, FromRow)]
pub struct Language {
    pub uuid: Uuid,
    pub name: String,
    pub code: String,
    pub short_code: String,
    pub description: String,
}
