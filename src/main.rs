use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::system::EntityCommands;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::ui::*;
use rand::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct Calculations;

const HALF_SIZE: f32 = 500.;

#[derive(Component)]
struct SliderButton;
#[derive(Component)]
struct SliderText;
#[derive(Component)]
struct SliderCoherenceRange;
#[derive(Component)]
struct SliderCoherenceStrength;
#[derive(Component)]
struct SliderSeparationRange;
#[derive(Component)]
struct SliderSeparationStrength;
#[derive(Component)]
struct SliderAlignmentRange;
#[derive(Component)]
struct SliderAlignmentStrength;

#[derive(Clone, Debug, Component)]
struct Slider {
    min: f32,
    max: f32,
    value: f32,
}

// Copied from ButtonBundle
#[derive(Bundle, Clone, Debug)]
struct SliderBundle {
    pub slider: Slider,
    pub node: Node,
    pub style: Style,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
    pub color: UiColor,
    pub image: UiImage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
}

impl Default for SliderBundle {
    fn default() -> Self {
        SliderBundle {
            slider: Slider {
                min: 0f32,
                max: 1f32,
                value: 0f32,
            },
            interaction: Default::default(),
            focus_policy: Default::default(),
            node: Default::default(),
            style: Default::default(),
            color: Default::default(),
            image: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
        }
    }
}

trait SpawnSlider<'w, 's> {
    fn spawn_slider<'a>(
        &'a mut self,
        asset_server: &Res<AssetServer>,
        min: f32,
        max: f32,
        position: Vec2,
        title: &str,
    ) -> EntityCommands<'w, 's, 'a>;
}

fn update_slider_button(
    button_style: &mut Style,
    button_node: &Node,
    slider: &Slider,
    slider_node: &Node,
) {
    let normalized_x = (slider.value - slider.min) / (slider.max - slider.min);
    let x = normalized_x * slider_node.size.x - button_node.size.x * 0.5;
    button_style.position.left = Val::Px(x);
}

fn update_slider_text(text: &mut Text, slider: &Slider) {
    text.sections[1].value = format!("{:.}", slider.value);
}

impl<'w, 's> SpawnSlider<'w, 's> for Commands<'w, 's> {
    fn spawn_slider<'a>(
        &'a mut self,
        asset_server: &Res<AssetServer>,
        min: f32,
        max: f32,
        position: Vec2,
        title: &str,
    ) -> EntityCommands<'w, 's, 'a> {
        let mut entity_commands = self.spawn_bundle(SliderBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(20.0)),
                position: Rect {
                    left: Val::Px(position.x),
                    top: Val::Px(position.y),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: Color::rgb(1.0, 0.0, 0.0).into(),
            slider: Slider {
                min: min,
                max: max,
                value: min + (max - min) * 0.5,
            },
            ..Default::default()
        });
        entity_commands.with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                        position: Rect {
                            // left: Val::Px(125.0),
                            top: Val::Px(-15.0),
                            ..Default::default()
                        },
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    color: Color::rgb(0.0, 1.0, 0.0).into(),
                    ..Default::default()
                })
                .insert(SliderButton);
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        position: Rect {
                            top: Val::Px(50.0),
                            ..Default::default()
                        },
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    // Use `Text` directly
                    text: Text {
                        // Construct a `Vec` of `TextSection`s
                        sections: vec![
                            TextSection {
                                value: title.to_string() + " ",
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::GOLD,
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(SliderText);
        });

        return entity_commands;
    }
}

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: HALF_SIZE * 2.0,
            height: HALF_SIZE * 2.0,
            vsync: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_boids)
        .add_startup_system(setup_ui)
        .add_system(coherence_update.label(Calculations))
        .add_system(separation_update.label(Calculations))
        .add_system(alignment_update.label(Calculations))
        .add_system(text_update)
        .add_system(slider_update.label(Calculations))
        .add_system(final_update.after(Calculations))
        .add_system(slider_button_position_update.after(Calculations))
        .run();
}

#[derive(Component)]
struct Coherence {
    center: Vec2,
}

#[derive(Component)]
struct Separation {
    center: Vec2,
}

#[derive(Component)]
struct Alignment {
    direction: Vec2,
}

#[derive(Component)]
struct Velocity {
    direction: Vec2,
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
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

    commands
        .spawn_slider(
            &asset_server,
            0.001,
            0.2,
            vec2(20.0, 600.0),
            "coherence strength",
        )
        .insert(SliderCoherenceStrength);
    commands
        .spawn_slider(
            &asset_server,
            50.0,
            150.0,
            vec2(20.0, 800.0),
            "coherence range",
        )
        .insert(SliderCoherenceRange);
    commands
        .spawn_slider(
            &asset_server,
            0.001,
            0.2,
            vec2(360.0, 600.0),
            "separation strength",
        )
        .insert(SliderSeparationStrength);
    commands
        .spawn_slider(
            &asset_server,
            10.0,
            100.0,
            vec2(360.0, 800.0),
            "separation range",
        )
        .insert(SliderSeparationRange);
    commands
        .spawn_slider(
            &asset_server,
            0.001,
            0.1,
            vec2(700.0, 600.0),
            "alignment strength",
        )
        .insert(SliderAlignmentStrength);
    commands
        .spawn_slider(
            &asset_server,
            20.0,
            150.0,
            vec2(700.0, 800.0),
            "alignment range",
        )
        .insert(SliderAlignmentRange);
}

fn setup_boids(mut commands: Commands, asset_server: Res<AssetServer>) {
    const BOID_COUNT: i16 = 200;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut rng = thread_rng();
    let texture: Handle<Image> = asset_server.load("textures/Arrow.png");
    for _ in 0..BOID_COUNT {
        let transform = Transform::from_xyz(
            rng.gen_range(-HALF_SIZE..HALF_SIZE),
            rng.gen_range(-HALF_SIZE..HALF_SIZE),
            0.0,
        );
        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                transform: transform,
                ..Default::default()
            })
            .insert(Coherence { center: Vec2::ZERO })
            .insert(Separation { center: Vec2::ZERO })
            .insert(Alignment {
                direction: Vec2::ZERO,
            })
            .insert(Velocity {
                direction: Vec2::ZERO,
            });
    }
}

fn coherence_update(
    mut boids: Query<(Entity, &Transform, &mut Coherence)>,
    other_boids: Query<(Entity, &Transform)>,
    slider_query: Query<&Slider, With<SliderCoherenceRange>>,
) {
    for (entity, transform, mut coherence) in boids.iter_mut() {
        let mut new_center = Vec2::ZERO;
        let mut count: f32 = 0.0;
        for (other_entity, other_transform) in other_boids.iter() {
            let distance = transform.translation.distance(other_transform.translation);
            let slider = slider_query.single();
            if other_entity != entity && distance < slider.value {
                new_center +=
                    Vec2::new(other_transform.translation.x, other_transform.translation.y);
                count += 1.0;
            }
        }
        coherence.center = if count == 0.0 {
            new_center
        } else {
            new_center / count
        };
    }
}

fn separation_update(
    mut boids: Query<(Entity, &Transform, &mut Separation)>,
    other_boids: Query<(Entity, &Transform)>,
    slider_query: Query<&Slider, With<SliderSeparationRange>>,
) {
    for (entity, transform, mut separation) in boids.iter_mut() {
        let mut new_center = Vec2::ZERO;
        for (other_entity, other_transform) in other_boids.iter() {
            let offset = transform.translation - other_transform.translation;
            let distance = offset.length();
            let slider = slider_query.single();
            if other_entity != entity && distance <= slider.value {
                new_center += Vec2::new(offset.x, offset.y);
            }
        }
        separation.center = new_center;
    }
}

fn alignment_update(
    mut boids: Query<(Entity, &Transform, &mut Alignment)>,
    other_boids: Query<(Entity, &Transform, &Velocity)>,
    slider_query: Query<&Slider, With<SliderAlignmentRange>>,
) {
    for (entity, transform, mut alignment) in boids.iter_mut() {
        let mut new_velocity = Vec2::ZERO;
        let mut count: f32 = 0.0;
        for (other_entity, other_transform, other_velocity) in other_boids.iter() {
            let distance = transform.translation.distance(other_transform.translation);
            let slider = slider_query.single();
            if other_entity != entity && distance < slider.value {
                new_velocity += other_velocity.direction * distance / slider.value;
                count += 1.0;
            }
        }
        alignment.direction = if count == 0.0 {
            new_velocity
        } else {
            new_velocity / count
        };
    }
}

fn normalize_or_zero(vec: Vec2) -> Vec2 {
    if vec == Vec2::ZERO {
        return vec;
    } else {
        return vec.normalize();
    }
}

fn final_update(
    mut boids: Query<(
        &mut Transform,
        &Coherence,
        &Separation,
        &Alignment,
        &mut Velocity,
    )>,
    time: Res<Time>,
    coherence_slider_query: Query<&Slider, With<SliderCoherenceStrength>>,
    separation_slider_query: Query<&Slider, With<SliderSeparationStrength>>,
    alignment_slider_query: Query<&Slider, With<SliderAlignmentStrength>>,
) {
    const MAX_SPEED: f32 = 100.0;
    let delta: f32 = (time.delta().as_millis()) as f32 / 1000.0;
    for (mut transform, coherence, separation, alignment, mut velocity) in boids.iter_mut() {
        let current_location = vec2(transform.translation.x, transform.translation.y);
        let coherence_change = coherence.center - current_location;
        let separation_change = separation.center;
        let coherence_slider = coherence_slider_query.single();
        let separation_slider = separation_slider_query.single();
        let alignment_slider = alignment_slider_query.single();
        velocity.direction += separation_change * separation_slider.value
            + coherence_change * coherence_slider.value
            + alignment.direction * alignment_slider.value;

        //limit max speed
        if velocity.direction.length() > MAX_SPEED {
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
    }
}

fn text_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.0}", average);
            }
        }
    }
}

fn slider_update(
    mut slider_query: Query<(
        &Interaction,
        &mut Slider,
        &GlobalTransform,
        &Node,
        &Children,
    )>,
    button_query: Query<&Interaction, With<SliderButton>>,
    windows: Res<Windows>,
) {
    let win = windows.get_primary().expect("no primary window");
    if let Some(mouse_xy) = win.cursor_position() {
        for (slider_interaction, mut slider, slider_transform, slider_node, children) in
            slider_query.iter_mut()
        {
            let button_interaction = button_query.get(children[0]).unwrap();
            if *slider_interaction != Interaction::Clicked
                && *button_interaction != Interaction::Clicked
            {
                continue;
            }
            let x = slider_transform.translation.x;
            let width = slider_node.size.x;
            let offset = width * 0.5;
            let normalized_x = (mouse_xy.x + offset - x) / width;
            slider.value = (normalized_x * (slider.max - slider.min) + slider.min)
                .clamp(slider.min, slider.max);
        }
    }
}

fn slider_button_position_update(
    mut button_query: Query<(&mut Style, &Node), With<SliderButton>>,
    mut text_query: Query<&mut Text, With<SliderText>>,
    slider_query: Query<(&Slider, &Node, &Children), Changed<Slider>>,
) {
    for (slider, slider_node, children) in slider_query.iter() {
        let (mut button_style, button_node) = button_query.get_mut(children[0]).unwrap();
        update_slider_button(&mut *button_style, button_node, slider, slider_node);

        let mut text = text_query.get_mut(children[1]).unwrap();
        update_slider_text(&mut *text, slider);
    }
}
