use::bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use rand::Rng;
use std::time::Duration;


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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut camera:Camera2dBundle = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::FixedVertical(500.0);

    commands.spawn(camera);

    let texture: Handle<Image> = asset_server.load("199_f2.png");
    println!("loading texture: {:?}",texture);

    let map_size = Vec2::splat(320.0);
    let tile_size = Vec2::splat(32.0);    
    let mut rng = rand::thread_rng();

    let half_x = 5;
    let half_y = 5;

    for y in -half_y..half_y {
        for x in -half_x..half_x{
            let position = Vec2::new(x as f32, y as f32);
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
                velocity: Velocity {x: 0.0, y:0.0}
            },
            ));
        }
    }
    

}

pub fn particle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Particle,&mut Transform)>,
) {
    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        movement_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        movement_factor += 1.0;
    }

    for (mut thing, mut transform) in query.iter_mut() {

        transform.rotate_z(rotation_factor * thing.rotation_speed * time.delta_seconds());

        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = movement_factor * thing.movement_speed * time.delta_seconds();
        let translation_delta = movement_distance * movement_direction;
    
        transform.translation += translation_delta;
    
        let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        
        transform.translation = transform.translation.min(extents).max(-extents);

        thing.velocity.x = translation_delta.x;
        thing.velocity.y = translation_delta.y;

        thing.pos.x += translation_delta.x;
        thing.pos.y += translation_delta.y;

        //println!("{}", thing.velocity.x);
        //println!("{}", thing.velocity.y);

        
    }
}

pub fn swarm_impl(d: i32, mut query: Query<(&Particle,&mut Transform)>,) {
    // for iter in range(max_iter) to loop several times over population
    for (thing, mut transform) in query.iter_mut() {
        // a. Compute new velocity of ith particle
        //thing.velocity.x = thing.velocity + (thing.best_pos - thing.pos);
    }
}

pub fn set_best(mut query: Query<(&mut Particle, &mut Transform)>) {
    for (mut thing, mut _transform) in query.iter_mut() {
        if calculate_fitness(&thing) > thing.bestfitness {
            thing.bestfitness = calculate_fitness(&thing);
            thing.best_pos.x = thing.pos.x;
            thing.best_pos.y = thing.pos.y;
            println!("{}", thing.best_pos.x);
            println!("{}", thing.best_pos.y);
        }
    }
}

fn calculate_fitness(p: &Particle) -> f32 {
    let optimal_pos = Position {x: 250.0, y: 250.0};
    let part_pos = &p.pos;
    let dist = (part_pos.x - optimal_pos.x).powf(2.0) + (part_pos.y - optimal_pos.y).powf(2.0);
    let fitness = 1.0 - (f32::sqrt(dist) * 0.001);
    return fitness;
}

