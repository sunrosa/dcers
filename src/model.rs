use std::string;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub struct ExportedJson {
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: String,
    #[serde(rename = "type")]
    pub mtype: MessageType,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "timestampEdited")]
    pub timestamp_edited: Option<DateTime<Utc>>,
    #[serde(rename = "callEndedTimestamp")]
    pub call_ended_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    pub content: String,
    pub author: User,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub stickers: Vec<Sticker>,
    pub reactions: Vec<Reaction>,
    pub mentions: Vec<User>,
    pub reference: Option<Reference>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageType {
    Default,
    RecipientAdd,
    RecipientRemove,
    Call,
    ChannelIconChange,
    ChannelNameChange,
    Reply,
    ChannelPinnedMessage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub discriminator: String,
    pub nickname: String,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment {
    pub id: String,
    pub url: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSizeBytes")]
    pub file_size_bytes: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Embed {
    pub title: String,
    pub url: String,
    pub timestamp: Option<DateTime<Utc>>,
    pub description: String,
    pub color: Option<String>, /* TODO: Change this to something typed */
    pub author: Option<EmbedAuthor>,
    pub thumbnail: Option<Thumbnail>,
    pub images: Vec<Image>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    #[serde(rename = "isInline")]
    pub is_inline: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sticker {
    pub id: String,
    pub name: String,
    pub format: String,
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reaction {
    pub emoji: Emoji,
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Emoji {
    pub id: String,
    pub name: String,
    #[serde(rename = "isAnimated")]
    pub is_animated: bool,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "guildId")]
    pub guild_id: Option<String>,
}
