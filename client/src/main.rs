use reqwest::Client;
use backend::models::post::{CreatePost, UpdatePost, PostId};  

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();


    // Create a test post
    // let test_post = CreatePost {
    //     title: "Client Test Post".into(),
    //     img_url: "www.google.com/somethinginappropriate".into(),
    //     explanation: "Don't click this".into(),
    //     user_id: 1
    // };

    // let res = client.post("http://localhost:3000/posts")
    //     .json(&test_post)
    //     .send()
    //     .await?;
 


    // Get posts for a user
    // let res = client.get("http://localhost:3000/users/1/posts")
    //     .send()
    //     .await?;

    
    // Get post by id
    // let res = client.get("http://localhost:3000/posts/2")
    //     .send()
    //     .await?;

    // Get all Posts
    // let res = client.get("http://localhost:3000/posts")
    //     .send()
    //     .await?;

    // Delete a post
    // let res = client.delete("http://localhost:3000/posts/2")
    // .send()
    // .await?;

    // Update a post
    let update_post = UpdatePost {
        id: PostId(4),
        title: "Client Test Post BUT UPDATED".into(),
        img_url: "www.google.com/somethingREALLYinappropriate".into(),
        explanation: "Okay, maybe click this ;)".into(),
        user_id: 1
    };

    let res = client.put("http://localhost:3000/posts")
        .json(&update_post)
        .send()
        .await?;


    let body = res.text().await?;
    println!("{}", body);
    Ok(())
}