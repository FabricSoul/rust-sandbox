use crate::components::element::{Element, ElementType};
use crate::resources::{
    mouse_state::MouseState, particle_matrix::ParticleMatrix, placement_size::PlacementSize,
    selected_element::SelectedElement,
};
use crate::utils::constants::*;
use crate::utils::particles::spawn_particle;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub fn handle_input(
    mut commands: Commands,
    mouse_state: Res<MouseState>,
    mut keyboard_input: EventReader<KeyboardInput>,
    mut particle_matrix: ResMut<ParticleMatrix>,
    mut selected_particle: ResMut<SelectedElement>,
    mut placement_size: ResMut<PlacementSize>,
) {
    // Update selected particle
    for event in keyboard_input.read() {
        match event.key_code {
            KeyCode::Digit1 => selected_particle.0 = Element::new("Sand".to_string()),
            KeyCode::Digit2 => selected_particle.0 = Element::new("Water".to_string()),
            KeyCode::Digit3 => selected_particle.0 = Element::new("Smoke".to_string()),
            KeyCode::Digit4 => selected_particle.0 = Element::new("Stone".to_string()),
            KeyCode::Digit5 => selected_particle.0 = Element::new("Erase".to_string()),
            KeyCode::Minus => {
                placement_size.size = (placement_size.size - 10.0).max(10.0);
            }
            KeyCode::Equal => {
                placement_size.size = (placement_size.size + 10.0).min(100.0);
            }
            _ => {}
        }
    }

    // Handle mouse input for particle placement or erasure
    if mouse_state.button_pressed {
        let half_size = placement_size.size / 2.0;
        let top_left = Vec2::new(
            (placement_size.position.x - half_size).max(LEFT_WALL),
            (placement_size.position.y + half_size).min(TOP_WALL),
        );
        let bottom_right = Vec2::new(
            (placement_size.position.x + half_size).min(RIGHT_WALL),
            (placement_size.position.y - half_size).max(BOTTOM_WALL),
        );

        for y in (bottom_right.y as i32..=top_left.y as i32).step_by(CHUNK_SIZE as usize) {
            for x in (top_left.x as i32..=bottom_right.x as i32).step_by(CHUNK_SIZE as usize) {
                let matrix_x = ((x as f32 - LEFT_WALL) / CHUNK_SIZE) as usize;
                let matrix_y = ((y as f32 - BOTTOM_WALL) / CHUNK_SIZE) as usize;

                if matrix_y < MATRIX_HEIGHT && matrix_x < MATRIX_WIDTH {
                    match selected_particle.0.element_type {
                        ElementType::Erase => {
                            if let Some(entity) = particle_matrix.matrix[matrix_y][matrix_x] {
                                commands.entity(entity).despawn();
                                particle_matrix.matrix[matrix_y][matrix_x] = None;
                            }
                        }
                        _ => {
                            if particle_matrix.matrix[matrix_y][matrix_x].is_none() {
                                spawn_particle(
                                    &mut commands,
                                    &mut particle_matrix,
                                    matrix_x,
                                    matrix_y,
                                    selected_particle.0.clone(),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
