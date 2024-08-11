use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

mod components;
mod resources;
mod systems;
mod utils;

use crate::resources::{MouseState, PlacementSize};
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, (setup::camera, setup::world, setup::ui))
        .insert_resource(MouseState {
            button_pressed: false,
        })
        .insert_resource(PlacementSize::new())
        .add_systems(
            Update,
            (
                utils::camera::edge_scrolling,
                utils::camera::zoom_camera,
                systems::update::placement_shape,
                systems::input::handle_input,
                systems::update::particles,
                systems::update::mouse_state,
            ),
        )
        .run();
}
