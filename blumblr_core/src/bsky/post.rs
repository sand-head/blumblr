use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Post {
    pub author: PostAuthor,
    pub text: String,
    pub replies: i64,
    pub likes: i64,
    pub reposts: i64,
    pub embed: Option<PostEmbed>,
}

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PostAuthor {
    pub display_name: Option<String>,
    pub user_name: String,
    pub avatar: Option<String>,
}

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase", tag = "$type")]
#[ts(export)]
pub enum PostEmbed {
    Images { images: Vec<PostImage> },
}

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PostImage {
    pub src: String,
    pub alt_text: Option<String>,
}
