use axum::extract::State;
use axum::response::{Html, Response};
use axum::{Form, Json};
use http::header::LOCATION;
use http::{HeaderValue, StatusCode};
use hyper::Body;
use serde_json::Value;
use tera::Context;
use tracing::error;

use crate::db::Store;
use crate::error::AppError;
use crate::models::post::{Post, CreatePost};
use crate::models::user::{Claims, OptionalClaims};
use crate::models::displaypost::{DisplayPost, DisplayPostId};
use crate::models::nasaquery::NasaQuery;

use crate::template::TEMPLATES;

#[allow(dead_code)]
pub async fn root(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("is_admin", &false);
    context.insert("is_banned", &false);

    let template_name = if let Some(claims_data) = claims {
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        let current_user_id = am_database.get_user_id_by_email(claims_data.email.clone()).await?;
        context.insert("current_user_id", &current_user_id);
        
        // determine if banned
        let is_banned = am_database.determine_if_user_banned(claims_data.email.clone()).await?;
        if is_banned == true {
            context.insert("is_banned", &true);
            "banned.html"
        }
        else {
            // determine if admin
            let is_admin = am_database.determine_if_user_admin(claims_data.email).await?;
            if is_admin == true {
                context.insert("is_admin", &true);
            }

            // Get all the post data
            let posts = am_database.get_all_posts().await?;
            let mut display_posts = Vec::new();
            for post in posts {
                let num_likes = am_database.get_number_of_votes_for_post(post.id.0).await?;
                let already_liked = am_database.determine_if_user_liked_post(current_user_id, post.id.0).await?;
                display_posts.push(
                    DisplayPost { 
                        id: DisplayPostId(post.id.0), 
                        title: (post.title), 
                        query_string: (post.query_string), 
                        explanation: (post.explanation), 
                        img_url: (post.img_url), 
                        apod_date: (post.apod_date), 
                        already_liked: (already_liked), 
                        num_likes: (num_likes) }
                )
            }

            let top_posts = am_database.get_top_posts().await?;
            let mut top_display_posts = Vec::new();
            for post_id in top_posts {
                let post = am_database.get_post_by_id(post_id).await?;
                let num_likes = am_database.get_number_of_votes_for_post(post.id.0).await?;
                let already_liked = am_database.determine_if_user_liked_post(current_user_id, post.id.0).await?;
                top_display_posts.push(
                    DisplayPost { 
                        id: DisplayPostId(post.id.0), 
                        title: (post.title), 
                        query_string: (post.query_string), 
                        explanation: (post.explanation), 
                        img_url: (post.img_url), 
                        apod_date: (post.apod_date), 
                        already_liked: (already_liked), 
                        num_likes: (num_likes) }
                )
            }
            context.insert("all_posts", &display_posts);
            context.insert("top_posts", &top_display_posts);

            "main.html"
        }
    } else {
        // Handle the case where the user isn't logged in
        context.insert("is_logged_in", &false);
        "index.html"
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}

pub async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the PROTECTED area :) \n Your claim data is: {}",
        claims
    ))
}


pub fn create_response_path() -> Response<Body> {
    let mut response = Response::builder()
    .status(StatusCode::FOUND)
    .body(Body::empty())
    .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));

    response
}

// NASA ----------------------------------------------------------------------------------------------------------------
pub async fn get_nasa_post_by_form(
    State(am_database): State<Store>,
    Form(new_query): Form<NasaQuery>
) -> Result<Response<Body>, AppError> {
    let query = NasaQuery {
        query_string: new_query.query_string
    };
    let json_query = Json(query);
    let _ = get_nasa_post(axum::extract::State(am_database), json_query).await?;

    let response = create_response_path();
    Ok(response)
}
pub async fn get_nasa_post(
    State(mut am_database): State<Store>,
    Json(query): Json<NasaQuery>
) -> Result<Json<Post>, AppError> {
    // Check to see if post is already in DB
    let is_cached = am_database.check_cache_by_query_string(query.clone()).await?;
    if is_cached == true {
        let cached_post = am_database.get_post_by_query_string(query.clone()).await?;
        return Ok(Json(cached_post));
    }
    // Otherwise, call NASA and create a post for it
    else {
        let date_value = &query.query_string;
        let key = std::env::var("NASA_API_KEY").unwrap();
        let query_string = format!("https://api.nasa.gov/planetary/apod/?api_key={key}&date={date_value}");
        let res = reqwest::get(&query_string)
            .await
            .map_err(|_| AppError::InternalServerError)?
            .text()
            .await
            .map_err(|_| AppError::InternalServerError)?;

        let response = serde_json::from_str::<Value>(&res).unwrap();    
        
        // deal with out of range response
        if response["code"] == 400 {
            return Err(AppError::InvalidDateRange)
        }

        // .as_str().unwrap().to_string() seems really stupid but it was the only way I was able to get it
        // to work without adding quotation marks to my fields. Or changing the post struct to use str
        let post_to_add = CreatePost {
            title: response["title"].as_str().unwrap().to_string(),
            explanation: response["explanation"].as_str().unwrap().to_string(),
            query_string: query.query_string,
            img_url: response["url"].as_str().unwrap().to_string(),
            apod_date: response["date"].as_str().unwrap().to_string()
        };

        let new_post = am_database.add_post(post_to_add).await?;
        Ok(Json(new_post))
    }
}

