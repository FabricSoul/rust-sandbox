// Add this helper function to check if a position is within the matrix bounds

use crate::utils::constants::*;

use crate::resources::particle_matrix::ParticleMatrix;

pub fn is_in_bounds(x: isize, y: isize) -> bool {
    x >= 0 && x < MATRIX_WIDTH as isize && y >= 0 && y < MATRIX_HEIGHT as isize
}

// Add this helper function to check if a position is empty
pub fn is_empty(particle_matrix: &ParticleMatrix, x: usize, y: usize) -> bool {
    particle_matrix.matrix[y][x].is_none()
}
