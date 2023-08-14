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
        // The router matches these FROM TOP TO BOTTOM explicitly!
        .route("/", get(root))

        // User login / registration
        .route("/users", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/protected", get(handlers::protected))

        // Posts
        .route("/posts", get(handlers::get_all_posts))
        .route("/posts/:id", get(handlers::get_post_by_id))
        .route("/posts", post(handlers::create_post))
        .route("/posts/:id", delete(handlers::delete_post_by_id))
        .route("/posts", put(handlers::update_post_by_id))
        .route("/users/:id/posts", get(handlers::get_user_posts_by_id))

        // Misc
        .route("/*_", get(handle_404))
        
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
