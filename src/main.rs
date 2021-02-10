use bevy::prelude::*;
mod model;

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 300.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(keyboard_input_system.system())
        .run();
}

struct InputText;
const TILE_SIZE: f32 = 100.0;

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {
    let black_tile = asset_server.load("textures/black_tile.png");
    let white_tile1 = asset_server.load("textures/white_tile.png");
    let white_tile2 = asset_server.load("textures/white_tile.png");
    let player = asset_server.load("textures/player.png");
    commands
        // 2d camera
        .spawn(CameraUiBundle::default())
        .spawn((InputText,))
        .with_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "This is where the text appears".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Bottom,
                        horizontal: HorizontalAlign::Center,
                    },
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(black_tile.into()),
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: materials.add(white_tile1.into()),
            transform:Transform{
                translation:Vec3{
                    x:-TILE_SIZE,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: materials.add(white_tile2.into()),
            transform:Transform{
                translation:Vec3{
                    x:TILE_SIZE,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(SpriteBundle {
            material: materials.add(player.into()),
            ..Default::default()
        });
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut text_query: Query<Mut<Text>, With<InputText>>) {
    if keyboard_input.pressed(KeyCode::Up) {
        for mut text in text_query.iter_mut() {
            text.value = "Pressing up".to_string();
        }
    } else if keyboard_input.pressed(KeyCode::Down) {
        for mut text in text_query.iter_mut() {
            text.value = "Pressing down".to_string();
        }
    } else if keyboard_input.pressed(KeyCode::Left) {
        for mut text in text_query.iter_mut() {
            text.value = "Pressing left".to_string();
        }
    } else if keyboard_input.pressed(KeyCode::Right) {
        for mut text in text_query.iter_mut() {
            text.value = "Pressing right".to_string();
        }
    } else {
        for mut text in text_query.iter_mut() {
            text.value = "Nothing Pressed".to_string();
        }
    }
}
