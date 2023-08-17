use crate::db::Store;
use crate::error::AppError;
use crate::models::post::{CreatePost, Post, UpdatePost};
use axum::extract::{Path, State};
use axum::Json;

// Posts ---------------------------------------------------------------------------------------------------------------
pub async fn get_all_posts(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Post>>, AppError> {
    let posts = am_database.get_all_posts().await?;
    Ok(Json(posts))
}

pub async fn get_post_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>,
) -> Result<Json<Post>, AppError> {
    let post = am_database.get_post_by_id(query).await?;
    Ok(Json(post))
}

pub async fn create_post(
    State(mut am_database): State<Store>,
    Json(post): Json<CreatePost>,
) -> Result<Json<Post>, AppError> {
    let new_post = am_database.add_post(post).await?;
    Ok(Json(new_post))
}

pub async fn delete_post_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>,
) -> Result<(), AppError> {
    am_database.delete_post_by_id(query).await?;

    Ok(())
}

pub async fn update_post_by_id(
    State(mut am_database): State<Store>,
    Json(updated_post): Json<UpdatePost>,
) -> Result<Json<Post>, AppError> {
    let updated_post = am_database.update_post_by_id(updated_post).await?;

    Ok(Json(updated_post))
}

pub async fn get_user_posts_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>,
) -> Result<Json<Vec<Post>>, AppError> {
    let user_posts = am_database.get_user_posts_by_id(query).await?;
    Ok(Json(user_posts))
}
