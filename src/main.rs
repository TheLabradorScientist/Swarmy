use bevy::input::common_conditions::input_toggle_active;
use bevy::{prelude::*, render::camera::ScalingMode};
mod math;
mod spawner;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Galileo".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(HelloPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, spawner::setup)
        .add_systems(FixedUpdate, (
            spawner::particle_movement_system,
            spawner::set_best,
        ))
        .run();
}


fn hello_world() {
    //println!("HIIII");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("BOB".to_string())));
    commands.spawn((Person, Name("JOEY".to_string())));
}


fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>){
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query{
            println!("Hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>){
    for mut name in &mut query{
        if name.0 == "BOB"{
            name.0 = "PADDINGTON".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App){      
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0,TimerMode::Repeating)));  
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

