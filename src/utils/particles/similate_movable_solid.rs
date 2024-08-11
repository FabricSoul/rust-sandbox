use crate::components::element::Element;
use crate::resources::particle_matrix::ParticleMatrix;
use crate::utils::particles::helper::*;
use rand::Rng;

pub fn simulate_movable_solid(
    x: usize,
    y: usize,
    particle_matrix: &ParticleMatrix,
    rng: &mut impl Rng,
    element: &Element,
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
            if rng.gen_bool(0.5 - element.friction as f64 / 2.0) {
                ((x - 1) as usize, (y - 1) as usize)
            } else {
                ((x + 1) as usize, (y - 1) as usize)
            }
        } else if down_left {
            ((x - 1) as usize, (y - 1) as usize)
        } else if down_right {
            ((x + 1) as usize, (y - 1) as usize)
        } else {
            (x as usize, y as usize)
        }
    }
}
