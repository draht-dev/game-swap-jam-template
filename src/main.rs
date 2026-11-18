mod game;
mod sfx;

use game::{Input, State, WORLD_HEIGHT, WORLD_WIDTH};
use macroquad::audio::{load_sound_from_bytes, play_sound_once};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game Swap Jam".to_owned(),
        window_width: WORLD_WIDTH as i32,
        window_height: WORLD_HEIGHT as i32,
        high_dpi: true,
        ..Default::default()
    }
}

fn read_input() -> Input {
    Input {
        left: is_key_down(KeyCode::Left) || is_key_down(KeyCode::A),
        right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
        up: is_key_down(KeyCode::Up) || is_key_down(KeyCode::W),
        down: is_key_down(KeyCode::Down) || is_key_down(KeyCode::S),
    }
}

fn letterbox() -> (f32, f32, f32) {
    let scale = (screen_width() / WORLD_WIDTH).min(screen_height() / WORLD_HEIGHT);
    let ox = (screen_width() - WORLD_WIDTH * scale) * 0.5;
    let oy = (screen_height() - WORLD_HEIGHT * scale) * 0.5;
    (ox, oy, scale)
}

fn to_screen(ox: f32, oy: f32, scale: f32, r: game::Rect) -> (f32, f32, f32, f32) {
    (ox + r.x * scale, oy + r.y * scale, r.w * scale, r.h * scale)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::new();
    // Ton beim Einsammeln. Im Browser erst nach der ersten Eingabe hörbar (Autoplay-Regel).
    let pickup = load_sound_from_bytes(&sfx::beep_wav(880.0, 0.12, 0.3)).await.ok();

    loop {
        let score_before = state.score;
        state.update(read_input(), get_frame_time());
        if state.score > score_before {
            if let Some(sound) = &pickup {
                play_sound_once(sound);
            }
        }

        clear_background(Color::from_rgba(10, 10, 14, 255));
        let (ox, oy, scale) = letterbox();
        draw_rectangle(
            ox,
            oy,
            WORLD_WIDTH * scale,
            WORLD_HEIGHT * scale,
            Color::from_rgba(18, 18, 28, 255),
        );

        let (px, py, pw, ph) = to_screen(ox, oy, scale, state.player);
        draw_rectangle(px, py, pw, ph, Color::from_rgba(80, 200, 255, 255));

        let (cx, cy, cw, ch) = to_screen(ox, oy, scale, state.collectible);
        draw_rectangle(cx, cy, cw, ch, Color::from_rgba(255, 200, 70, 255));

        let font_size = (28.0 * scale).max(16.0);
        draw_text(
            &format!("Punkte: {}", state.score),
            ox + 16.0 * scale,
            oy + 32.0 * scale,
            font_size,
            WHITE,
        );
        draw_text(
            "Bewegen: Pfeiltasten oder WASD",
            ox + 16.0 * scale,
            oy + WORLD_HEIGHT * scale - 16.0 * scale,
            (20.0 * scale).max(14.0),
            Color::from_rgba(180, 180, 190, 255),
        );

        next_frame().await;
    }
}
