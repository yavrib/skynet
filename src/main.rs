#[macro_use]
extern crate cached;

use std::error::Error;
use std::io::Read;

pub mod meme_loader;
pub mod skynetmsg;
pub mod skynetusr;
pub mod hypers;

use discord::Discord;
use discord::model::*;
use discord::builders::*;
use std::env;
use cached::{SizedCache, TimedCache, Cached};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::rc::Rc;
use std::cell::RefCell;
use skynetmsg::SkyNetMsg;
use skynetusr::SkyNetUsr;
use rand::seq::SliceRandom;
use bastion::bastion::Bastion;
use bastion::context::BastionContext;
use bastion::child::{Message as BMessage} ;
use bastion::supervisor::SupervisionStrategy;
use crate::hypers::HyperUsers;

cached!{
    EMBESIL_STORE: TimedCache<SkyNetUsr, SkyNetUsr> = TimedCache::with_lifespan(10_u64);
    fn embesil(usr: SkyNetUsr) -> SkyNetUsr = { usr }
}

cached!{
    MSG_STORE: SizedCache<SkyNetMsg, SkyNetMsg> = SizedCache::with_size(65536_usize);
    // STALKED_USER: SizedCache<User, User> = SizedCache::with_size(1024_usize);
    // fn stalk(user: User) -> User { user };
    fn store(msg: SkyNetMsg) -> SkyNetMsg = { msg }
}

cached!{
    BOT_MSG_STORE: SizedCache<SkyNetMsg, SkyNetMsg> = SizedCache::with_size(65536_usize);
    // STALKED_USER: SizedCache<User, User> = SizedCache::with_size(1024_usize);
    // fn stalk(user: User) -> User { user };
    fn store_bot_message(msg: SkyNetMsg) -> SkyNetMsg = { msg }
}

const PREFIX: &str = "!skynet";

fn main() {
	Bastion::platform();

	let scale = 1;

	Bastion::supervisor("skynet", "system")
		.strategy(SupervisionStrategy::OneForOne)
		.children(|context: BastionContext, msg: Box<dyn BMessage>| {
			let mut file = File::open("hyperusers.json").unwrap();
			let mut contents = String::new();
			file.read_to_string(&mut contents).unwrap();

			let hypers: HyperUsers = HyperUsers {
				hypers: vec!["548224010916331520", "243377404125380608"]
			};
			let msgsem = AtomicUsize::new(0);

			client_implementation(hypers, msgsem);

			// Rebind to the system
			context.hook();
		}, "", scale)
		.launch();

	Bastion::start()
}

fn client_implementation(hypers: HyperUsers, msgsem: AtomicUsize) {
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
					} else if message.content.to_lowercase().contains("ironi") ||
						message.content.to_uppercase().contains("IRONI") {
						if !message.author.bot {
							discord.send_message(message.channel_id, "Evet ironik bir malsın", "", false);
						}
					}

					match message.author.bot {
						false => store(SkyNetMsg{
							msg: Message {
								..message
							},
							..Default::default()
						}),
						true => store_bot_message(SkyNetMsg{
							msg: Message {
								..message
							},
							..Default::default()
						}),
					};
				} else {
					// You can send command like !skynet stalk @someone
					println!("INCOMING {:?}", message);
					match &message.content {
						command if command.starts_with(format!("{} stalk", PREFIX).as_str()) => {
							println!("stalking");
						},
						command if command.starts_with(format!("{} cleanmebeybi", PREFIX).as_str()) => {
							println!("cleaning");
							let mut kache = BOT_MSG_STORE.lock().unwrap();
							println!("HYPERS {:?}", hypers);
							if hypers.hypers.contains(&message.author.id.0.to_string().as_str()) {
								for m in kache.key_order() {
									if m.msg.author.bot {
										msgsem.store(m.msg.id.0 as usize, Ordering::Relaxed);
										let _ = discord.delete_message(m.msg.channel_id, m.msg.id);
									}
								}
							} else {
								discord.send_message(message.channel_id,
													 format!("You have no power here <@!{}> ( ︶︿︶)_╭∩╮", message.author.id).as_str(),
													 "",
													 false
								);
							}
						},
						command if command.starts_with(format!("{} say", PREFIX).as_str()) => {
							discord.delete_message(message.channel_id, message.id);
							let sentence = message.content.clone().split_off(format!("{} say ", PREFIX).len());
							let _ = discord.send_message(message.channel_id, sentence.as_str(), "", false);
						},
						command if command.starts_with(format!("{} rulet", PREFIX).as_str()) => {
							if hypers.hypers.contains(&message.author.id.0.to_string().as_str()) {
								let mut kache = MSG_STORE.lock().unwrap();
								let mut def = SkyNetMsg::default();
								def.msg.id = message.id;
								def.msg.channel_id = message.channel_id;
								let mut possible_users = HashSet::new();
								for i in kache.key_order() {
									possible_users.insert(SkyNetUsr { usr: i.msg.author.clone() });
								}

								let mut rng = rand::thread_rng();
								let mut peeps = Vec::new();
								peeps.extend(possible_users.into_iter());
								peeps.shuffle(&mut rng);
								let lucky_bastard: SkyNetUsr = peeps.pop().unwrap();

								let servers = discord.get_servers().unwrap();
								// one and only one
								let server_info = servers.first().unwrap();

								discord.send_message(message.channel_id,
													 format!("Günün şanslısı sen seçildin <@!{}> ⊂(◉‿◉)つ", lucky_bastard.usr.id).as_str(),
													 "",
													 false
								);

								// ezik
								discord.edit_member(server_info.clone().id,
													lucky_bastard.usr.id,
													|em| {
														EditMember::nickname(em, "EZIK")
													}
								);

								// sagir
								discord.edit_member(server_info.clone().id,
													lucky_bastard.usr.id,
													|em| {
														EditMember::deaf(em, true)
													}
								);
							} else {
								discord.send_message(message.channel_id,
													 format!("You have no power here <@!{}> ( ︶︿︶)_╭∩╮", message.author.id).as_str(),
													 "",
													 false
								);
							}
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
				println!("------Message Delete Event START------");
				println!("{:?}", message_id);
				let mut kache = MSG_STORE.lock().unwrap();
				let mut bot_cache = BOT_MSG_STORE.lock().unwrap();

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

				if let Some(deleted_bot_message) = bot_cache.cache_get(&msg) {
					println!("Deleted Bot Message Content");
					println!("{:?}", deleted_bot_message);
					println!("------Message Delete Event END------");
					if msgsem.load(Ordering::Relaxed) == deleted_bot_message.msg.id.0 as usize {
						discord.send_message(
							deleted_bot_message.msg.channel_id,
							deleted_bot_message.msg.content.as_str(),
							"",
							false
						);
					}
				}

				if let Some(msg) = kache.cache_get(&msg) {
					println!("Message Content");
					println!("{:?}", msg);
					println!("------Message Delete Event END------");

					if !msg.msg.author.bot {
						let mut es = EMBESIL_STORE.lock().unwrap();

						// insert the embesil over here
						let embesil_user = SkyNetUsr {
							usr: msg.clone().msg.author
						};

						if let Some(dumb_fuck) = es.cache_get(&embesil_user.clone()) {
							let priv_chan =
								discord.create_private_channel(dumb_fuck.clone().usr.id).unwrap();

							for _ in 1..=10 {
								discord.send_message(
									priv_chan.id,
									"Silme Hayvan!",
									"",
									false
								);
							}
						}

						drop(es);
						embesil(embesil_user);

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
