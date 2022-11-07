use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0; // Scale by a factor of 25px

pub fn to_coord(game_coord: i32) -> f64 {
    return (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    return (to_coord(game_coord) as u32)
}

pub fn draw_block(color: Color, x: i32, y: i32, ctx: &Context, g2d: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        ctx.transform,
        g2d,
    )
}

pub fn draw_rect(color: Color, x: i32, y: i32, width: i32, height: i32, ctx: &Context, g2d: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [
            gui_x, 
            gui_y, 
            BLOCK_SIZE * (width as f64), 
            BLOCK_SIZE * (height as f64)
        ],
        ctx.transform,
        g2d,
    )
}