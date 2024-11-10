use::bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use rand::Rng;
use std::{f32::INFINITY, time::Duration};


const BOUNDS: Vec2 = Vec2::new(1200.0,640.0);

#[derive(Component, Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
pub struct Particle {
    movement_speed: f32,
    rotation_speed: f32,
    fitness: f32,
    bestfitness: f32,
    pos: Position,
    best_pos: Position,
    velocity: Velocity,
}

#[derive(Component, Debug)]
pub struct Predator {
    pos: Position,
    velocity: Velocity,
}

pub fn setup_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("swarm_bg.png");
    commands.spawn((SpriteBundle {
        texture: texture.clone(),
        sprite: Sprite { 
            ..default()
        },
        ..default()        
    },));
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut camera:Camera2dBundle = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::FixedVertical(500.0);

    commands.spawn(camera);

    let texture: Handle<Image> = asset_server.load("199_f2.png");
    println!("loading texture: {:?}",texture);

    let tile_size = Vec2::splat(32.0);    
    let mut rng = rand::thread_rng();

    let half_x = 5;
    let half_y = 5;

    for y in -half_y..half_y {
        for x in -half_x..half_x{
            let position = Vec2::new(50.0, 100.0);
            let translation  = (position * tile_size).extend(rng.gen::<f32>());
            let rotation = Quat::from_rotation_z(0.0);
            let scale = Vec3::splat(rng.gen::<f32>() * 2.0);
            let mut timer = Timer::from_seconds(0.1, TimerMode::Repeating);
            timer.set_elapsed(Duration::from_secs_f32(rng.gen::<f32>()));
            commands.spawn((SpriteBundle {
                texture: texture.clone(),
                transform: Transform { translation, rotation, scale },
                sprite: Sprite { 
                    custom_size: Some(tile_size),
                    ..default()
                },
                ..default()        
            },
            Particle {
                movement_speed: 400.0,
                rotation_speed: f32::to_radians(360.0),
                fitness: 0.0,
                bestfitness: 0.0,
                pos: Position {x: position.x, y: position.y},
                best_pos: Position {x: position.x, y: position.y},
                velocity: Velocity {x: rand::thread_rng().gen_range(-1.0..1.0), y: rand::thread_rng().gen_range(-1.0..1.0)}
            },
            ));

        }
    }
    let position = Vec2::new(
        rand::thread_rng().gen_range(-BOUNDS.x / 4.0..BOUNDS.x / 4.0), 
        rand::thread_rng().gen_range(-BOUNDS.y / 4.0..BOUNDS.y / 4.0)
    );

    //let position = Vec2::new(rand::thread_rng().gen_range(-100.0..100.0), rand::thread_rng().gen_range(-100.0..100.0));
    let translation  = position.extend(0.0);
    let texture2: Handle<Image> = asset_server.load("129.png");
    let rotation = Quat::from_rotation_z(0.0);
    let scale = Vec3::splat(5.0);
    let mut timer = Timer::from_seconds(0.1, TimerMode::Repeating);
    timer.set_elapsed(Duration::from_secs_f32(rng.gen::<f32>()));
    commands.spawn((SpriteBundle {
        texture: texture2.clone(),
        transform: Transform { translation, rotation, scale },
        sprite: Sprite { 
            custom_size: Some(tile_size),
            ..default()
        },
        ..default()        
        },
        Predator {
            pos: Position {x: position.x, y: position.y},
            velocity: Velocity {x: rand::thread_rng().gen_range(-1.0..1.0), y: rand::thread_rng().gen_range(-1.0..1.0)}
        },
    ));
}


pub fn swarm_impl(
    time: Res<Time>,
    mut query: Query<(&mut Particle, &mut Transform)>,
) {
    let w = 0.95;      
    let c1 = 1.4;     
    let c2 = 1.4;     

    let min_x = -BOUNDS.x / 2.0;
    let max_x = BOUNDS.x / 2.0;

    let min_y = -BOUNDS.y / 2.0;
    let max_y = BOUNDS.y / 2.0;

    let (best_pos_swarm, _best_fitness_swarm) = set_best(&mut query);

    for (mut particle, mut transform) in query.iter_mut() {
        // Calculate the fitness and update the best position if fitness improves
        let fitness = calculate_fitness(&particle);
        if fitness > particle.bestfitness {
            particle.bestfitness = fitness;
            particle.best_pos.x = particle.pos.x;
            particle.best_pos.y = particle.pos.y;
        }

        // Generate random factors r1 and r2
        let r1: f32 = rand::random();
        let r2: f32 = rand::random();

        // Update velocity based on personal and swarm bests
        particle.velocity.x = 
            w * particle.velocity.x +
            r1 * c1 * (particle.best_pos.x - transform.translation.x) +
            r2 * c2 * (best_pos_swarm.x - transform.translation.x);
        particle.velocity.y = 
            w * particle.velocity.y +
            r1 * c1 * (particle.best_pos.y - transform.translation.y) +
            r2 * c2 * (best_pos_swarm.y - transform.translation.y);

        // Update translation based on velocity
        transform.translation.x += particle.velocity.x * 0.01;
        transform.translation.y += particle.velocity.y * 0.01;

        // Clamp position within bounds
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);

        // Update particle position to match the new translation
        particle.pos.x = transform.translation.x;
        particle.pos.y = transform.translation.y;
    }
}

pub fn set_best(query: &mut Query<(&mut Particle, &mut Transform)>) -> (Position, f32) {
    let mut all_best_fit: f32 = -(std::f32::INFINITY);
    let mut all_best_pos: Position = Position { x: std::f32::INFINITY, y: std::f32::INFINITY };

    for (mut particle, mut _transform) in query.iter_mut() {
        let calc = calculate_fitness(&particle);
        if calc > particle.bestfitness {
            particle.bestfitness = calc;
            particle.best_pos.x = particle.pos.x;
            particle.best_pos.y = particle.pos.y;
            //println!("Im being updated!");
        }

        if particle.bestfitness > all_best_fit {
            all_best_fit = particle.bestfitness;
            all_best_pos = Position { x: particle.best_pos.x, y: particle.best_pos.y };
            //println!("Updated")
        }
        //println!("{}", particle.pos.x);
       //println!("{}", particle.pos.y);
    }

    //println!("{}", all_best_pos.x);
    //println!("{}", all_best_pos.y);
    //println!("{}", all_best_fit);

    return (all_best_pos, all_best_fit);
}

fn calculate_fitness(p: &Particle) -> f32 {
    let optimal_pos = Position { x: -300.0, y: 0.0 };
    let part_pos = &p.pos;
    
    let dist = get_dist(part_pos, &optimal_pos);
    let fitness = 1.0 - (dist * 0.001);

    //println!("FITNESS {}", fitness);
    
    return fitness;
}

pub fn predator_move(
    time: Res<Time>,
    mut q: Query<(&mut Predator, &mut Transform)>,
    particle_query: Query<&Particle>,
) {
    let mut min_dist: f32 = f32::INFINITY;
    let mut min_part_pos = Position { x: f32::INFINITY, y: f32::INFINITY };

    let (mut pred, mut transform) = q.single_mut();
    let pred_pos: &Position = &pred.pos;

    // Find the closest particle to the predator
    for particle in particle_query.iter() {
        let new_dist = get_dist(pred_pos, &particle.pos);
        if new_dist < min_dist {
            min_dist = new_dist;
            min_part_pos = Position { x: particle.pos.x, y: particle.pos.y };
        }
    }

    // Define a threshold distance to stop moving towards the particle
    let stopping_threshold = 50.0; // Adjust this value to control how close the predator gets before stopping

    // Only move if the predator is farther than the stopping threshold
    if min_dist > stopping_threshold {
        println!("min_dist {}",min_dist);
        // Calculate the normalized direction vector towards the closest particle
        let to_part = Vec2::new(min_part_pos.x - pred_pos.x, min_part_pos.y - pred_pos.y);
        let to_part_normalized = to_part.normalize();

        // Continuously update the rotation to face the particle
        let rotate_to_part = Quat::from_rotation_arc(Vec3::Y, to_part_normalized.extend(0.));
        let offset_rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.0);
        transform.rotation = rotate_to_part * offset_rotation;

        // Move the predator in the direction it’s facing
        let speed = 100.0; // Adjust speed as needed
        let delta_time = time.delta_seconds();
        transform.translation.x += to_part_normalized.x * speed * delta_time;
        transform.translation.y += to_part_normalized.y * speed * delta_time;
    } else {
        // Keep facing the closest particle without moving
        println!("AHHHHHHHHHHHHHHHH");
        let to_part = Vec2::new(min_part_pos.x - pred_pos.x, min_part_pos.y - pred_pos.y).normalize();
        let rotate_to_part = Quat::from_rotation_arc(Vec3::Y, to_part.extend(0.));
        let offset_rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.0);
        transform.rotation = rotate_to_part * offset_rotation;
    }
    pred.pos.x = transform.translation.x;
    pred.pos.y = transform.translation.y;

}


fn get_dist(pos_a: &Position, pos_b: &Position) -> f32 {
    return f32::sqrt((pos_a.x - pos_b.x).powf(2.0) + (pos_a.y - pos_b.y).powf(2.0));
}

// pub fn particle_movement_system(
//     time: Res<Time>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<(&mut Particle,&mut Transform)>,
// ) {
//     let mut rotation_factor = 0.0;
//     let mut movement_factor = 0.0;

//     if keyboard_input.pressed(KeyCode::ArrowLeft) {
//         rotation_factor -= 1.0;
//     }

//     if keyboard_input.pressed(KeyCode::ArrowRight) {
//         rotation_factor += 1.0;
//     }

//     if keyboard_input.pressed(KeyCode::ArrowDown) {
//         movement_factor -= 1.0;
//     }

//     if keyboard_input.pressed(KeyCode::ArrowUp) {
//         movement_factor += 1.0;
//     }

//     for (mut thing, mut transform) in query.iter_mut() {

//         transform.rotate_z(rotation_factor * thing.rotation_speed * time.delta_seconds());

//         let movement_direction = transform.rotation * Vec3::Y;
//         let movement_distance = movement_factor * thing.movement_speed * time.delta_seconds();
//         let translation_delta = movement_distance * movement_direction;
    
//         transform.translation += translation_delta;
    
//         let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        
//         transform.translation = transform.translation.min(extents).max(-extents);

//         thing.velocity.x = translation_delta.x;
//         thing.velocity.y = translation_delta.y;

//         thing.pos.x += translation_delta.x;
//         thing.pos.y += translation_delta.y;

//         //println!("{}", thing.velocity.x);
//         //println!("{}", thing.velocity.y);

        
//     }
// }