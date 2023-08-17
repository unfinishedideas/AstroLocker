use axum::extract::State;
use axum::response::Response;
use axum::Form;
use hyper::Body;

use crate::db::Store;
use crate::error::AppError;
use crate::models::user::UserEmail;
use crate::handlers::create_response_path;

// Admin ---------------------------------------------------------------------------------------------------------------
pub async fn ban_user(
    State(mut am_database): State<Store>,
    Form(email_to_ban): Form<UserEmail>
) -> Result<Response<Body>, AppError> {
    am_database.ban_user_by_email(email_to_ban.email).await?;
    let response = create_response_path();
    Ok(response)
}

pub async fn unban_user(
    State(mut am_database): State<Store>,
    Form(email_to_unban): Form<UserEmail>
) -> Result<Response<Body>, AppError> {
    am_database.unban_user_by_email(email_to_unban.email).await?;
    let response = create_response_path();
    Ok(response)
}

pub async fn promote_admin(
    State(mut am_database): State<Store>,
    Form(email_to_admin): Form<UserEmail>
) -> Result<Response<Body>, AppError> {
    am_database.promote_admin_by_email(email_to_admin.email).await?;
    let response = create_response_path();
    Ok(response)
}

pub async fn demote_admin(
    State(mut am_database): State<Store>,
    Form(email_to_admin): Form<UserEmail>
) -> Result<Response<Body>, AppError> {
    am_database.demote_admin_by_email(email_to_admin.email).await?;
    let response = create_response_path();
    Ok(response)
}