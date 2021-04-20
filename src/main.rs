use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::tasks::prelude::*;
use rand::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct Calculations;

const HALF_SIZE: f32 = 500.;

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: HALF_SIZE * 2.0,
            height: HALF_SIZE * 2.0,
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_boids.system())
        .add_startup_system(setup_ui.system())
        .add_system(coherence_update.system().label(Calculations))
        .add_system(separation_update.system().label(Calculations))
        .add_system(alignment_update.system().label(Calculations))
        .add_system(final_update.system().after(Calculations))
        .add_system(text_update.system())
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

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn setup_boids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let boid = asset_server.load("textures/Arrow.png");
    const BOID_COUNT: i16 = 400;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut rng = thread_rng();
    for _ in 0..BOID_COUNT {
        let transform = Transform::from_xyz(
            rng.gen_range(-HALF_SIZE..HALF_SIZE),
            rng.gen_range(-HALF_SIZE..HALF_SIZE),
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

const BATCH_SIZE: usize = 32;

const COHERENCE_DISTANCE: f32 = 50.0;
const SEPARATION_DISTANCE: f32 = 10.0;

fn coherence_update(
    mut boids: Query<(Entity, &Transform, &mut Coherence)>,
    other_boids: Query<(Entity, &Transform)>,
    pool: Res<ComputeTaskPool>,
) {
    boids.par_for_each_mut(&pool, BATCH_SIZE, |(entity, transform, mut coherence)| {
        let mut new_center = Vec2::ZERO;
        let mut count: f32 = 0.0;
        for (other_entity, other_transform) in other_boids.iter() {
            let distance = transform.translation.distance(other_transform.translation);
            if other_entity != entity && distance < COHERENCE_DISTANCE {
                new_center += Vec2::from(other_transform.translation);
                count += 1.0;
            }
        }
        coherence.center = if count == 0.0 {
            new_center
        } else {
            new_center / count
        };
    });
}

fn separation_update(
    mut boids: Query<(Entity, &Transform, &mut Separation)>,
    other_boids: Query<(Entity, &Transform)>,
    pool: Res<ComputeTaskPool>,
) {
    boids.par_for_each_mut(&pool, BATCH_SIZE, |(entity, transform, mut separation)| {
        let mut new_center = Vec2::ZERO;
        let mut count: f32 = 0.0;
        for (other_entity, other_transform) in other_boids.iter() {
            let distance = transform.translation.distance(other_transform.translation);
            if other_entity != entity && distance <= SEPARATION_DISTANCE {
                new_center += Vec2::from(other_transform.translation);
                count += 1.0;
            }
        }
        separation.center = if count == 0.0 {
            new_center
        } else {
            new_center / count
        };
    });
}

fn alignment_update(
    mut boids: Query<(Entity, &Transform, &mut Alignment)>,
    other_boids: Query<(Entity, &Transform, &Velocity)>,
    pool: Res<ComputeTaskPool>,
) {
    boids.par_for_each_mut(&pool, BATCH_SIZE, |(entity, transform, mut alignment)| {
        let mut new_velocity = Vec2::ZERO;
        let mut count: f32 = 0.0;
        for (other_entity, other_transform, other_velocity) in other_boids.iter() {
            let distance = transform.translation.distance(other_transform.translation);
            if other_entity != entity && distance < COHERENCE_DISTANCE {
                new_velocity += other_velocity.direction;
                count += 1.0;
            }
        }
        alignment.direction = if count == 0.0 {
            new_velocity
        } else {
            new_velocity / count
        };
    });
}

fn normalize_or_zero(vec: Vec2) -> Vec2 {
    if vec == Vec2::ZERO {
        return vec;
    } else {
        return vec.normalize();
    }
}

const MAX_SPEED: f32 = 75.0;

fn final_update(
    mut boids: Query<(
        &mut Transform,
        &Coherence,
        &Separation,
        &Alignment,
        &mut Velocity,
    )>,
    time: Res<Time>,
    pool: Res<ComputeTaskPool>,
) {
    let delta: f32 = (time.delta().as_millis()) as f32 / 1000.0;
    boids.par_for_each_mut(&pool, BATCH_SIZE, |(mut transform, coherence, separation, alignment, mut velocity)| {
        let current_location = vec2(transform.translation.x, transform.translation.y);
        let coherence_change = normalize_or_zero(coherence.center - current_location);
        let separation_change = normalize_or_zero(current_location - separation.center);
        velocity.direction +=
            normalize_or_zero((separation_change + coherence_change * 10.0) + alignment.direction);

        //limit max speed
        if velocity.direction.distance(Vec2::ZERO) > MAX_SPEED {
            velocity.direction = velocity.direction.normalize() * MAX_SPEED;
        }

        transform.translation.x += velocity.direction.x * delta;
        transform.translation.y += velocity.direction.y * delta;

        // bind to torus
        if transform.translation.x < -HALF_SIZE {
            transform.translation.x = HALF_SIZE;
        }
        if transform.translation.x > HALF_SIZE {
            transform.translation.x = -HALF_SIZE;
        }
        if transform.translation.y < -HALF_SIZE {
            transform.translation.y = HALF_SIZE;
        }
        if transform.translation.y > HALF_SIZE {
            transform.translation.y = -HALF_SIZE;
        }

        let direction = normalize_or_zero(velocity.direction);
        transform.rotation = Quat::from_rotation_z(-direction.angle_between(Vec2::Y));
    });
}

fn text_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}
