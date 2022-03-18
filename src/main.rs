use macroquad::{
    prelude::{
        is_key_pressed, is_mouse_button_pressed, next_frame, rand, screen_height, screen_width,
        set_cursor_grab, show_mouse, Conf, KeyCode, MouseButton, scene, collections::storage,
    },
};

mod ai;
mod bounds;
mod constants;
mod game;
mod physics;
mod player;
mod score;

use crate::{
    constants::*,
    game::Game,
    player::{ControlType, PlayerPosition, UserType},
};

#[macroquad::main(window_conf)]
async fn main() {
    println!("_________New game_________");

    rand::srand(macroquad::miniquad::date::now() as _);

    let mut game_paused = true;

    let mut game = Game::new();

    let mut id_left = game.add_player(UserType::Client(ControlType::Mouse), PlayerPosition::Left);
    let mut id_right = game.add_player(UserType::Ai(RAPHAEL), PlayerPosition::Right);
    // game.add_player(UserType::Client(ControlType::Keyboard(KeyCode::W, KeyCode::S)), PlayerPosition::Left);

    scene::add_node(game);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            game_paused ^= true;
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Main menu")
                .open(&mut game_paused)
                .collapsible(false)
                // .fixed_size(MENU_SIZE)
                .fixed_pos((100.0, 100.0))
                .show(egui_ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Test");
                    })
                });
        });

        if !game_paused {

            // if is_mouse_button_pressed(MouseButton::Left) {
            //     let new_player = match game.player_type(id_left).unwrap().is_ai() {
            //         true => game.add_player(UserType::Client(ControlType::Mouse), PlayerPosition::Left),
            //         false => game.add_player(UserType::Ai(SARAH), PlayerPosition::Left),
            //     };
            //     game.remove_player(id_left);
            //     id_left = new_player
            // }
    
            // if is_mouse_button_pressed(MouseButton::Right) {
            //     let new_player = match game.player_type(id_right).unwrap().is_ai() {
            //         true => game.add_player(UserType::Client(ControlType::Mouse), PlayerPosition::Right),
            //         false => game.add_player(UserType::Ai(SARAH), PlayerPosition::Right),
            //     };
            //     game.remove_player(id_right);
            //     id_right = new_player
            // }

            // game.update();

        } else {
            set_cursor_grab(false);
            show_mouse(true);
        }

        // game.update_camera();
        // game.show();
        
        egui_macroquad::draw();

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
