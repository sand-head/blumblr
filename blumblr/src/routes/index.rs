use blumblr_core::bsky::{post::Post, BskyClient};
use serde::Serialize;
use tuono_lib::{Props, Request, Response};

#[derive(Serialize)]
struct MyResponse {
    posts: Vec<Post>,
}

#[tuono_lib::handler]
async fn get_server_side_props(_req: Request, bsky: BskyClient) -> Response {
    let posts = bsky.get_posts().await;
    let posts = match posts {
        Ok(data) => data,
        Err(_) => panic!("Could not acquire posts from author"),
    };
    Response::Props(Props::new(MyResponse { posts }))
}
