use macroquad::{prelude::*, rand::gen_range, ui::root_ui};

mod ai;
mod bounds;
mod constants;
mod physics;
mod player;

use crate::{
    constants::*,
    physics::{ColliderType, GameObject},
    player::Player,
};

#[macroquad::main(window_conf)]
async fn main() {
    println!("_________New game_________");

    let top_bound = GameObject::from_pos(
        BOUNDS.x,
        BOUNDS.y - BOUNDS_THICKNESS,
        ColliderType::Rectangle(BOUNDS.w, BOUNDS_THICKNESS),
    );
    let bottom_bound = GameObject::from_pos(
        BOUNDS.x,
        BOUNDS.h,
        ColliderType::Rectangle(BOUNDS.w, BOUNDS_THICKNESS),
    );

    let left_bound = GameObject::from_pos(
        BOUNDS.x - BOUNDS_THICKNESS,
        BOUNDS.y,
        ColliderType::Rectangle(BOUNDS_THICKNESS, BOUNDS.h),
    );
    let right_bound = GameObject::from_pos(
        BOUNDS.w,
        BOUNDS.y,
        ColliderType::Rectangle(BOUNDS_THICKNESS, BOUNDS.h),
    );

    let paddle_left = GameObject::from_pos(
        BOUNDS.x + PLAYER_PADDING,
        BOUNDS.center().y - PLAYER_HEIGHT / 2.0,
        ColliderType::Rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
    );
    let paddle_right = GameObject::from_pos(
        BOUNDS.w - PLAYER_PADDING - PLAYER_WIDTH,
        BOUNDS.center().y - PLAYER_HEIGHT / 2.0,
        ColliderType::Rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
    );

    let mut player_left = Player::new("Player 1", paddle_left, BOUNDS, PLAYER_VELOCITY.into());
    let mut player_right = Player::new("Player 2", paddle_right, BOUNDS, PLAYER_VELOCITY.into());

    let mut ball = GameObject::from_pos(
        BOUNDS.center().x - BALL_RADIUS,
        BOUNDS.center().y - BALL_RADIUS,
        ColliderType::Sphere(BALL_RADIUS),
    );

    let mut score_time = get_time();
    let mut random_start = match gen_range::<i32>(1, 2) {
        1 => -1,
        _ => 1,
    };

    let mut game_paused = false;
    let mut show_prediction = false;
    let mut mouse_controlled = false;

    let mut camera = Camera2D::from_display_rect(Rect::new(
        BOUNDS.x - 1.0,
        BOUNDS.y - 1.0,
        BOUNDS.w + 2.0,
        BOUNDS.h + 2.0,
    ));

    let mut ai_left = SARAH;
    let mut ai_right = RAPHAEL;

    loop {
        let game_position = BOUNDS.screen_offset();
        let game_size = BOUNDS.screen_size();
        camera.viewport = Some((
            game_position.x as i32,
            game_position.y as i32,
            game_size.x as i32,
            game_size.y as i32,
        ));

        set_camera(&camera);

        let frame_time = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            game_paused ^= true;
        }

        if !game_paused {
            let reset_button = root_ui().button(vec2(0.0, 0.0), "Reset ball");
            let show_prediction_button = root_ui().button(vec2(0.0, 20.0), "Show ball prediction");

            if show_prediction_button {
                show_prediction ^= true;
            }

            //___SCORING___//

            if ball.check_collisions(&left_bound).is_some() {
                player_right.scored();
                score_time = get_time();
            }

            if ball.check_collisions(&right_bound).is_some() {
                player_left.scored();
                score_time = get_time();
            }

            if reset_button {
                score_time = get_time();
            }

            if score_time != 0.0 {
                ball.position = BOUNDS.center() - Vec2::from(BALL_SIZE) / 2.0;
                ball.velocity = vec2(0.0, 0.0);

                if get_time() > score_time + 1.0 {
                    ball.velocity = vec2((random_start as f32) * 1000.0, gen_range(-400.0, 400.0));
                    random_start *= -1;
                    score_time = 0.0;
                }
            }

            //___PHYSICS___//

            ball.handle_bounces(
                vec![
                    &player_left.object,
                    &player_right.object,
                    &top_bound,
                    &bottom_bound,
                ],
                frame_time,
            );

            //___PLAYER INPUTS___//

            if is_mouse_button_pressed(MouseButton::Right) {
                mouse_controlled ^= true;
                if mouse_controlled {
                    score_time = get_time();
                }
            }

            if mouse_controlled {
                set_cursor_grab(true);
                show_mouse(false);
                player_left.mouse_control(frame_time);
            } else {
                set_cursor_grab(false);
                show_mouse(true);
                player_left.keyboard_control(KeyCode::W, KeyCode::S, frame_time);
            }

            //___AIs___//

            if !mouse_controlled {
                ai_left.logic.observe(
                    player_left.object.position,
                    ball.check_collisions_vec(vec![&player_right.object]),
                    ball.position,
                    ball.velocity,
                );
                player_left.ai_control(&mut ai_left, frame_time);
            }

            ai_right.logic.observe(
                player_right.object.position,
                ball.check_collisions_vec(vec![&player_left.object]),
                ball.position,
                ball.velocity,
            );
            player_right.ai_control(&mut ai_right, frame_time);

            if show_prediction {
                if let Some(predicted_position) = ai_left.logic.predicted_position {
                    GameObject::from_pos(
                        predicted_position.x,
                        predicted_position.y,
                        ColliderType::Sphere(BALL_RADIUS),
                    )
                    .show_object(WHITE);
                }

                if let Some(predicted_position) = ai_right.logic.predicted_position {
                    GameObject::from_pos(
                        predicted_position.x,
                        predicted_position.y,
                        ColliderType::Sphere(BALL_RADIUS),
                    )
                    .show_object(WHITE);
                }
            }
        } else {
            set_cursor_grab(false);
            show_mouse(true);
            let resume_button =
                root_ui().button(vec2(screen_width() / 2.0, screen_height() / 2.0), "Resume");
            if resume_button {
                game_paused = false;
            }
        }

        draw_text(
            &player_left.score.to_string(),
            (BOUNDS.center().x + BOUNDS.x) / 2.0,
            BOUNDS.center().y / 2.0,
            60.0,
            WHITE,
        );
        draw_text(
            &player_right.score.to_string(),
            (BOUNDS.w + BOUNDS.center().x) / 2.0,
            BOUNDS.center().y / 2.0,
            60.0,
            WHITE,
        );

        player_left.object.show_object(WHITE);
        player_right.object.show_object(WHITE);
        ball.show_object(WHITE);
        top_bound.show_object(WHITE);
        bottom_bound.show_object(WHITE);

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
