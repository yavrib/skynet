#[macro_use]
extern crate cached;

use discord::Discord;
use discord::model::*;
use std::env;
use cached::{SizedCache, Cached};
use std::hash::{Hash, Hasher};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SkyNetMsg(Message);

impl Eq for SkyNetMsg {}
impl Hash for SkyNetMsg {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.id.hash(state);
		self.0.channel_id.hash(state);
		// These are enough don't hash the [content].
		// or any other shit here
	}
}

impl PartialEq for SkyNetMsg {
	fn eq(&self, other: &Self) -> bool {
		self.0.id == other.0.id &&
			self.0.channel_id == other.0.channel_id
	}
}

cached!{
    MSG_STORE: SizedCache<SkyNetMsg, SkyNetMsg> = SizedCache::with_size(1024_usize);
    fn store(msg: SkyNetMsg) -> SkyNetMsg = { msg }
}

fn main() {
	// Log in to Discord using a bot token from the environment
	let discord = Discord::from_bot_token(
		&env::var("DISCORD_TOKEN").expect("Expected token"),
	).expect("login failed");

	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
	println!("Ready.");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				store(SkyNetMsg(message));
			}
			Ok(Event::MessageDelete { channel_id, message_id }) => {
				let mut store = MSG_STORE.lock().unwrap();

				let fake_author: User = User {
					id: UserId(123_u64),
					name: String::new(),
					discriminator: 1,
					avatar: None,
					bot: false,
				};

				let msg: SkyNetMsg = SkyNetMsg(Message {
					channel_id,
					id: message_id,

					// Useless shit because guyz didn't implement default.
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
				});

				if let Some(msg) = store.cache_get(&msg) {
					if !msg.0.author.bot {
						let sinirlendirdin_beni_ibne =
							format!("<@!{}> dedi ki:\n{}", msg.0.author.id, msg.0.content.as_str());
						let _ = discord.send_message(
							msg.0.channel_id,
							sinirlendirdin_beni_ibne.as_str(),
							"",
							false);
					}
				}
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break
			}
			Err(err) => println!("Receive error: {:?}", err)
		}
	}
}
