use std::error::Error;

pub mod post;

use bsky_sdk::{
    api::{
        app::bsky::feed::{
            defs::ThreadViewPostParentRefs,
            get_author_feed,
            get_post_thread::{self, OutputThreadRefs},
        },
        types::{
            string::{AtIdentifier, Did},
            LimitedU16, Union,
        },
    },
    BskyAgent,
};
use post::Post;

fn unnest_post_thread(thread: Union<OutputThreadRefs>) -> Vec<Post> {
    let mut posts = Vec::new();

    if let Union::Refs(thread_ref) = thread {
        if let OutputThreadRefs::AppBskyFeedDefsThreadViewPost(post_data) = thread_ref {
            if let Some(parent) = &post_data.parent {
                posts.append(&mut unnest_post_thread_parents(parent));
            }
        }
    }

    posts
}

fn unnest_post_thread_parents(parent: &Union<ThreadViewPostParentRefs>) -> Vec<Post> {
    let mut posts = Vec::new();

    if let Union::Refs(parent_ref) = parent {
        if let ThreadViewPostParentRefs::ThreadViewPost(post_data) = parent_ref {
            if let Some(parent) = &post_data.parent {
                posts.append(&mut unnest_post_thread_parents(parent));
            }

            posts.push(Post::from_bsky(&post_data.post));
        }
    }

    posts
}

#[derive(Clone)]
pub struct BskyClient {
    agent: BskyAgent,
    did: Did,
}
impl BskyClient {
    pub async fn init(user: String, password: String) -> Result<Self, Box<dyn Error>> {
        let agent = BskyAgent::builder().build().await?;
        let session = agent.login(user, password).await?;

        Ok(Self {
            agent,
            did: session.did.clone(),
        })
    }

    pub async fn get_posts(&self) -> Result<Vec<Post>, Box<dyn Error>> {
        let feed = self
            .agent
            .api
            .app
            .bsky
            .feed
            .get_author_feed(
                get_author_feed::ParametersData {
                    actor: AtIdentifier::Did(self.did.clone()),
                    cursor: None,
                    filter: Some("posts_with_replies".to_string()),
                    limit: Some(25.try_into().unwrap()),
                    include_pins: Some(true),
                }
                .into(),
            )
            .await?;

        let json = serde_json::to_string_pretty(&feed.data.feed)?;
        println!("{}", json);

        Ok(
            futures::future::join_all(feed.data.feed.iter().map(|p| async {
                let thread = self.get_post_thread(p.post.uri.clone()).await.unwrap();
                let mut post = Post::from_bsky(&p.post);
                post.thread = Some(thread);

                post
            }))
            .await,
        )
    }

    async fn get_post_thread(&self, uri: String) -> Result<Vec<Post>, Box<dyn Error>> {
        let thread = self
            .agent
            .api
            .app
            .bsky
            .feed
            .get_post_thread(
                get_post_thread::ParametersData {
                    depth: Some(LimitedU16::try_from(0).unwrap()),
                    parent_height: None,
                    uri,
                }
                .into(),
            )
            .await?;

        let json = serde_json::to_string_pretty(&thread.data.thread)?;
        println!("{}", json);

        Ok(unnest_post_thread(thread.data.thread))
    }
}
