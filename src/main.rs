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
            width: 800.,
            height: 800.,
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_boids.system())
        .add_system(coherence_update.system().label(Calculations))
        .add_system(separation_update.system().label(Calculations))
        // .add_system(alignment.system().label(Calculations))
        .add_system(final_update.system().after(Calculations))
        .run();
}

struct Coherence {
    center: Vec2,
}

struct Separation {
    center: Vec2,
}

struct Alignment {
    direction: Vec2,
}

struct Velocity {
    direction: Vec2,
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
            .insert(Coherence { center: Vec2::ZERO })
            .insert(Separation { center: Vec2::ZERO })
            .insert(Alignment {
                direction: Vec2::ZERO,
            })
            .insert(Velocity {
                direction: velocity,
            });
    }
}

const COHERENCE_DISTANCE: f32 = 50.0;
const SEPARATION_DISTANCE: f32 = 10.0;

fn coherence_update(
    mut boids: Query<(Entity, &Transform, &mut Coherence)>,
    other_boids: Query<(Entity, &Transform)>,
) {
    for (entity, transform, mut coherence) in boids.iter_mut() {
        let mut new_center = Vec3::ZERO;
        let mut count: f32 = 0.0;
        for (other_entity, other_transform) in other_boids.iter() {
            let distance = transform.translation.distance(other_transform.translation);
            if other_entity != entity
                && distance < COHERENCE_DISTANCE
                && distance > SEPARATION_DISTANCE
            {
                new_center += other_transform.translation;
                count += 1.0;
            }
        }
        coherence.center = vec2(new_center.x / count, new_center.y / count);
    }
}

fn separation_update(
    mut boids: Query<(Entity, &Transform, &mut Separation)>,
    other_boids: Query<(Entity, &Transform)>,
) {
    for (entity, transform, mut separation) in boids.iter_mut() {
        let mut new_center = Vec3::ZERO;
        let mut count: f32 = 0.0;
        for (other_entity, other_transform) in other_boids.iter() {
            let distance = transform.translation.distance(other_transform.translation);
            if other_entity != entity && distance <= SEPARATION_DISTANCE {
                new_center += other_transform.translation;
                count += 1.0;
            }
        }
        separation.center = vec2(new_center.x / count, new_center.y / count);
    }
}

fn final_update(
    mut boids: Query<(
        &mut Transform,
        &Coherence,
        &Separation,
        &Alignment,
        &Velocity,
    )>,
    time: Res<Time>,
) {
    let delta: f32 = (time.delta().as_millis()) as f32 / 1000.0;
    for (mut transform, coherence, separation, alignment, velocity) in boids.iter_mut() {
        let current_location = vec2(transform.translation.x, transform.translation.y);
        let change = velocity.direction + coherence.center - current_location;
        transform.translation.x += change.x * delta;
        transform.translation.y += change.y * delta;
    }
}
