use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::root;
// use crate::routes::comment_routes::comment_routes;
use crate::{handlers, layers};

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        .route("/", get(root))

        // User login / registration
        .route("/users", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/logout", get(handlers::logout))
        .route("/protected", get(handlers::protected))

        // Posts
        .route("/posts", get(handlers::get_all_posts))
        .route("/posts/:id", get(handlers::get_post_by_id))
        .route("/posts", post(handlers::create_post))
        .route("/posts/:id", delete(handlers::delete_post_by_id))
        .route("/posts", put(handlers::update_post_by_id))
        .route("/users/:id/posts", get(handlers::get_user_posts_by_id))

        // Votes
        .route("/votes", post(handlers::create_vote_from_form))
        .route("/votes/delete", post(handlers::delete_vote_from_form))
        .route("/votes/:id", delete(handlers::delete_vote_by_id)) // <- turns out this is useless due to how I set up the db
        // .route("/posts/votes/:id", post(handlers::get_votes_for_post))

        // NASA
        // .route("/get_apod", post(handlers::get_nasa_post))
        .route("/get_apod", post(handlers::get_nasa_post_by_form))

        // Misc
        .route("/*_", get(handle_404))

        // Admin
        .route("/ban", post(handlers::ban_user))
        .route("/unban", post(handlers::unban_user))
        .route("/promote", post(handlers::promote_admin))
        .route("/demote", post(handlers::demote_admin))
        
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
