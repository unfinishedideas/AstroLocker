use axum::extract::State;
use axum::response::Response;
use axum::{Form, Json};
use hyper::Body;

use crate::db::Store;
use crate::error::AppError;
use crate::handlers::create_response_path;
use crate::models::vote::{CreateVote, Vote};

// Votes ---------------------------------------------------------------------------------------------------------------
pub async fn create_vote(
    State(mut am_database): State<Store>,
    Json(vote): Json<CreateVote>,
) -> Result<Json<Vote>, AppError> {
    let new_vote = CreateVote {
        post_id: vote.post_id,
        user_id: vote.user_id,
    };
    let finished_vote = am_database.create_vote(new_vote).await?;

    Ok(Json(finished_vote))
}

pub async fn create_vote_from_form(
    State(mut am_database): State<Store>,
    Form(vote): Form<CreateVote>,
) -> Result<Response<Body>, AppError> {
    let new_vote = CreateVote {
        post_id: vote.post_id,
        user_id: vote.user_id,
    };
    am_database.create_vote(new_vote).await?;
    let response = create_response_path();

    Ok(response)
}

pub async fn delete_vote_from_form(
    State(mut am_database): State<Store>,
    Form(vote): Form<CreateVote>,
) -> Result<Response<Body>, AppError> {
    let old_vote = CreateVote {
        post_id: vote.post_id,
        user_id: vote.user_id,
    };
    am_database.delete_vote(old_vote).await?;
    let response = create_response_path();
    Ok(response)
}

pub async fn get_votes_for_post(
    State(mut am_database): State<Store>,
    query: i32,
) -> Result<Json<i64>, AppError> {
    let num_votes = am_database.get_number_of_votes_for_post(query).await?;
    Ok(Json(num_votes))
}
