use ::sdl::event::{Event, Key};
use player::Player;
use tickable::Tickable;
use screen::Screen;

pub fn move_player(event: &::sdl::event::Event, player: &mut Player) {
	match event {
		&Event::Key(k, down, _, _) => {
			if down {
				if k == Key::Up {
					player.up = true;
				}
				else if k == Key::Down {
					player.down = true;
				}
				else if k == Key::Left {
					player.left = true;
				}
				else if k == Key::Right {
					player.right = true;
				}
			} else {
				if k == Key::Up {
					player.up = false;
				}
				else if k == Key::Down {
					player.down = false;
				}
				else if k == Key::Left {
					player.left = false;
				}
				else if k == Key::Right {
					player.right = false;
				}
			}
		},
		_ => {}
	}
	player.tick();
}

pub fn screen_should_end(event: &::sdl::event::Event, screen: &mut Screen) {
	match event {
			&Event::Quit => {
				screen.should_end = true;
			},
			&Event::Key(k, down, _, _) => {
				if down {
					if k == Key::Escape {
						screen.should_end = true;
					}
				}
			},
			_ => {}
		}
}