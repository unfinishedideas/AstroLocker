use crate::make_db_id;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {}, title: {}, explanation: {}, img_url: {}, apod_date: {}, already_liked {}, num_likes {}",
    id,
    title,
    explanation,
    img_url,
    apod_date,
    already_liked,
    num_likes
)]
pub struct DisplayPost {
    pub id: DisplayPostId,
    pub title: String,
    pub query_string: String,
    pub explanation: String,
    pub img_url: String,
    pub apod_date: String,
    pub already_liked: bool,
    pub num_likes: i64,
}

impl DisplayPost {
    #[allow(dead_code)]
    pub fn new(
        id: DisplayPostId,
        title: String,
        query_string: String,
        explanation: String,
        img_url: String,
        apod_date: String,
        already_liked: bool,
        num_likes: i64,
    ) -> Self {
        DisplayPost {
            id,
            title,
            query_string,
            explanation,
            img_url,
            apod_date,
            already_liked,
            num_likes,
        }
    }
}

make_db_id!(DisplayPostId);

// // Clients use this to create new requests
// #[derive(Debug, Serialize, Deserialize)]
// pub struct CreateDisplayPost {
//     pub title: String,
//     pub query_string: String,
//     pub explanation: String,
//     pub img_url: String,
//     pub apod_date: String,
//     pub already_liked: bool
//     pub num_likes: i64
// }

// #[derive(Deserialize)]
// pub struct GetDisplayPostById {
//     pub post_id: i32,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UpdateDisplayPost {
//     pub id: DisplayPostId,
//     pub title: String,
//     pub query_string: String,
//     pub explanation: String,
//     pub img_url: String,
//     pub apod_date: String,
//     pub already_liked: bool,
//     pub num_likes: i64
// }
