use vect::Vect;

struct PlayerInput {
    up_key: bool,
    down_key: bool,
    left_key: bool,
    right_key: bool,
    jump_key: bool,
    left_mouse: bool,
    right_mouse: bool,
    mouse_position: Vect,
}

fn asd(evt: , player: ) {
        let player = player.borrow_mut();
        match evt {
            &Event::KeyDown {keycode: Some(keycode), ..} => {
                let mut ver = 0;
                let mut hor = 0;
                if keycode == Keycode::W {
                    ver += -1;
                } else if keycode == Keycode::S {
                    ver += 1;
                } else if keycode == Keycode::A {
                    hor += -1;
                } else if keycode == Keycode::D {
                    hor += 1;
                }
                player.hor.set(hor);
                player.ver.set(ver);
            }
            &Event::KeyUp {keycode: Some(keycode), ..} => { // Might cause 1 frame of lag when letting go of one direction, then immediately pressing another
                if keycode == Keycode::W || keycode == Keycode::S {
                    player.ver.set(0);
                } else if keycode == Keycode::A || keycode == Keycode::D {
                    player.hor.set(0);
                }
            }
            _ => {}
        }
    })