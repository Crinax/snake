use macroquad::prelude::*;

#[macroquad::main("Snake")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("Hello, world!", 100.0, 100.0, 80.0, WHITE);

        next_frame().await
    }
}
