use argon2::Config;
use axum::extract::{Path, Query, State};
use axum::response::{Html, Response};
use axum::{Form, Json};
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;
use serde_json::{json, Value};
use tera::Context;
use tracing::{error, info};

use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;
use crate::models::post::{Post, CreatePost, UpdatePost};
use crate::models::vote::{Vote, CreateVote};
use crate::models::user::{Claims, OptionalClaims, User, UserSignup, KEYS, UserEmail};
use crate::models::nasaquery::NasaQuery;
use crate::models::displaypost::{DisplayPost, DisplayPostId};

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

// User ----------------------------------------------------------------------------------------------------------------
pub async fn register(
    State(mut database): State<Store>,
    Form(mut credentials): Form<UserSignup>,
) -> Result<Response<Body>, AppError> {
    // We should also check to validate other things at some point like email address being in right format
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    if credentials.password != credentials.confirm_password {
        return Err(AppError::MissingCredentials);
    }

    // Check to see if there is already a user in the database with the given email address
    let existing_user = database.get_user(&credentials.email).await;

    if let Ok(_) = existing_user {
        return Err(AppError::UserAlreadyExists);
    }

    // Here we're assured that our credentials are valid and the user doesn't already exist
    // hash their password
    let hash_config = Config::default();
    let salt = std::env::var("SALT").expect("Missing SALT");
    let hashed_password = match argon2::hash_encoded(
        credentials.password.as_bytes(),
        // If you'd like unique salts per-user, simply pass &[] and argon will generate them for you
        salt.as_bytes(),
        &hash_config,
    ) {
        Ok(result) => result,
        Err(_) => {
            return Err(AppError::Any(anyhow::anyhow!("Password hashing failed")));
        }
    };

    credentials.password = hashed_password;

    let _ = database.create_user(credentials.clone()).await?;

    // at this point we've authenticated the user's identity
    // create JWT to return
    let claims = Claims {
        id: 0,
        email: credentials.email.to_owned(),
        exp: get_timestamp_after_8_hours(),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::MissingCredentials)?;

    let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(response)
}



// TODO: Add redirect for failed login!
pub async fn login(
    State(mut database): State<Store>,
    Form(creds): Form<User>,
) -> Result<Response<Body>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let existing_user = database.get_user(&creds.email).await?;

    let is_password_correct =
        match argon2::verify_encoded(&*existing_user.password, creds.password.as_bytes()) {
            Ok(result) => result,
            Err(_) => {
                return Err(AppError::InternalServerError);
            }
        };

    if !is_password_correct {
        return Err(AppError::InvalidPassword);
    }

    // at this point we've authenticated the user's identity
    // create JWT to return
    let claims = Claims {
        id: 0,
        email: creds.email.to_owned(),
        exp: get_timestamp_after_8_hours(),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::MissingCredentials)?;

    let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(response)
}


// Courtesy of Jesse Ellis via Zulip!
pub async fn logout () -> Result<Response<Body>, AppError> {
    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_static("jwt=; path=/; expires=Thu, 01 Jan 1970 00:00:00 UTC;"),
    );
    response.headers_mut().insert(
        LOCATION,
        HeaderValue::from_static("/"),
    );
    Ok(response)
}

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
    let new_post = am_database
        .add_post(post)
        .await?;
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

// Votes ---------------------------------------------------------------------------------------------------------------
pub async fn create_vote(
    State(mut am_database): State<Store>,
    Json(vote): Json<CreateVote>
) -> Result<Json<Vote>, AppError> {
    let new_vote = CreateVote {
        post_id: vote.post_id,
        user_id: vote.user_id
    };
    let finished_vote = am_database.create_vote(new_vote).await?;

    Ok(Json(finished_vote))
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

pub async fn create_vote_from_form(
    State(mut am_database): State<Store>,
    Form(vote): Form<CreateVote>
) -> Result<Response<Body>, AppError> {
    let new_vote = CreateVote {
        post_id: vote.post_id,
        user_id: vote.user_id
    };
    am_database.create_vote(new_vote).await?;
    let response = create_response_path();

    Ok(response)
}

pub async fn delete_vote_from_form(
    State(mut am_database): State<Store>,
    Form(vote): Form<CreateVote>
) -> Result<Response<Body>, AppError> {
    let old_vote = CreateVote {
        post_id: vote.post_id,
        user_id: vote.user_id
    };
    am_database.delete_vote(old_vote).await?;    
    let response = create_response_path();
    Ok(response)
}

pub async fn delete_vote_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>,
) -> Result<(), AppError> {
    am_database.delete_vote_by_id(query).await?;
    Ok(())
}

pub async fn get_votes_for_post(
    State(mut am_database): State<Store>,
    query: i32,
) -> Result<Json<i64>, AppError> {
    let num_votes = am_database.get_number_of_votes_for_post(query).await?;
    Ok(Json(num_votes))
}


// NASA ----------------------------------------------------------------------------------------------------------------
pub async fn get_nasa_post_by_form(
    State(mut am_database): State<Store>,
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

// Admin ---------------------------------------------------------------------------------------------------------------
// TODO: Add redirects to these!
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