use bevy::prelude::*;

#[derive(Resource)]
pub struct PlacementSize {
    pub size: f32,
    pub position: Vec2,
}

impl PlacementSize {
    pub fn new() -> Self {
        PlacementSize {
            size: 10.0,
            position: Vec2::ZERO,
        }
    }
}
