use reqwest::Client;
use backend::models::post::CreatePost;  

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

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
    let res = client.get("http://localhost:3000/users/1/posts")
        .send()
        .await?;
    
    let body = res.text().await?;
    println!("{}", body);
    Ok(())
}