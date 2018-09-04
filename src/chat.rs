
use std::{
	vec::Vec
};

use ws;

struct Connector {
	token: String,
	nick: String,
	channels: Vec<String>,
	out: ws::Sender
}

impl ws::Handler for Connector {

	fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
		println!("[Twitch Chat] Connected to Twitch!");

		// Send the initial connection packets.
		self.out.send("PASS ".to_string() + &self.token)?;
		self.out.send("NICK ".to_string() + &self.nick)?;

		// Also send our CAP requests.
		self.out.send("CAP REQ :twitch.tv/membership")?;
		self.out.send("CAP REQ :twitch.tv/tags")?;
		self.out.send("CAP REQ :twitch.tv/commands")?;

		// Now that the initial things are out of the way, lets connect to our channels.
		for channel in &self.channels {
			self.out.send("JOIN #".to_string() + &channel)?;
		}

		Ok(())
	}

	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		let mut text: String = msg.to_string();
		text = text.trim().to_string();
		println!("{}", text);
		// Check if this is a ping packet.
		if &text == "PING :tmi.twitch.tv" {
			return self.out.send("PONG :tmi.twitch.tv");
		}
		Ok(())
	}
}

pub struct TwitchChat {
	channels: Vec<String>,
	token: String,
	nick:  String,

	handler: Option<Connector>
}

impl TwitchChat {

	pub fn new(token: &str, nick: &str, channels: Vec<String>) -> Self {
		TwitchChat {
			channels,
			token: token.to_string(),
			nick: nick.to_string(),
			handler: None
		}
	}

	pub fn connect(&mut self) -> Result<(), String> {
		let endpoint = "wss://irc-ws.chat.twitch.tv";

		println!("Connecting to: {}", endpoint);
		ws::connect(endpoint, |out| {
			Connector {
				out,

				token: self.token.clone(),
				nick: self.nick.clone(),
				channels: self.channels.clone()
			}
		}).unwrap();
		Ok(())
	}
}
