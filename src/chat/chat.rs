
use std::{
	vec::Vec,
	collections::HashMap
};

use chat::handler::{Handler, HandlerResult};
use websocket::{
	client::{
		ClientBuilder,
		sync::Client
	},
	stream::sync::{
		TlsStream, TcpStream
	},
	OwnedMessage
};

const ADDRESS: &'static str = "wss://irc-ws.chat.twitch.tv";

fn handle_handler_result(result: HandlerResult, chat: &mut TwitchChat) -> Result<(), String> {
	match result {
		HandlerResult::Nothing => {},
		HandlerResult::Error(e) => println!("An internal handler error has occurred: {}", e),
		HandlerResult::Message(_message) => {} // chat.send_message(&message)?
	}
	Ok(())
}

/// Connect to Twitch's chat and listen for messages
pub struct TwitchChat {
	channels: Vec<String>,

	handler: Box<Handler>,
	client: Client<TlsStream<TcpStream>>
}

fn create_client() -> Result<Client<TlsStream<TcpStream>>, String> {
	println!("Connecting to {}", ADDRESS);
	let client = ClientBuilder::new(ADDRESS)
		.unwrap()
		.add_protocol("rust-websocket")
		.connect_secure(None)
		.unwrap();
	println!("Connected to Twitch!");

	Ok(client)
}

impl TwitchChat {

	pub fn connect(handler: Box<Handler>) -> Result<Self, String> {
		let client = create_client()?;

		Ok(TwitchChat {
			client,
			channels: Vec::new(),
			handler
		})
	}

	pub fn send_packet(&mut self, message: OwnedMessage) -> Result<(), String> {
		self.client.send_message(&message).map_err(|_| "could not send packet".to_string())?;
		Ok(())
	}

	pub fn authenticate(&mut self, token: &str, nick: &str) -> Result<(), String> {
		self.send_packet(OwnedMessage::Text(format!("PASS {}", token)))?;
		self.send_packet(OwnedMessage::Text(format!("NICK {}", nick)))?;
		Ok(())
	}

	pub fn send_cap(&mut self, cap: &str) -> Result<(), String> {
		self.send_packet(OwnedMessage::Text(format!("CAP REQ :{}", cap)))?;
		Ok(())
	}

	pub fn join(&mut self, channels: Vec<String>) -> Result<(), String> {
		for channel in &channels {
			self.send_packet(OwnedMessage::Text(format!("JOIN #{}", &channel)))?;
		}

		Ok(())
	}

	pub fn handle_chat(mut self) -> Result<(), String> {
		let result = self.handler.on_connect();
		handle_handler_result(result, &mut self)?;

		while let Ok(packet) = self.client.recv_message() {
			match packet {
				OwnedMessage::Ping(packet) => self.client.send_message(&OwnedMessage::Ping(packet)).unwrap(),
				OwnedMessage::Text(text) => {
					// Check if this is a Twitch-style ping packet
					if text == "PING :tmi.twitch.tv" {
						self.send_packet(OwnedMessage::Text("PONG :tmi.twitch.tv".into()))?;
						continue;
					}
					println!("{}", text);
				},
				OwnedMessage::Close(_) => return Ok(()),
				_ => println!("Unhandled packet type!")
			};
		}
		Ok(())
	}
}
