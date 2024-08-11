use crate::components::{element::ElementType, placement_shape::PlacementShape};
use crate::resources::*;
use crate::utils::camera::RotatingCamera;
use crate::utils::constants::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn placement_shape(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<RotatingCamera>>,
    mut placement_size: ResMut<PlacementSize>,
    mut placement_shape_query: Query<(Entity, &mut Transform, &mut Sprite), With<PlacementShape>>,
    selected_particle: Res<SelectedElement>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // Check if the mouse position is within bounds
        if world_position.x >= LEFT_WALL
            && world_position.x <= RIGHT_WALL
            && world_position.y >= BOTTOM_WALL
            && world_position.y <= TOP_WALL
        {
            placement_size.position = world_position;

            let half_size = placement_size.size / 2.0;
            let top_left = Vec2::new(
                (world_position.x - half_size).max(LEFT_WALL),
                (world_position.y + half_size).min(TOP_WALL),
            );
            let bottom_right = Vec2::new(
                (world_position.x + half_size).min(RIGHT_WALL),
                (world_position.y - half_size).max(BOTTOM_WALL),
            );
            let size = bottom_right - top_left;
            let center = (top_left + bottom_right) / 2.0;

            // Determine the color based on the selected particle
            let color = match selected_particle.0.element_type {
                ElementType::Erase => Color::srgba(1.0, 0.0, 0.0, 0.2), // Semi-transparent red for Erase
                _ => Color::srgba(1.0, 1.0, 1.0, 0.2), // Default color for other particles
            };

            if let Ok((_, mut transform, mut sprite)) = placement_shape_query.get_single_mut() {
                transform.translation = center.extend(2.0);
                sprite.custom_size = Some(size);
                sprite.color = color; // Update the color
            } else {
                commands
                    .spawn(SpriteBundle {
                        transform: Transform::from_translation(center.extend(2.0)),
                        sprite: Sprite {
                            color,
                            custom_size: Some(size),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(PlacementShape);
            }
        }
    }
}
