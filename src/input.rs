use sdl2::Sdl;
use sdl2::event::Event;

use tickable::Tickable;

struct Input {
	context: Sdl,
	handlers: Vec<(Event, Box<Fn()>)>,
}

impl Input {
	fn handle(&self, event: &Event) {
		for (evt, hndlr) in &self.handlers {
			if evt == *event {hndlr()};
		}
	}
}