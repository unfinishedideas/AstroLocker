use argon2::Config;
use axum::extract::State;
use axum::response::Response;
use axum::Form;
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;

use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;
use crate::models::user::{Claims, User, UserSignup, KEYS};

// User ----------------------------------------------------------------------------------------------------------------
pub async fn register(
    State(database): State<Store>,
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

pub async fn login(
    State(database): State<Store>,
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
pub async fn logout() -> Result<Response<Body>, AppError> {
    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_static("jwt=; path=/; expires=Thu, 01 Jan 1970 00:00:00 UTC;"),
    );
    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    Ok(response)
}
