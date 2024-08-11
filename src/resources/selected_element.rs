use crate::components::element::Element;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedElement(pub Element);
