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
    otherBoids: Query<(Entity, &Transform)>,
) {
    const COHERENCE_DISTANCE: f32 = 20;
    for (entity, transform, boid) in boids.iter_mut() {
        let mut coherence = Vec3::ZERO;
        let mut count = 0;
        for (otherEntity, otherTransform) in otherBoids.iter() {
            if otherEntity != entity
                && transform.translation.distance(otherTransform.translation) < COHERENCE_DISTANCE
            {
                coherence += otherTransform.translation;
                count += 1;
            }
        }
    }
}

fn final_update(boids: Query<(Entity, &Transform, &mut Boid)>) {}
