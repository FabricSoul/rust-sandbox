use crate::utils::constants::{MATRIX_HEIGHT, MATRIX_WIDTH};
use bevy::prelude::*;

// Resources
#[derive(Resource)]
pub struct ParticleMatrix {
    pub matrix: Vec<Vec<Option<Entity>>>,
}

impl ParticleMatrix {
    pub fn new() -> Self {
        ParticleMatrix {
            matrix: vec![vec![None; MATRIX_WIDTH]; MATRIX_HEIGHT],
        }
    }
}
