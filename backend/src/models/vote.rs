use crate::make_db_id;
use serde_derive::{Deserialize, Serialize};
use crate::models::post::PostId;

#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {}, post_id: {}, user_id: {}",
    id,
    post_id,
    user_id
)]
pub struct Vote {
    pub id: VoteId,
    pub post_id: PostId,
    pub user_id: i32
}

make_db_id!(VoteId);

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVote {
    pub post_id: PostId,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct GetVoteById {
    pub vote_id: i32,
}
