use discord::model::*;
use std::hash::{Hasher, Hash};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SkyNetMsg{
    pub msg: Message,
    pub history: Vec<SkyNetMsg>
}

impl Default for SkyNetMsg {
    fn default() -> Self {
        let fake_author: User = User {
            id: UserId(123_u64),
            name: String::new(),
            discriminator: 1,
            avatar: None,
            bot: false,
        };

        SkyNetMsg {
            msg: Message {
                channel_id: ChannelId(0_u64),
                id: MessageId(0_u64),
                content: String::new(),
                nonce: None,
                tts: false,
                timestamp: String::new(),
                edited_timestamp: None,
                pinned: false,
                kind: MessageType::Regular,

                author: fake_author,
                mention_everyone: false,
                mentions: Vec::<User>::new(),
                mention_roles: Vec::<RoleId>::new(),
                reactions: Vec::<MessageReaction>::new(),

                attachments: Vec::<Attachment>::new(),
                /// Follows OEmbed standard
                embeds: Vec::<Value>::new(),
            },
            history: Vec::new()
        }
    }
}

impl Eq for SkyNetMsg {}
impl Hash for SkyNetMsg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.msg.id.hash(state);
        self.msg.channel_id.hash(state);
        // These are enough don't hash the [content].
        // or any other shit here
    }
}

impl PartialEq for SkyNetMsg {
    fn eq(&self, other: &Self) -> bool {
        self.msg.id == other.msg.id &&
            self.msg.channel_id == other.msg.channel_id
    }
}
