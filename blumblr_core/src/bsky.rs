use std::error::Error;

pub mod post;

use bsky_sdk::{
    api::{
        app::bsky::{
            embed,
            feed::{defs::PostViewEmbedRefs, get_author_feed, get_post_thread, post::RecordData},
        },
        types::{
            string::{AtIdentifier, Did},
            LimitedU16, TryFromUnknown, Union,
        },
    },
    BskyAgent,
};
use post::{Post, PostAuthor, PostEmbed, PostImage};

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
                let record = match RecordData::try_from_unknown(p.post.record.clone()) {
                    Ok(record) => record,
                    Err(_) => panic!("Could not deserialize record data"),
                };
                let _ = self.get_post_thread(p.post.uri.clone()).await;
                Post {
                    author: PostAuthor {
                        display_name: p.post.author.display_name.clone(),
                        user_name: p.post.author.handle.to_string(),
                        avatar: p.post.author.avatar.clone(),
                    },
                    text: record.text,
                    replies: p.post.reply_count.unwrap_or(0),
                    likes: p.post.like_count.unwrap_or(0),
                    reposts: p.post.repost_count.unwrap_or(0),
                    embed: match &p.post.embed {
                        Some(embed) => match embed {
                            Union::Refs(embed) => match embed {
                                PostViewEmbedRefs::AppBskyEmbedImagesView(object) => {
                                    Some(PostEmbed::Images {
                                        images: object
                                            .images
                                            .iter()
                                            .map(|i| PostImage {
                                                src: i.fullsize.clone(),
                                                alt_text: Some(i.alt.clone()),
                                            })
                                            .collect(),
                                    })
                                }
                                PostViewEmbedRefs::AppBskyEmbedVideoView(object) => None,
                                PostViewEmbedRefs::AppBskyEmbedExternalView(object) => None,
                                PostViewEmbedRefs::AppBskyEmbedRecordView(object) => None,
                                PostViewEmbedRefs::AppBskyEmbedRecordWithMediaView(object) => None,
                            },
                            Union::Unknown(_) => None,
                        },
                        None => None,
                    },
                }
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

        Ok(Vec::new())
    }
}
