use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NasaQuery {
    pub query_string: String,
}
