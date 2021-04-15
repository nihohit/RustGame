use bevy::math::vec2;
use bevy::prelude::*;
use rand::prelude::*;

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_boids.system())
        .add_system(coherence.system())
        .add_system(separation.system())
        .add_system(alignment.system())
        .add_system(final_update.system())
        .run();
}

struct Boid {
    next_x: f32,
    next_y: f32,
    velocity: Vec2,
    next_velocity: Vec2,
}

fn setup_boids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let boid = asset_server.load("textures/boid.png");
    const BOID_COUNT: i16 = 200;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut rng = thread_rng();
    for i in 0..BOID_COUNT {
        let transform = Transform::from_xyz(
            rng.gen_range(-500.0..500.0),
            rng.gen_range(-500.0..500.0),
            0.0,
        );
        let velocity = vec2(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(boid.clone().into()),
                transform: transform,
                ..Default::default()
            })
            .insert(Boid {
                next_x: transform.translation.x,
                next_y: transform.translation.y,
                velocity: velocity,
                next_velocity: velocity,
            });
    }
}
