use super::*;

use std::string;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type Id = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportedJson {
    pub guild: Guild,
    pub channel: Channel,
    pub messages: Vec<Message>,
    #[serde(rename = "dateRange")]
    pub date_range: DateRange,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: Id,
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
    GuildMemberJoin,
    #[serde(rename = "20")]
    _20,
    ThreadCreated,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub discriminator: String,
    pub nickname: String,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment {
    pub id: Id,
    pub url: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSizeBytes")]
    pub file_size_bytes: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Embed {
    pub title: String,
    pub url: Option<String>,
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
    pub id: Id,
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
    pub id: Id,
    pub name: String,
    #[serde(rename = "isAnimated")]
    pub is_animated: bool,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    #[serde(rename = "messageId")]
    pub message_id: Option<Id>,
    #[serde(rename = "channelId")]
    pub channel_id: Id,
    #[serde(rename = "guildId")]
    pub guild_id: Option<Id>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Guild {
    pub id: Id,
    pub name: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Channel {
    pub id: Id,
    #[serde(rename = "type")]
    pub ctype: ChannelType,
    #[serde(rename = "categoryId")]
    pub category_id: Id,
    pub category: String,
    pub name: String,
    pub topic: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ChannelType {
    DirectGroupTextChat,
    GuildTextChat,
    DirectTextChat,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DateRange {
    pub after: Option<DateTime<Utc>>,
    pub before: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Does the model deserialize without error? Tested on a medium-sized group DM.
    fn group_1() {
        json::read_export(
            "Direct Messages - Group - 1600s need not apply [1032681951666122772].json",
        )
        .unwrap();
    }

    #[test]
    /// Does the model deserialize without error? Tested on a large server general channel.
    fn server_1() {
        json::read_export("VRChess club - General Area - general [793007680939098124].json")
            .unwrap();
    }

    #[test]
    /// Does the model deserialize without error? Tested on a medium server channel.
    fn server_2() {
        json::read_export(
            "ball. (working title) - Voice Channel - mari-self-talk [998767292798210048].json",
        )
        .unwrap();
    }

    #[test]
    /// Does the model deserialize without error? Tested on a medium bot command channel.
    fn server_3() {
        json::read_export("VRChess club - Voice Channels - bot-commands [989779220651712522].json")
            .unwrap();
    }

    #[test]
    /// Does the model deserialize without error? Tested on a huge DM.
    fn dm_1() {
        json::read_export("Direct Messages - Private - Mad as a Nutter [367668636959244289].json")
            .unwrap();
    }

    #[test]
    /// Does the model deserialize without error? Tested on a medium DM.
    fn dm_2() {
        json::read_export("Direct Messages - Private - mint [730391742418911272].json").unwrap();
    }
}
