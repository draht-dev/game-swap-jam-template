use macroquad::prelude::*;

#[macroquad::main("Game Swap Jam")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
