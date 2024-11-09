use matplotlib;
use ndarray;
use bevy::prelude::*;
use rand::Rng;

fn setup() {
    // Create particles
    let mut random = rand::thread_rng();
    let n_particles: f32 = 20.0;
    let x = random.gen_range(2.0..n_particles) * 5.0;
    let y = random.gen_range(2.0..n_particles) * 0.1;

    // Initialize data
    let pbest = x;
    let pbest_obj = y;
}