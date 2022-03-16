use macroquad::{prelude::*, rand::gen_range, ui::root_ui};

mod ai;
mod bounds;
mod constants;
mod game;
mod physics;
mod player;

use crate::{
    constants::*,
    game::Game,
    physics::{ColliderType, GameObject},
    player::{ControlType, Player, PlayerPosition, UserType},
};

#[macroquad::main(window_conf)]
async fn main() {
    println!("_________New game_________");

    rand::srand(macroquad::miniquad::date::now() as _);

    let mut game_paused = false;    
    let mut game = Game::new();
    
    game.add_player(UserType::Ai(SARAH), PlayerPosition::Right);
    // game.add_player(UserType::Ai(SARAH), PlayerPosition::Left);
    game.add_player(UserType::Client(ControlType::Mouse), PlayerPosition::Left);
    // game.add_player(UserType::Client(ControlType::Keyboard(KeyCode::W, KeyCode::S)), PlayerPosition::Left);

    loop {
        let frame_time = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            game_paused ^= true;
        }

        if !game_paused {
            game.update(frame_time);
            game.show();
        } else {
            set_cursor_grab(false);
            show_mouse(true);
            let resume_button =
                root_ui().button(vec2(screen_width() / 2.0, screen_height() / 2.0), "Resume");
            if resume_button {
                game_paused = false;
            }
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong Clone".to_owned(),
        window_width: 1200,
        window_height: 1000,
        ..Default::default()
    }
}
