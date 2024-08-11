use bevy::prelude::*;
// Constants
pub const WALL_THICKNESS: f32 = 1.0;
pub const LEFT_WALL: f32 = -3600.;
pub const RIGHT_WALL: f32 = 3600.;
pub const BOTTOM_WALL: f32 = -2400.;
pub const TOP_WALL: f32 = 2400.;
pub const WALL_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
pub const CHUNK_SIZE: f32 = 10.0;
pub const MATRIX_WIDTH: usize = ((RIGHT_WALL - LEFT_WALL) / CHUNK_SIZE) as usize;
pub const MATRIX_HEIGHT: usize = ((TOP_WALL - BOTTOM_WALL) / CHUNK_SIZE) as usize;
