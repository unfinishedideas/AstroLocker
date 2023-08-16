// use serde_json::Value;

// use crate::models::{nasaquery::NasaQuery, post::CreatePost};


// pub async fn call_nasa(query: NasaQuery) -> CreatePost {
//     let client = reqwest::Client::new();
//     let key = std::env::var("NASA_API_KEY").unwrap();
//     let query_string = query.query_string;
//     let nasa_query = format!("GET https://api.nasa.gov/planetary/apod/?date={query_string}?api_key={key}");
//     let res = client.get(nasa_query)
//         .send()
//         .await;
//     let body = res.text().await?;
//     let parsed_json: Value = serde_json::from_str(&body).expect("Something went wrong parsing JSON!");
    
//     println!("{}", &body);    // REMOVE THIS <=====================================================================
    
//     let post_to_add = CreatePost {
//         title: parsed_json["title"].to_string(),
//         query_string: query_string,
//         explanation: parsed_json["explanation"].to_string(),
//         img_url: parsed_json["url"].to_string(),
//         apod_date: parsed_json["date"].to_string()
//     };
//     post_to_add
// } 