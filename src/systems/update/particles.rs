use crate::components::{
    element::{Element, ElementType},
    position::Position,
};
use crate::resources::particle_matrix::ParticleMatrix;
use crate::utils::{constants::*, particles::*};
use bevy::prelude::*;
use rand::seq::SliceRandom;

pub fn particles(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &Element, &mut Position)>,
    mut particle_matrix: ResMut<ParticleMatrix>,
) {
    let mut rng = rand::thread_rng();
    let mut moves = Vec::new();

    // Determine moves
    for (entity, element, position) in particle_query.iter() {
        let (new_x, new_y) = match element.element_type {
            ElementType::MovableSolid => {
                simulate_movable_solid(position.x, position.y, &particle_matrix, &mut rng, element)
            }
            ElementType::Liquid => {
                simulate_liquid(position.x, position.y, &particle_matrix, &mut rng, element)
            }
            ElementType::ImmovableSolid => (position.x, position.y),
            ElementType::Gas => {
                simulate_gas(position.x, position.y, &particle_matrix, &mut rng, element)
            }
            ElementType::Erase => continue,
        };

        if new_x != position.x || new_y != position.y {
            moves.push((entity, new_x, new_y));
        }
    }

    // Shuffle the moves to prevent bias
    moves.shuffle(&mut rng);

    // Apply moves
    for (entity, new_x, new_y) in moves {
        if let Ok((_, _, mut position)) = particle_query.get_mut(entity) {
            if particle_matrix.matrix[new_y][new_x].is_none() {
                particle_matrix.matrix[position.y][position.x] = None;
                particle_matrix.matrix[new_y][new_x] = Some(entity);
                position.x = new_x;
                position.y = new_y;

                let new_translation = Vec3::new(
                    LEFT_WALL + (new_x as f32 + 0.5) * CHUNK_SIZE,
                    BOTTOM_WALL + (new_y as f32 + 0.5) * CHUNK_SIZE,
                    1.0,
                );
                commands
                    .entity(entity)
                    .insert(Transform::from_translation(new_translation));
            }
        }
    }
}
