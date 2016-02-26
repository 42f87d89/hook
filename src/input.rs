use sdl2::Sdl;
use sdl2::event::Event;

use tickable::Tickable;

pub struct Input {
	context: Sdl,
	handlers: Vec<Box<FnMut(&Event)>>,
}

impl Input {
	pub fn new(context: Sdl) -> Self {
		Input {context: context, handlers: Vec::new()}
	}
	pub fn handle(&mut self) {
		let mut events = self.context.event_pump().unwrap();
		for event in events.poll_iter() {
            for handler in &mut self.handlers {
				handler(&event);
			}
        }
	}
	pub fn add_handler(&mut self, handler: Box<FnMut(&Event)>) {
		self.handlers.push(handler);
	}
}