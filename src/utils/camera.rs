use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const ZOOM_SPEED: f32 = 0.1;
const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 5.0;
const EDGE_THICKNESS: f32 = 40.0;
const CAMERA_MOVE_SPEED: f32 = 750.0;

#[derive(Component)]
pub struct RotatingCamera;

pub fn edge_scrolling(
    mut camera_query: Query<&mut Transform, With<RotatingCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.single();
    let mut camera_transform = camera_query.single_mut();

    if let Some(position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let mut move_direction = Vec2::ZERO;

        // Check left edge
        if position.x < EDGE_THICKNESS {
            move_direction.x -= 1.0;
        }
        // Check right edge
        if position.x > window_size.x - EDGE_THICKNESS {
            move_direction.x += 1.0;
        }
        // Check bottom edge
        if position.y < EDGE_THICKNESS {
            move_direction.y += 1.0;
        }
        // Check top edge
        if position.y > window_size.y - EDGE_THICKNESS {
            move_direction.y -= 1.0;
        }

        if move_direction != Vec2::ZERO {
            let move_amount = move_direction.normalize() * CAMERA_MOVE_SPEED * time.delta_seconds();
            camera_transform.translation += move_amount.extend(0.0);
        }
    }
}

pub fn zoom_camera(
    mut camera_query: Query<&mut Transform, With<RotatingCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
    windows: Query<Entity, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let mut camera_transform = camera_query.single_mut();

    for ev in scroll_evr.read() {
        // Only process events for the primary window
        if ev.window == window {
            let zoom_factor = 1.0 - ev.y * ZOOM_SPEED;

            let new_scale = (camera_transform.scale.x * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);
            camera_transform.scale = Vec3::splat(new_scale);
        }
    }
}
