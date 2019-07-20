#[macro_use]
extern crate cached;

pub mod meme_loader;

use discord::Discord;
use discord::model::*;
use std::env;
use cached::{SizedCache, Cached};
use std::hash::{Hash, Hasher};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use rand::Rng;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, Clone)]
pub struct SkyNetMsg{
	msg: Message,
	history: Vec<SkyNetMsg>
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

cached!{
    MSG_STORE: SizedCache<SkyNetMsg, SkyNetMsg> = SizedCache::with_size(65536_usize);
    // STALKED_USER: SizedCache<User, User> = SizedCache::with_size(1024_usize);
    // fn stalk(user: User) -> User { user };
    fn store(msg: SkyNetMsg) -> SkyNetMsg = { msg }
}

const PREFIX: &str = "!skynet";

fn main() {
	let mut memes = HashMap::new();
	memes = meme_loader::load(memes);
	// Log in to Discord using a bot token from the environment
	let discord = Discord::from_bot_token(
		&env::var("DISCORD_TOKEN").expect("Expected token"),
	).expect("login failed");

	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
	println!("Ready.");
//	discord.send_message(message.channel_id, "Ben geldim ibneler...", "", false);
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("Message create event received");
				println!("------Message Create Event START------");
				println!("{:?}", message);
				println!("------Message Create Event END------");
				if !message.content.starts_with(PREFIX) {
					println!("{:?}", message);
					if message.content.to_uppercase().starts_with("YAPAY ZEKA DEVREYE") {
						discord.send_message(message.channel_id, "tamam abi", "", false);
					} else if message.content.starts_with("09") {
						discord.send_message(message.channel_id, "HADİ SİK ONU GÖTÜNDEN", "", false);
					}
					store(SkyNetMsg{
						msg: Message {
							..message
						},
						..Default::default()
					});
				} else {
					// You can send command like !skynet stalk @someone
					println!("{:?}", message);
					match &message.content {
						command if command.starts_with(format!("{} stalk", PREFIX).as_str()) => {
							print!("stalking");
						},
						command if command.starts_with(format!("{} say", PREFIX).as_str()) => {
							discord.delete_message(message.channel_id, message.id);
							let sentence = message.content.clone().split_off(format!("{} say ", PREFIX).len());
							let _ = discord.send_message(message.channel_id, sentence.as_str(), "", false);
						},
						command if command.starts_with(format!("{} rulet", PREFIX).as_str()) => {
							let sentence = message.content
								.clone()
								.split_off(format!("{} rulet ", PREFIX).len());

							let channel = discord.get_channel(message.channel_id).unwrap();
							println!("CHANNEL {:?}", channel);

//							discord.kick_member()
							let _ = discord.send_message(
								message.channel_id,
								sentence.as_str(),
								"",
								false);
						},
						command if command.starts_with(format!("{} meme", PREFIX).as_str()) => {
							discord.delete_message(message.channel_id, message.id);
							let sentence = message.content.clone().split_off(format!("{} meme ", PREFIX).len());
							let pf = memes.get(sentence.as_str());
							let meme_list = pf.unwrap();
							let num = rand::thread_rng().gen_range(0, meme_list.len());
							let file_to_send = meme_list.get(num).unwrap();
							let fe = file_to_send.file_name().unwrap().to_str().unwrap();
							let file = File::open(file_to_send).unwrap();
							let _ = discord.send_file(message.channel_id, "", file, fe);
						},
						command if command.starts_with(format!("{} help", PREFIX).as_str()) => {
							let _ = discord.send_message(message.channel_id, "You can use stalk command", "", false);
						},
						rest => {
							let msg = format!("Unknown command {}, type !skynet help to see help", rest);
							let _ = discord.send_message(message.channel_id, msg.as_str(), "", false);
						},
					};
				}
			}
			Ok(Event::MessageUpdate { id, kind, content, nonce, author, mentions, mention_roles, channel_id, .. }) => {
				let mut def = SkyNetMsg::default();
				def.msg.id = id;
				def.msg.channel_id = channel_id;

				let mut kache = MSG_STORE.lock().unwrap();
				let mut optional_msg = kache.cache_get(&def).cloned();

				let updated_message =
					discord.get_message(channel_id, id);

				println!("------Message Update Event START------");
				println!("{:?}", updated_message);
				println!("------Message Update Event END------");

				if let Some(msg) = optional_msg.clone() {
					{
						kache.cache_remove(&def);
					}
					def.history.push(msg.clone());
					def.msg = updated_message.unwrap();
					let mut history_clone = def.history.clone();
					let defclone = def.clone();
					drop(kache);
					store(defclone);

					if !msg.msg.author.bot && msg.msg.content != def.msg.content {
						history_clone.push(msg.clone());
						let mesajlar = history_clone
							.iter()
							.map(|m| format!("{}", m.msg.content.as_str()))
							.collect::<Vec<String>>()
							.join("\n");


						let sinirlendirdin_beni_ibne =
							format!("<@!{}> editleyip dedi ki:\n{}", msg.msg.author.id, mesajlar);
						let _ = discord.send_message(
							msg.msg.channel_id,
							sinirlendirdin_beni_ibne.as_str(),
							"",
							false);
					}
				}
			},
			Ok(Event::MessageDelete { channel_id, message_id }) => {
				println!("Message delete event received");
				println!("------Message Create Event START------");
				println!("{:?}", message_id);
				let mut kache = MSG_STORE.lock().unwrap();

				let fake_author: User = User {
					id: UserId(123_u64),
					name: String::new(),
					discriminator: 1,
					avatar: None,
					bot: false,
				};

				let mut msg = SkyNetMsg::default();
				msg.msg.channel_id = channel_id;
				msg.msg.id = message_id;

				if let Some(msg) = kache.cache_get(&msg) {
					println!("Message Content");
					println!("{:?}", msg);
					println!("------Message Create Event END------");

					if !msg.msg.author.bot {
						let sinirlendirdin_beni_ibne =
							format!("<@!{}> dedi ki:\n{}", msg.msg.author.id, msg.msg.content.as_str());
						let _ = discord.send_message(
							msg.msg.channel_id,
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
