use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::root;
// use crate::routes::comment_routes::comment_routes;
use crate::admin_handlers;
use crate::post_handlers;
use crate::user_handlers;
use crate::vote_handlers;
use crate::{handlers, layers};

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        .route("/", get(root))
        // User login / registration
        .route("/users", post(user_handlers::register))
        .route("/login", post(user_handlers::login))
        .route("/logout", get(user_handlers::logout))
        .route("/protected", get(handlers::protected))
        // Posts
        .route("/posts", get(post_handlers::get_all_posts))
        .route("/posts/:id", get(post_handlers::get_post_by_id))
        .route("/posts", post(post_handlers::create_post))
        .route("/posts/:id", delete(post_handlers::delete_post_by_id))
        .route("/posts", put(post_handlers::update_post_by_id))
        .route("/users/:id/posts", get(post_handlers::get_user_posts_by_id))
        // Votes
        .route("/votes", post(vote_handlers::create_vote_from_form))
        .route("/votes/delete", post(vote_handlers::delete_vote_from_form))
        // NASA
        // .route("/get_apod", post(handlers::get_nasa_post))
        .route("/get_apod", post(handlers::get_nasa_post_by_form))
        // Misc
        .route("/*_", get(handle_404))
        // Admin
        .route("/ban", post(admin_handlers::ban_user))
        .route("/unban", post(admin_handlers::unban_user))
        .route("/promote", post(admin_handlers::promote_admin))
        .route("/demote", post(admin_handlers::demote_admin))
        // .merge(comment_routes())
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}
