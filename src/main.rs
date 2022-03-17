use macroquad::{prelude::*, ui::root_ui};

mod ai;
mod bounds;
mod constants;
mod game;
mod physics;
mod player;

use crate::{
    constants::*,
    game::Game,
    player::{ControlType, PlayerPosition, UserType},
};

#[macroquad::main(window_conf)]
async fn main() {
    println!("_________New game_________");

    rand::srand(macroquad::miniquad::date::now() as _);

    let mut game_paused = false;
    let mut game = Game::new();

    let player_left = UserType::Client(ControlType::Mouse);
    let player_right = UserType::Ai(RAPHAEL);

    game.add_player(player_left, PlayerPosition::Left);
    game.add_player(player_right, PlayerPosition::Right);
    // game.add_player(UserType::Client(ControlType::Keyboard(KeyCode::W, KeyCode::S)), PlayerPosition::Left);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            game_paused ^= true;
        }

        if is_mouse_button_pressed(MouseButton::Middle) {
            let matching_id = game.get_id(player_right);
            if let Some(id) = matching_id.first() {
                game.remove_player(*id);
            }
            game.add_player(UserType::Ai(SARAH), PlayerPosition::Right);
        }

        if !game_paused {
            game.update();
        } else {
            set_cursor_grab(false);
            show_mouse(true);
            let resume_button =
                root_ui().button(vec2(screen_width() / 2.0, screen_height() / 2.0), "Resume");
            if resume_button {
                game_paused = false;
            }
        }
        game.update_camera();
        game.show();

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
