use crate::components::element::Element;
use crate::resources::particle_matrix::ParticleMatrix;
use crate::utils::particles::helper::*;
use rand::Rng;

pub fn simulate_gas(
    x: usize,
    y: usize,
    particle_matrix: &ParticleMatrix,
    rng: &mut impl Rng,
    element: &Element,
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
            && rng.gen_bool(1.0 - element.dispersion_rate as f64 / 100.0)
        {
            (x as usize, (y - 1) as usize) // Chance to move down (sinking effect) inversely based on dispersion rate
        } else {
            (x as usize, y as usize)
        }
    }
}
