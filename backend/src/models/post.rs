use crate::make_db_id;
use serde_derive::{Deserialize, Serialize};

// TODO: Change user_id's to UserId type!!!!!!
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {}, title: {}, img_url: {}, user_id: {}",
    id,
    title,
    img_url,
    user_id
)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub img_url: String,
    pub user_id: i32,
}

impl Post {
    #[allow(dead_code)]
    pub fn new(id: PostId, title: String, img_url: String, user_id: i32) -> Self {
        Post {
            id,
            title,
            img_url,
            user_id,
        }
    }
}

make_db_id!(PostId);

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub img_url: String,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct GetQuestionById {
    pub post_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestion {
    pub id: PostId,
    pub title: String,
    pub img_url: String,
    pub user_id: i32,
}
