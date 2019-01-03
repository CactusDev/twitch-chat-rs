
pub enum HandlerResult {
	Nothing,
	Error(String),
	Message(String)
}

pub trait Handler {
	fn on_connect(&mut self) -> HandlerResult { HandlerResult::Nothing }
	fn on_disconnect(&mut self) -> HandlerResult { HandlerResult::Nothing }
	fn on_reconnect(&mut self) -> HandlerResult { HandlerResult::Nothing }
	
	fn on_message(&mut self) -> HandlerResult;

	fn on_user_join(&mut self) -> HandlerResult { HandlerResult::Nothing }
	fn on_user_leave(&mut self) -> HandlerResult { HandlerResult::Nothing }
	// TODO: is there any other events that we can have?
}
