use crate::components::{
    element::{Element, ElementType},
    position::Position,
};
use crate::resources::particle_matrix::ParticleMatrix;
use crate::utils::constants::*;
use bevy::prelude::*;

pub fn spawn_particle(
    commands: &mut Commands,
    particle_matrix: &mut ParticleMatrix,
    x: usize,
    y: usize,
    element: Element,
) {
    let chunk_position = Vec2::new(
        LEFT_WALL + (x as f32 + 0.5) * CHUNK_SIZE,
        BOTTOM_WALL + (y as f32 + 0.5) * CHUNK_SIZE,
    );

    let entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(chunk_position.extend(1.0)),
                sprite: Sprite {
                    color: match element.element_type {
                        ElementType::Liquid => element.get_color_with_alpha(0.1),
                        _ => element.get_color_with_random_alpha(),
                    },
                    custom_size: Some(Vec2::splat(CHUNK_SIZE)),
                    ..default()
                },
                ..default()
            },
            element,
            Position { x, y },
        ))
        .id();

    particle_matrix.matrix[y][x] = Some(entity);
}
