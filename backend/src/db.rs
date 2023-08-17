use axum::Json;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

use crate::error::AppError;
use crate::models::nasaquery::NasaQuery;
use crate::models::post::{CreatePost, Post, PostId, UpdatePost};
use crate::models::user::{User, UserSignup};
use crate::models::vote::{CreateVote, Vote, VoteId};
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

    // Users -----------------------------------------------------------------------------------------------------------
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

    pub async fn get_user_id_by_email(&self, email: String) -> Result<i32, AppError> {
        let res = sqlx::query(r#"SELECT id FROM users WHERE email=$1"#)
            .bind(email)
            .fetch_one(&self.conn_pool)
            .await?;

        let id: i32 = res.get("id");

        Ok(id)
    }

    pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
        let result =
            sqlx::query("INSERT INTO users(email, password, is_banned) values ($1, $2, false)")
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

    pub async fn determine_if_user_banned(&mut self, email: String) -> Result<bool, AppError> {
        let res = sqlx::query(r#"SELECT * FROM users WHERE email=$1"#)
            .bind(email)
            .fetch_one(&self.conn_pool)
            .await?;
        Ok(res.get("is_banned"))
    }

    pub async fn determine_if_user_admin(&mut self, email: String) -> Result<bool, AppError> {
        let res = sqlx::query(
            r#"
            SELECT * FROM users 
            INNER JOIN admins 
            ON admins.admin_user_id = users.id
            WHERE email=$1;
        "#,
        )
        .bind(email)
        .fetch_optional(&self.conn_pool)
        .await?;

        if res.is_none() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    // Posts -----------------------------------------------------------------------------------------------------------
    pub async fn get_all_posts(&mut self) -> Result<Vec<Post>, AppError> {
        let res = sqlx::query(
            r#"
            SELECT * FROM posts;
            "#,
        )
        .fetch_all(&self.conn_pool)
        .await?;

        let posts: Vec<_> = res
            .into_iter()
            .map(|row| Post {
                id: PostId(row.get("id")),
                title: row.get("title"),
                query_string: row.get("query_string"),
                explanation: row.get("explanation"),
                img_url: row.get("img_url"),
                apod_date: row.get("apod_date"),
            })
            .collect();

        Ok(posts)
    }

    pub async fn get_post_by_id(&mut self, post_id: i32) -> Result<Post, AppError> {
        let res = sqlx::query(
            r#"
            SELECT * FROM posts WHERE id=$1
            "#,
        )
        .bind(post_id)
        .fetch_one(&self.conn_pool)
        .await?;

        let post = Post {
            id: PostId(res.get("id")),
            title: res.get("title"),
            query_string: res.get("query_string"),
            explanation: res.get("explanation"),
            img_url: res.get("img_url"),
            apod_date: res.get("apod_date"),
        };

        Ok(post)
    }

    pub async fn get_top_posts(&mut self) -> Result<Vec<i32>, AppError> {
        // This query courtesy of https://www.tutorialspoint.com/count-number-of-times-value-appears-in-particular-column-in-mysql
        let res = sqlx::query(
            r#"
            SELECT post_id, COUNT(*) AS number FROM votes GROUP BY post_id ORDER BY number DESC LIMIT 10;
            "#
        )
        .fetch_all(&self.conn_pool)
        .await?;

        let posts: Vec<_> = res.into_iter().map(|row| row.get("post_id")).collect();

        Ok(posts)
    }

    pub async fn add_post(&mut self, new_post: CreatePost) -> Result<Post, AppError> {
        let res = sqlx::query(
            r#"
            INSERT INTO posts (title, query_string, explanation, img_url, apod_date) 
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(new_post.title)
        .bind(new_post.query_string)
        .bind(new_post.explanation)
        .bind(new_post.img_url)
        .bind(new_post.apod_date)
        .fetch_one(&self.conn_pool)
        .await?;

        let post = Post {
            id: PostId(res.get("id")),
            title: res.get("title"),
            query_string: res.get("query_string"),
            explanation: res.get("explanation"),
            img_url: res.get("img_url"),
            apod_date: res.get("apod_date"),
        };

        Ok(post)
    }

    pub async fn delete_post_by_id(&mut self, post_id: i32) -> Result<(), AppError> {
        sqlx::query(
            r#"
    DELETE FROM posts WHERE id = $1
    "#,
        )
        .bind(post_id)
        .execute(&self.conn_pool)
        .await
        .unwrap();

        Ok(())
    }

    pub async fn update_post_by_id(&mut self, new_post: UpdatePost) -> Result<Post, AppError> {
        sqlx::query(
            r#"
            UPDATE posts
            SET title = $1, query_string = $2, explanation = $3, img_url = $4, apod_date = $5
            WHERE id = $6
            "#,
        )
        .bind(new_post.title)
        .bind(new_post.query_string)
        .bind(new_post.explanation)
        .bind(new_post.img_url)
        .bind(new_post.apod_date)
        .bind(new_post.id.0)
        .execute(&self.conn_pool)
        .await?;

        let res = sqlx::query(
            r#"
            SELECT * FROM posts WHERE id=$1
            "#,
        )
        .bind(new_post.id.0)
        .fetch_one(&self.conn_pool)
        .await?;

        let new_post = Post {
            id: PostId(res.get("id")),
            title: res.get("title"),
            query_string: res.get("query_string"),
            explanation: res.get("explanation"),
            img_url: res.get("img_url"),
            apod_date: res.get("apod_date"),
        };

        Ok(new_post)
    }

    pub async fn get_post_by_query_string(&mut self, query: NasaQuery) -> Result<Post, AppError> {
        let res = sqlx::query(
            r#"
            SELECT * FROM posts WHERE query_string=$1
            "#,
        )
        .bind(query.query_string)
        .fetch_one(&self.conn_pool)
        .await?;

        let post = Post {
            id: PostId(res.get("id")),
            title: res.get("title"),
            query_string: res.get("query_string"),
            explanation: res.get("explanation"),
            img_url: res.get("img_url"),
            apod_date: res.get("apod_date"),
        };

        Ok(post)
    }

    pub async fn check_cache_by_query_string(
        &mut self,
        query: NasaQuery,
    ) -> Result<bool, AppError> {
        let res = sqlx::query!(
            r#"
            SELECT EXISTS ( SELECT * FROM posts WHERE query_string=$1);
            "#,
            query.query_string
        )
        .fetch_one(&self.conn_pool)
        .await?;

        println!("{:?}", res);
        if res.exists == Some(false) {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub async fn get_user_posts_by_id(&mut self, user_id: i32) -> Result<Vec<Post>, AppError> {
        let res = sqlx::query(
            r#"
            SELECT * FROM posts
            INNER JOIN votes
            ON posts.id = votes.post_id
            WHERE user_id=$1;
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.conn_pool)
        .await?;

        let posts: Vec<_> = res
            .into_iter()
            .map(|row| Post {
                id: PostId(row.get("id")),
                title: row.get("title"),
                query_string: row.get("query_string"),
                explanation: row.get("explanation"),
                img_url: row.get("img_url"),
                apod_date: row.get("apod_date"),
            })
            .collect();

        Ok(posts)
    }

    pub async fn determine_if_user_liked_post(
        &mut self,
        user_id: i32,
        post_id: i32,
    ) -> Result<bool, AppError> {
        let res = sqlx::query!(
            r#"
            SELECT * FROM votes WHERE user_id=$1 AND post_id=$2
            "#,
            user_id,
            post_id
        )
        .fetch_optional(&self.conn_pool)
        .await?;

        if res.is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // Votes -----------------------------------------------------------------------------------------------------------
    pub async fn create_vote(&mut self, new_vote: CreateVote) -> Result<Vote, AppError> {
        let res = sqlx::query(
            r#"
            INSERT INTO votes (post_id, user_id) VALUES($1, $2)
            RETURNING *
            "#,
        )
        .bind(new_vote.post_id.0)
        .bind(new_vote.user_id)
        .fetch_one(&self.conn_pool)
        .await?;

        let created_vote = Vote {
            id: VoteId(res.get("id")),
            post_id: PostId(res.get("post_id")),
            user_id: res.get("user_id"),
        };

        Ok(created_vote)
    }

    pub async fn delete_vote(&mut self, old_vote: CreateVote) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM votes WHERE user_id=$1 AND post_id=$2
            "#,
        )
        .bind(old_vote.user_id)
        .bind(old_vote.post_id.0)
        .execute(&self.conn_pool)
        .await?;

        Ok(())
    }

    pub async fn get_number_of_votes_for_post(&mut self, post_id: i32) -> Result<i64, AppError> {
        let res = sqlx::query!(
            r#"
            SELECT COUNT(posts.id) AS "num_votes" FROM
votes INNER JOIN posts ON votes.post_id = posts.id
WHERE post_id = $1;
            "#,
            post_id
        )
        // .bind(post_id)
        .fetch_one(&self.conn_pool)
        .await?;

        let mut num_votes = 0;
        if res.num_votes.is_some() {
            num_votes = res.num_votes.unwrap()
        }
        Ok(num_votes)
    }

    // Admin -----------------------------------------------------------------------------------------------------------
    pub async fn ban_user_by_email(&mut self, email_to_ban: String) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE users
            SET is_banned=true
            WHERE email = $1
            "#,
        )
        .bind(email_to_ban)
        .execute(&self.conn_pool)
        .await?;

        Ok(())
    }

    pub async fn unban_user_by_email(&mut self, email_to_ban: String) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE users
            SET is_banned=false
            WHERE email = $1
            "#,
        )
        .bind(email_to_ban)
        .execute(&self.conn_pool)
        .await?;

        Ok(())
    }

    pub async fn promote_admin_by_email(
        &mut self,
        email_to_promote: String,
    ) -> Result<(), AppError> {
        // Get UserId
        let res = sqlx::query(
            r#"
            SELECT id FROM users WHERE email=$1
            "#,
        )
        .bind(email_to_promote)
        .fetch_one(&self.conn_pool)
        .await?;

        let user_id: i32 = res.get("id");

        // Add to Admins
        sqlx::query(
            r#"
            INSERT INTO admins (admin_user_id) VALUES ($1)
            "#,
        )
        .bind(user_id)
        .execute(&self.conn_pool)
        .await?;

        Ok(())
    }

    pub async fn demote_admin_by_email(&mut self, email_to_demote: String) -> Result<(), AppError> {
        // Get UserId
        let res = sqlx::query(
            r#"
            SELECT id FROM users WHERE email=$1
            "#,
        )
        .bind(email_to_demote)
        .fetch_one(&self.conn_pool)
        .await?;

        let user_id: i32 = res.get("id");

        // Remove from Admins
        sqlx::query(
            r#"
            DELETE FROM admins WHERE admin_user_id=$1
            "#,
        )
        .bind(user_id)
        .execute(&self.conn_pool)
        .await?;

        Ok(())
    }

    // pub async fn make_user_admin()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
