use reqwest::Client;
use backend::models::post::{CreatePost, UpdatePost, PostId};  
use backend::models::vote::{CreateVote, VoteId};
use backend::models::user::{UserSignup};
use chrono::{Datelike, Timelike, Utc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    // // Create a User
    // let new_user = UserSignup {
    //     email: "mr_fake@fake.com".into(),
    //     password: "12345".into(),
    //     confirm_password: "12345".into()
    // };

    // let res = client.post("http://localhost:3000/users")
    //     .json(&new_user)
    //     .send()
    //     .await?;


    // // Create a test post
    // let test_post = CreatePost {
    //     title: "Client Test Post".into(),
    //     query_string: "this is a shitty query".into(),
    //     explanation: "Don't click this".into(),
    //     img_url: "www.google.com/somethinginappropriate".into(),
    //     apod_date: Utc::now().to_string()
    // };

    // let res = client.post("http://localhost:3000/posts")
    //     .json(&test_post)
    //     .send()
    //     .await?;
 


    // Get posts for a user
    // let res = client.get("http://localhost:3000/users/3/posts")
    //     .send()
    //     .await?;

    
    // // Get post by id
    // let res = client.get("http://localhost:3000/posts/2")
    //     .send()
    //     .await?;

    // // Get all Posts
    let res = client.get("http://localhost:3000/posts")
        .send()
        .await?;

    // // Update a post
    // let update_post = UpdatePost {
    //     id: PostId(4),
    //     title: "UPDATED NATION".into(),
    //     query_string: "QUERY MY BERRIES".into(),
    //     img_url: "www.google.com/somethingREALLYinappropriate".into(),
    //     explanation: "Okay, maybe click this ;)".into(),
    //     apod_date: "1996".into()
    // };

    // let res = client.put("http://localhost:3000/posts")
    //     .json(&update_post)
    //     .send()
    //     .await?;


    // // Vote for a post
    // let new_vote = CreateVote{
    //     post_id: PostId(4),
    //     user_id: 3
    // };
    // let res = client.post("http://localhost:3000/votes")
    //     .json(&new_vote)
    //     .send()
    //     .await?;


    // Delete a vote for a post
    // let res = client.delete("http://localhost:3000/votes/3")
    // .send()
    // .await?;

    // Delete a post
    // let res = client.delete("http://localhost:3000/posts/4")
    // .send()
    // .await?;

    let body = res.text().await?;
    println!("{}", body);
    Ok(())
}