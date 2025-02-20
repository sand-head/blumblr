use bsky_sdk::api::{
    app::bsky::feed::{
        defs::{PostViewData, PostViewEmbedRefs},
        post::RecordData,
    },
    types::{Object, TryFromUnknown, Union},
};
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
    pub thread: Option<Vec<Post>>,
}
impl Post {
    pub fn from_bsky(bsky_post: &Object<PostViewData>) -> Post {
        let record = match RecordData::try_from_unknown(bsky_post.record.clone()) {
            Ok(record) => record,
            Err(_) => panic!("Could not deserialize record data"),
        };
        Post {
            author: PostAuthor {
                display_name: bsky_post.author.display_name.clone(),
                user_name: bsky_post.author.handle.to_string(),
                avatar: bsky_post.author.avatar.clone(),
            },
            text: record.text,
            replies: bsky_post.reply_count.unwrap_or(0),
            likes: bsky_post.like_count.unwrap_or(0),
            reposts: bsky_post.repost_count.unwrap_or(0),
            embed: match &bsky_post.embed {
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
            thread: None,
        }
    }
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
