use crate::utils::camera::*;
use bevy::prelude::*;

pub fn camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(RotatingCamera);
}
