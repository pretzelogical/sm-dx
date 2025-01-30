use serde::Deserialize;

use super::client::client;

#[derive(Deserialize)]
pub struct PostResponseTag {
    pub id: i64,
    pub post_id: i64,
    pub author_id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PostResponseComment {
    pub id: i64,
    pub post_id: i64,
    pub author_id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PostResponseItem {
    pub id: i64,
    pub author_id: i64,
    pub date: i64,
    pub title: String,
    pub content: String,
    pub img: Option<String>,
    pub comment: Option<Vec<PostResponseComment>>,
    pub tag: Option<Vec<PostResponseTag>>,
}

#[derive(Deserialize)]
pub struct PostResponse(Vec<PostResponseItem>);

pub async fn get_latest_posts(token: String) -> Result<PostResponse, String> {
    match client(token).get("http://127.0.0.1:8080/post").send().await {
        Ok(res) => {
            let posts: PostResponse = res.json().await.unwrap();
            Ok(posts)
        }
        Err(error) => Err(error.to_string()),
    }
}
