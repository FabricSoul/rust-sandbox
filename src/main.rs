use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod camera;
use crate::camera::{edge_scrolling, zoom_camera, RotatingCamera};

const WALL_THICKNESS: f32 = 1.0;
// x coordinates
const LEFT_WALL: f32 = -900.;
const RIGHT_WALL: f32 = 900.;
// y coordinates
const BOTTOM_WALL: f32 = -600.;
const TOP_WALL: f32 = 600.;
const WALL_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

// Chunk
const LINE_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
const CHUNK_SIZE: f32 = 10.0;
const LINE_THICKNESS: f32 = 1.0;

// Square
const SQUARE_COLOR: Color = Color::srgb(1.0, 1.0, 0.6);

// New constants for falling
const GRAVITY: f32 = 1.0; // Pixels per second squared
                          //
const MATRIX_WIDTH: usize = ((RIGHT_WALL - LEFT_WALL) / CHUNK_SIZE) as usize;
const MATRIX_HEIGHT: usize = ((TOP_WALL - BOTTOM_WALL) / CHUNK_SIZE) as usize;

#[derive(Component)]
struct ChunkSquare;

#[derive(Resource)]
struct ChunkMatrix {
    matrix: Vec<Vec<bool>>,
    entities: Vec<Vec<Option<Entity>>>,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
}

#[derive(Resource)]
struct MouseState {
    button_pressed: bool,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
        }
    }
}

impl ChunkMatrix {
    fn new() -> Self {
        ChunkMatrix {
            matrix: vec![vec![false; MATRIX_WIDTH]; MATRIX_HEIGHT],
            entities: vec![vec![None; MATRIX_WIDTH]; MATRIX_HEIGHT],
        }
    }
}

#[derive(Component)]
struct ChunkLine;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ChunkMatrix::new())
        .insert_resource(MouseState {
            button_pressed: false,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                edge_scrolling,
                zoom_camera,
                update_mouse_state,
                handle_input,
                apply_custom_physics,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(RotatingCamera);

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    // Draw Chunk Lines
    draw_chunk_lines(&mut commands);
}

fn draw_chunk_lines(commands: &mut Commands) {
    let arena_width = RIGHT_WALL - LEFT_WALL;
    let arena_height = TOP_WALL - BOTTOM_WALL;

    // Vertical lines
    for x in (LEFT_WALL as i32..=RIGHT_WALL as i32).step_by(CHUNK_SIZE as usize) {
        if x != LEFT_WALL as i32 && x != RIGHT_WALL as i32 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(x as f32, 0.0, 0.0),
                        scale: Vec3::new(LINE_THICKNESS, arena_height, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: LINE_COLOR,
                        ..default()
                    },
                    ..default()
                },
                ChunkLine,
            ));
        }
    }

    // Horizontal lines
    for y in (BOTTOM_WALL as i32..=TOP_WALL as i32).step_by(CHUNK_SIZE as usize) {
        if y != BOTTOM_WALL as i32 && y != TOP_WALL as i32 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, y as f32, 0.0),
                        scale: Vec3::new(arena_width, LINE_THICKNESS, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: LINE_COLOR,
                        ..default()
                    },
                    ..default()
                },
                ChunkLine,
            ));
        }
    }
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

fn handle_input(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<RotatingCamera>>,
    mouse_state: Res<MouseState>,
    mut chunk_matrix: ResMut<ChunkMatrix>,
) {
    if mouse_state.button_pressed {
        let window = window_query.single();
        let (camera, camera_transform) = camera_query.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            if world_position.x > LEFT_WALL
                && world_position.x < RIGHT_WALL
                && world_position.y > BOTTOM_WALL
                && world_position.y < TOP_WALL
            {
                let matrix_x = ((world_position.x - LEFT_WALL) / CHUNK_SIZE) as usize;
                let matrix_y = ((world_position.y - BOTTOM_WALL) / CHUNK_SIZE) as usize;

                if matrix_y < MATRIX_HEIGHT
                    && matrix_x < MATRIX_WIDTH
                    && !chunk_matrix.matrix[matrix_y][matrix_x]
                {
                    let chunk_position = Vec2::new(
                        LEFT_WALL + (matrix_x as f32 + 0.5) * CHUNK_SIZE,
                        BOTTOM_WALL + (matrix_y as f32 + 0.5) * CHUNK_SIZE,
                    );

                    let entity = commands
                        .spawn((
                            SpriteBundle {
                                transform: Transform::from_xyz(
                                    chunk_position.x,
                                    chunk_position.y,
                                    1.0,
                                ),
                                sprite: Sprite {
                                    color: SQUARE_COLOR,
                                    custom_size: Some(Vec2::splat(CHUNK_SIZE)),
                                    ..default()
                                },
                                ..default()
                            },
                            ChunkSquare,
                        ))
                        .id();

                    chunk_matrix.matrix[matrix_y][matrix_x] = true;
                    chunk_matrix.entities[matrix_y][matrix_x] = Some(entity);

                    println!("Square spawned at ({}, {})", matrix_x, matrix_y);
                }
            }
        }
    }
}

fn apply_custom_physics(
    mut chunk_matrix: ResMut<ChunkMatrix>,
    mut query: Query<&mut Transform, With<ChunkSquare>>,
) {
    let mut moves = Vec::new();
    // First pass: Determine moves
    for y in 0..MATRIX_HEIGHT {
        for x in 0..MATRIX_WIDTH {
            if chunk_matrix.matrix[y][x] {
                let mut new_y = y;
                let mut new_x = x;
                let mut moved = false;

                // Try to move down
                for _ in 0..GRAVITY as usize {
                    if new_y > 0 && !chunk_matrix.matrix[new_y - 1][new_x] {
                        new_y -= 1;
                        moved = true;
                    } else {
                        break;
                    }
                }

                // If can't move down, try to move left or right
                if !moved && new_y > 0 && chunk_matrix.matrix[new_y - 1][new_x] {
                    let can_move_left = new_x > 0
                        && !chunk_matrix.matrix[new_y][new_x - 1]
                        && !chunk_matrix.matrix[new_y - 1][new_x - 1];
                    let can_move_right = new_x < MATRIX_WIDTH - 1
                        && !chunk_matrix.matrix[new_y][new_x + 1]
                        && !chunk_matrix.matrix[new_y - 1][new_x + 1];

                    if can_move_left && can_move_right {
                        // Randomly choose left or right
                        if rand::random() {
                            new_x -= 1;
                        } else {
                            new_x += 1;
                        }
                        new_y -= 1;
                        moved = true;
                    } else if can_move_left {
                        new_x -= 1;
                        new_y -= 1;
                        moved = true;
                    } else if can_move_right {
                        new_x += 1;
                        new_y -= 1;
                        moved = true;
                    }
                }

                // If the square can move, record the move
                if moved {
                    moves.push((x, y, new_x, new_y));
                }
            }
        }
    }

    // Second pass: Apply moves
    for (old_x, old_y, new_x, new_y) in moves {
        chunk_matrix.matrix[old_y][old_x] = false;
        chunk_matrix.matrix[new_y][new_x] = true;
        if let Some(entity) = chunk_matrix.entities[old_y][old_x].take() {
            chunk_matrix.entities[new_y][new_x] = Some(entity);
            if let Ok(mut transform) = query.get_mut(entity) {
                transform.translation.x = LEFT_WALL + (new_x as f32 + 0.5) * CHUNK_SIZE;
                transform.translation.y = BOTTOM_WALL + (new_y as f32 + 0.5) * CHUNK_SIZE;
            }
        }
        println!(
            "Square moved from ({}, {}) to ({}, {})",
            old_x, old_y, new_x, new_y
        );
    }

    // Debug print
    // println!("Matrix state after physics:");
    // for y in (0..MATRIX_HEIGHT).rev() {
    //     let row: String = chunk_matrix.matrix[y]
    //         .iter()
    //         .map(|&cell| if cell { '#' } else { '.' })
    //         .collect();
    //     println!("{}", row);
    // }
}
