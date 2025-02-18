use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub author: PostAuthor,
    pub text: String,
    pub replies: i64,
    pub likes: i64,
    pub reposts: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAuthor {
    pub display_name: Option<String>,
    pub user_name: String,
    pub avatar: Option<String>,
}
