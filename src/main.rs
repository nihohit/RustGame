use bevy::math::vec2;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct Calculations;

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
        .add_system(coherence.system().label(Calculations))
        // .add_system(separation.system().label(Calculations))
        // .add_system(alignment.system().label(Calculations))
        .add_system(final_update.system().after(Calculations))
        .run();
}

struct Boid {
    coherence_result: Vec2,
    separation_result: Vec2,
    alignemnt_result: Vec2,
    velocity: Vec2,
}

fn setup_boids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let boid = asset_server.load("textures/black_tile.png");
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
                coherence_result: Vec2::ZERO,
                separation_result: Vec2::ZERO,
                alignemnt_result: velocity,
                velocity: velocity,
            });
    }
}

fn coherence(
    mut boids: Query<(Entity, &Transform, &mut Boid)>,
    other_boids: Query<(Entity, &Transform)>,
) {
    const COHERENCE_DISTANCE: f32 = 20.0;
    for (entity, transform, mut boid) in boids.iter_mut() {
        let mut coherence = Vec3::ZERO;
        let mut count: f32 = 0.0;
        for (other_entity, other_transform) in other_boids.iter() {
            if other_entity != entity
                && transform.translation.distance(other_transform.translation) < COHERENCE_DISTANCE
            {
                coherence += other_transform.translation;
                count += 1.0;
            }
        }
        boid.coherence_result = vec2(coherence.x / count, coherence.y / count);
    }
}

fn final_update(mut boids: Query<(&mut Transform, &mut Boid)>, time: Res<Time>) {
    let delta: f32 = (time.delta().as_millis()) as f32 / 1000.0;
    for (mut transform, boid) in boids.iter_mut() {
        let current_location = vec2(transform.translation.x, transform.translation.y);
        let change = boid.velocity + boid.coherence_result - current_location;
        transform.translation.x += change.x * delta;
        transform.translation.y += change.y * delta;
    }
}
