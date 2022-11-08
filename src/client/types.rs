use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "unlisted")]
    Unlisted,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "direct")]
    Direct,
}

// https://docs.joinmastodon.org/entities/status/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Status {
    pub id: String,
    pub uri: String,
    pub created_at: String,
    pub account: Account,
    pub content: String,
    pub visibility: Visibility,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub media_attachments: Vec<Attachment>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub emojis: Vec<Emoji>,
    pub reblogs_count: u64,
    pub favourites_count: u64,
    pub replies_count: u64,
    pub url: Option<String>,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Option<Box<Status>>,
    pub poll: Option<Poll>,
    pub card: Option<Card>,
    pub language: Option<String>, // TODO: two-letter language code
    pub text: Option<String>,
    pub favourited: Option<bool>,
    pub reblogged: Option<bool>,
    pub muted: Option<bool>,
    pub bookmarked: Option<bool>,
    pub pinned: Option<bool>,
    pub server_base: Option<String>,
    pub interact_url: Option<String>,
}

// https://docs.joinmastodon.org/entities/poll/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Poll {
    pub id: String,
    pub expires_at: Option<String>,
    pub expired: bool,
    pub multiple: bool,
    pub votes_count: u64,
    pub voters_count: u64,
    pub options: Vec<PollOption>,
    pub voted: Option<bool>,
}

// https://docs.joinmastodon.org/entities/poll-option/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct PollOption {
    pub title: String,
    pub votes_count: u64,
}

// https://docs.joinmastodon.org/entities/card/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Card {
    pub url: String,
    pub title: String,
    pub description: String,
    pub image: Option<String>,
}

// https://docs.joinmastodon.org/entities/mention/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Mention {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub url: String,
}

// https://docs.joinmastodon.org/entities/tag/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

// https://docs.joinmastodon.org/entities/account/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub created_at: String,
    pub followers_count: u64,
    pub following_count: u64,
    pub statuses_count: u64,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub emojis: Vec<Emoji>,
    pub moved: Option<Box<Account>>,
    pub fields: Option<Vec<Field>>,
    pub bot: Option<bool>,
}

// https://docs.joinmastodon.org/entities/emoji/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Emoji {
    pub shortcode: String,
    pub static_url: String,
    pub url: String,
}

// https://docs.joinmastodon.org/entities/field/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum MediaType {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "gifv")]
    Gifv,
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "unknown")]
    Unknown,
}

// https://docs.joinmastodon.org/entities/attachment/
#[derive(Deserialize, Serialize, Debug, PartialEq, Props, Clone)]
pub struct Attachment {
    pub id: String,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub url: String,
    pub preview_url: String,
    pub remote_url: Option<String>,
    pub meta: Option<Meta>, // TODO: No description found
    pub description: Option<String>,
    pub blurhash: Option<String>,
    pub text_url: Option<String>, // Deprecated
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Props, Clone)]
pub struct Focus {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Props, Clone)]

pub struct Meta {
    pub original: Original,
    pub small: Small,
    pub focus: Option<Focus>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Props, Clone)]
pub struct Original {
    pub width: u64,
    pub height: u64,
    pub size: Option<String>,
    pub aspect: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Props, Clone)]
pub struct Small {
    pub width: u64,
    pub height: u64,
    pub size: String,
    pub aspect: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Props)]
pub struct Instance {
    pub uri: String,
    pub title: String,
    pub description: String,
    pub email: String,
    pub version: String,
    // pub urls: Vec<String>,
    // pub stats: Vec<String>,
    pub thumbnail: Option<String>,
    pub languages: Vec<String>,
    pub contact_account: Option<Account>,
}
