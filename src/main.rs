use macroquad::prelude::*;

const THETA: f32 = 3.14 / 7.0;

fn draw_tree(size: f32, current_x: f32, current_y: f32, iterations: i32, alpha: f32) {
    let rto_x: f32 = current_x + (alpha - THETA).cos() * size;
    let rto_y = current_y - (alpha - THETA).sin() * size;

    draw_line(current_x, current_y, rto_x, rto_y, 1.0, BLACK);

    let lto_x = current_x + (alpha + THETA).cos() * size;
    let lto_y = current_y - (alpha + THETA).sin() * size;

    draw_line(current_x, current_y, lto_x, lto_y, 1.0, BLACK);

    if iterations != 0 {
        draw_tree(size * 0.9, rto_x, rto_y, iterations - 1, alpha - THETA);
        draw_tree(size * 0.9, lto_x, lto_y, iterations - 1, alpha + THETA)
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(500.0, 500.0);
    loop {
        clear_background(WHITE);

        draw_line(400.0, 500.0, 400.0, 600.0, 1.0, BLACK);
        draw_tree(65.0, 400.0, 500.0, 6, 3.14 / 2.0);

        next_frame().await;
    }
}