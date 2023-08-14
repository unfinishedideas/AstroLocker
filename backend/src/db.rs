use axum::Json;
use serde_json::Value;
use std::sync::{Arc, Mutex, RwLock};

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::info;

use crate::error::AppError;
use crate::models::post::{Post, PostId};
use crate::models::user::{User, UserSignup};
// use crate::models::answer::{Answer, AnswerId};
// use crate::models::comment::{Comment, CommentId, CommentReference};
// use crate::models::page::{AnswerWithComments, PagePackage, QuestionWithComments};
// use crate::models::question::{
//     GetQuestionById, IntoQuestionId, Question, QuestionId, UpdateQuestion,
// };

#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
    pub posts: Arc<Mutex<Vec<Post>>>,
}

pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

impl Store {
    pub fn with_pool(pool: PgPool) -> Self {
        Self {
            conn_pool: pool,
            posts: Default::default(),
        }
    }

    // pub async fn test_database(&self) -> Result<(), sqlx::Error> {
    //     let row: (i64,) = sqlx::query_as("SELECT $1")
    //         .bind(150_i64)
    //         .fetch_one(&self.conn_pool)
    //         .await?;

    //     info!("{}", &row.0);

    //     assert_eq!(row.0, 150);
    //     Ok(())
    // }

    // Users
        pub async fn get_user(&self, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT email, password FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(&self.conn_pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
        // TODO: Encrypt/bcrypt user passwords
        let result = sqlx::query("INSERT INTO users(email, password) values ($1, $2)")
            .bind(&user.email)
            .bind(&user.password)
            .execute(&self.conn_pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if result.rows_affected() < 1 {
            Err(AppError::InternalServerError)
        } else {
            Ok(Json(
                serde_json::json!({"message": "User created successfully!"}),
            ))
        }
    }

    // Posts
    pub async fn add_post(
        &mut self,
        title: String,
        img_url: String,
        explanation: String,
        user_id: i32
    ) -> Result<Post, AppError> {      
        let res = sqlx::query(
            r#"
            INSERT INTO posts (title, explanation, img_url, user_id) 
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
            .bind(title)
            .bind(explanation)
            .bind(img_url)
            .bind(user_id)
            .fetch_one(&self.conn_pool)
            .await?;

        let post = Post {
            id: PostId(res.get("id")),
            title: res.get("title"),
            explanation: res.get("explanation"),
            img_url: res.get("img_url"),
            user_id: res.get("user_id")
        };

        Ok(post)
    }

    pub async fn get_user_posts_by_id(
        &mut self,
        user_id: i32
    ) -> Result<Vec<Post>, AppError> {
        let res = sqlx::query(
            r#"
            SELECT * FROM posts WHERE user_id=$1
            "#
        )
        .bind(user_id)
        .fetch_all(&self.conn_pool)
        .await?;

        let posts: Vec<_> = res
        .into_iter()
        .map(|row| {
            Post{
                id: PostId(row.get("id")),
                title: row.get("title"),
                explanation: row.get("explanation"),
                img_url: row.get("img_url"),
                user_id: row.get("user_id")
                // id: row.id.into(),
                // title: row.title,
                // explanation: row.explanation,
                // img_url: row.img_url,
                // user_id: row.user_id
            }
        })
        .collect();

        Ok(posts)
    }

    // Upvotes

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
