use crate::make_db_id;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {}, title: {}, explanation: {}, img_url: {}, apod_date: {}",
    id,
    title,
    explanation,
    img_url,
    apod_date
)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub query_string: String,
    pub explanation: String,
    pub img_url: String,
    pub apod_date: String
}

impl Post {
    #[allow(dead_code)]
    pub fn new(id: PostId, title: String, query_string: String, explanation: String, 
        img_url: String, apod_date: String
    ) -> Self {
        Post {
            id,
            title,
            query_string,
            explanation,
            img_url,
            apod_date
        }
    }
}

make_db_id!(PostId);

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub query_string: String,
    pub explanation: String,
    pub img_url: String,
    pub apod_date: String
}

#[derive(Deserialize)]
pub struct GetPostById {
    pub post_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    pub id: PostId,
    pub title: String,
    pub query_string: String,
    pub explanation: String,
    pub img_url: String,
    pub apod_date: String
}
