use bevy::input::{keyboard::KeyboardInput, mouse::MouseButtonInput};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{prelude::SliceRandom, Rng};

mod camera;
use crate::camera::{edge_scrolling, zoom_camera, RotatingCamera};

// Constants
const WALL_THICKNESS: f32 = 1.0;
const LEFT_WALL: f32 = -3600.;
const RIGHT_WALL: f32 = 3600.;
const BOTTOM_WALL: f32 = -2400.;
const TOP_WALL: f32 = 2400.;
const WALL_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
const CHUNK_SIZE: f32 = 10.0;
const MATRIX_WIDTH: usize = ((RIGHT_WALL - LEFT_WALL) / CHUNK_SIZE) as usize;
const MATRIX_HEIGHT: usize = ((TOP_WALL - BOTTOM_WALL) / CHUNK_SIZE) as usize;

// Components
#[derive(Component)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Component, Clone)]
enum Particle {
    Sand,
    Liquid,
    Solid,
    Gas,
    Smoke,
    Erase,
}

impl Particle {
    fn color(&self) -> Color {
        match self {
            Particle::Sand => Color::srgb(1.0, 1.0, 0.6),
            Particle::Liquid => Color::srgb(0.0, 0.0, 1.0),
            Particle::Solid => Color::srgb(0.5, 0.5, 0.5),
            Particle::Gas => Color::srgb(0.8, 1.0, 0.8),
            Particle::Smoke => Color::srgb(0.5, 0.5, 0.5),
            Particle::Erase => Color::srgba(1.0, 0.0, 0.0, 0.5),
        }
    }
}

#[derive(Component)]
struct PlacementShape;

// Resources
#[derive(Resource)]
struct ParticleMatrix {
    matrix: Vec<Vec<Option<Entity>>>,
}

impl ParticleMatrix {
    fn new() -> Self {
        ParticleMatrix {
            matrix: vec![vec![None; MATRIX_WIDTH]; MATRIX_HEIGHT],
        }
    }
}

#[derive(Resource)]
struct SelectedParticle(Particle);

#[derive(Resource)]
struct MouseState {
    button_pressed: bool,
}

#[derive(Resource)]
struct PlacementSize {
    size: f32,
    position: Vec2,
}

impl PlacementSize {
    fn new() -> Self {
        PlacementSize {
            size: 10.0,
            position: Vec2::ZERO,
        }
    }
}

// Systems
fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(RotatingCamera);
    commands.insert_resource(ParticleMatrix::new());
    commands.insert_resource(SelectedParticle(Particle::Sand));

    // Spawn walls
    spawn_wall(&mut commands, WallLocation::Left);
    spawn_wall(&mut commands, WallLocation::Right);
    spawn_wall(&mut commands, WallLocation::Bottom);
    spawn_wall(&mut commands, WallLocation::Top);
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

fn update_mouse_state(
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut mouse_state: ResMut<MouseState>,
) {
    for event in mouse_button_events.read() {
        if event.button == MouseButton::Left {
            mouse_state.button_pressed = event.state.is_pressed();
        }
    }
}

fn update_placement_shape(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<RotatingCamera>>,
    mut placement_size: ResMut<PlacementSize>,
    mut placement_shape_query: Query<(Entity, &mut Transform, &mut Sprite), With<PlacementShape>>,
    selected_particle: Res<SelectedParticle>,
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
            let color = match selected_particle.0 {
                Particle::Erase => Color::rgba(1.0, 0.0, 0.0, 0.2), // Semi-transparent red for Erase
                _ => Color::rgba(1.0, 1.0, 1.0, 0.2), // Default color for other particles
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

fn handle_input(
    mut commands: Commands,
    mouse_state: Res<MouseState>,
    mut keyboard_input: EventReader<KeyboardInput>,
    mut particle_matrix: ResMut<ParticleMatrix>,
    mut selected_particle: ResMut<SelectedParticle>,
    mut placement_size: ResMut<PlacementSize>,
) {
    // Update selected particle
    for event in keyboard_input.read() {
        match event.key_code {
            KeyCode::Digit1 => selected_particle.0 = Particle::Sand,
            KeyCode::Digit2 => selected_particle.0 = Particle::Liquid,
            KeyCode::Digit3 => selected_particle.0 = Particle::Solid,
            KeyCode::Digit4 => selected_particle.0 = Particle::Gas,
            KeyCode::Digit5 => selected_particle.0 = Particle::Smoke,
            KeyCode::Digit0 => selected_particle.0 = Particle::Erase,
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
                    match selected_particle.0 {
                        Particle::Erase => {
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

fn spawn_particle(
    commands: &mut Commands,
    particle_matrix: &mut ParticleMatrix,
    x: usize,
    y: usize,
    particle_type: Particle,
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
                    color: particle_type.color(),
                    custom_size: Some(Vec2::splat(CHUNK_SIZE)),
                    ..default()
                },
                ..default()
            },
            particle_type,
            Position { x, y },
        ))
        .id();

    particle_matrix.matrix[y][x] = Some(entity);
}

fn update_particles(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &Particle, &mut Position)>,
    mut particle_matrix: ResMut<ParticleMatrix>,
) {
    let mut rng = rand::thread_rng();
    let mut moves = Vec::new();

    // Determine moves
    for (entity, particle, position) in particle_query.iter() {
        let (new_x, new_y) = match particle {
            Particle::Sand => simulate_sand(position.x, position.y, &particle_matrix, &mut rng),
            Particle::Liquid => simulate_liquid(position.x, position.y, &particle_matrix, &mut rng),
            Particle::Solid => (position.x, position.y),
            Particle::Gas => simulate_gas(position.x, position.y, &particle_matrix, &mut rng),
            Particle::Smoke => simulate_smoke(position.x, position.y, &particle_matrix, &mut rng),
            Particle::Erase => continue,
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

// Add this helper function to check if a position is within the matrix bounds
fn is_in_bounds(x: isize, y: isize) -> bool {
    x >= 0 && x < MATRIX_WIDTH as isize && y >= 0 && y < MATRIX_HEIGHT as isize
}

// Add this helper function to check if a position is empty
fn is_empty(particle_matrix: &ParticleMatrix, x: usize, y: usize) -> bool {
    particle_matrix.matrix[y][x].is_none()
}

fn simulate_sand(
    x: usize,
    y: usize,
    particle_matrix: &ParticleMatrix,
    rng: &mut impl Rng,
) -> (usize, usize) {
    let x = x as isize;
    let y = y as isize;

    if is_in_bounds(x, y - 1) && is_empty(particle_matrix, x as usize, (y - 1) as usize) {
        (x as usize, (y - 1) as usize)
    } else {
        let down_left = is_in_bounds(x - 1, y - 1)
            && is_empty(particle_matrix, (x - 1) as usize, (y - 1) as usize);
        let down_right = is_in_bounds(x + 1, y - 1)
            && is_empty(particle_matrix, (x + 1) as usize, (y - 1) as usize);

        if down_left && down_right {
            if is_empty(particle_matrix, (x - 1) as usize, y as usize)
                && is_empty(particle_matrix, (x + 1) as usize, y as usize)
            {
                if rng.gen_bool(0.5) {
                    ((x - 1) as usize, (y - 1) as usize)
                } else {
                    ((x + 1) as usize, (y - 1) as usize)
                }
            } else if is_empty(particle_matrix, (x - 1) as usize, y as usize) {
                ((x - 1) as usize, (y - 1) as usize)
            } else if is_empty(particle_matrix, (x + 1) as usize, y as usize) {
                ((x + 1) as usize, (y - 1) as usize)
            } else {
                (x as usize, y as usize)
            }
        } else if down_left && is_empty(particle_matrix, (x - 1) as usize, y as usize) {
            ((x - 1) as usize, (y - 1) as usize)
        } else if down_right && is_empty(particle_matrix, (x + 1) as usize, y as usize) {
            ((x + 1) as usize, (y - 1) as usize)
        } else {
            (x as usize, y as usize)
        }
    }
}

fn simulate_liquid(
    x: usize,
    y: usize,
    particle_matrix: &ParticleMatrix,
    rng: &mut impl Rng,
) -> (usize, usize) {
    let x = x as isize;
    let y = y as isize;

    if is_in_bounds(x, y - 1) && is_empty(particle_matrix, x as usize, (y - 1) as usize) {
        (x as usize, (y - 1) as usize)
    } else {
        let left =
            is_in_bounds(x - 1, y) && is_empty(particle_matrix, (x - 1) as usize, y as usize);
        let right =
            is_in_bounds(x + 1, y) && is_empty(particle_matrix, (x + 1) as usize, y as usize);

        if left && right {
            if rng.gen_bool(0.5) {
                ((x - 1) as usize, y as usize)
            } else {
                ((x + 1) as usize, y as usize)
            }
        } else if left {
            ((x - 1) as usize, y as usize)
        } else if right {
            ((x + 1) as usize, y as usize)
        } else if is_in_bounds(x, y + 1)
            && is_empty(particle_matrix, x as usize, (y + 1) as usize)
            && rng.gen_bool(0.1)
        {
            (x as usize, (y + 1) as usize) // Small chance to move up (bubbling effect)
        } else {
            (x as usize, y as usize)
        }
    }
}

fn simulate_gas(
    x: usize,
    y: usize,
    particle_matrix: &ParticleMatrix,
    rng: &mut impl Rng,
) -> (usize, usize) {
    let x = x as isize;
    let y = y as isize;

    if is_in_bounds(x, y + 1) && is_empty(particle_matrix, x as usize, (y + 1) as usize) {
        (x as usize, (y + 1) as usize)
    } else {
        let left =
            is_in_bounds(x - 1, y) && is_empty(particle_matrix, (x - 1) as usize, y as usize);
        let right =
            is_in_bounds(x + 1, y) && is_empty(particle_matrix, (x + 1) as usize, y as usize);

        if left && right {
            if rng.gen_bool(0.5) {
                ((x - 1) as usize, y as usize)
            } else {
                ((x + 1) as usize, y as usize)
            }
        } else if left {
            ((x - 1) as usize, y as usize)
        } else if right {
            ((x + 1) as usize, y as usize)
        } else if is_in_bounds(x, y - 1)
            && is_empty(particle_matrix, x as usize, (y - 1) as usize)
            && rng.gen_bool(0.2)
        {
            (x as usize, (y - 1) as usize) // Small chance to move down (sinking effect)
        } else {
            (x as usize, y as usize)
        }
    }
}

fn simulate_smoke(
    x: usize,
    y: usize,
    particle_matrix: &ParticleMatrix,
    rng: &mut impl Rng,
) -> (usize, usize) {
    let x = x as isize;
    let y = y as isize;

    if is_in_bounds(x, y + 1) && is_empty(particle_matrix, x as usize, (y + 1) as usize) {
        (x as usize, (y + 1) as usize)
    } else {
        let left =
            is_in_bounds(x - 1, y) && is_empty(particle_matrix, (x - 1) as usize, y as usize);
        let right =
            is_in_bounds(x + 1, y) && is_empty(particle_matrix, (x + 1) as usize, y as usize);

        if left && right {
            match rng.gen_range(0..10) {
                0..=6 => (x as usize, y as usize), // 70% chance to stay in place
                7..=8 => ((x - 1) as usize, y as usize), // 20% chance to move left
                _ => ((x + 1) as usize, y as usize), // 10% chance to move right
            }
        } else if left {
            if rng.gen_bool(0.3) {
                ((x - 1) as usize, y as usize)
            } else {
                (x as usize, y as usize)
            }
        } else if right {
            if rng.gen_bool(0.3) {
                ((x + 1) as usize, y as usize)
            } else {
                (x as usize, y as usize)
            }
        } else {
            (x as usize, y as usize)
        }
    }
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(MouseState {
            button_pressed: false,
        })
        .insert_resource(PlacementSize::new())
        .add_systems(
            Update,
            (
                edge_scrolling,
                zoom_camera,
                update_placement_shape,
                handle_input,
                update_particles,
                update_mouse_state,
            ),
        )
        .run();
}

