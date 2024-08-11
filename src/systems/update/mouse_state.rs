use crate::resources::mouse_state::MouseState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

pub fn mouse_state(
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut mouse_state: ResMut<MouseState>,
) {
    for event in mouse_button_events.read() {
        if event.button == MouseButton::Left {
            mouse_state.button_pressed = event.state.is_pressed();
        }
    }
}
