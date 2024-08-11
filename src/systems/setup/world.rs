use crate::components::element::Element;
use crate::resources::{particle_matrix::ParticleMatrix, selected_element::SelectedElement};
use crate::utils::constants::*;
use bevy::prelude::*;

pub fn world(mut commands: Commands) {
    commands.insert_resource(ParticleMatrix::new());
    commands.insert_resource(SelectedElement(Element::new("Sand".to_string())));

    // Spawn walls
    spawn_wall(&mut commands, WallLocation::Left);
    spawn_wall(&mut commands, WallLocation::Right);
    spawn_wall(&mut commands, WallLocation::Bottom);
    spawn_wall(&mut commands, WallLocation::Top);
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

fn spawn_wall(commands: &mut Commands, location: WallLocation) {
    let (position, size) = match location {
        WallLocation::Left => (
            Vec2::new(LEFT_WALL, 0.),
            Vec2::new(WALL_THICKNESS, TOP_WALL - BOTTOM_WALL + WALL_THICKNESS),
        ),
        WallLocation::Right => (
            Vec2::new(RIGHT_WALL, 0.),
            Vec2::new(WALL_THICKNESS, TOP_WALL - BOTTOM_WALL + WALL_THICKNESS),
        ),
        WallLocation::Bottom => (
            Vec2::new(0., BOTTOM_WALL),
            Vec2::new(RIGHT_WALL - LEFT_WALL + WALL_THICKNESS, WALL_THICKNESS),
        ),
        WallLocation::Top => (
            Vec2::new(0., TOP_WALL),
            Vec2::new(RIGHT_WALL - LEFT_WALL + WALL_THICKNESS, WALL_THICKNESS),
        ),
    };

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(position.extend(0.0)),
        sprite: Sprite {
            color: WALL_COLOR,
            custom_size: Some(size),
            ..default()
        },
        ..default()
    });
}
