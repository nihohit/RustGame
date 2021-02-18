use bevy::prelude::*;

mod model;

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 500.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_ui.system())
        .add_startup_system(setup_board.system())
        .add_startup_system_to_stage("post_startup", setup_piece_transforms.system())
        .add_system(keyboard_input.system())
        .add_system(update_transforms.system())
        .run();
}

struct InputText;

struct Position {
    x: i16,
    y: i16
}

struct Selected;

const TILE_SIZE: f32 = 100.0;

fn setup_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
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
        });
}

fn coords_to_transform(x: i16, y: i16) -> Transform {
    return Transform {
        translation: Vec3 {
            x: f32::from(x) * TILE_SIZE,
            y: f32::from(y) * TILE_SIZE,
            z: 0.0,
        },
        ..Default::default()
    };
}

fn setup_board(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let black_tile = asset_server.load("textures/black_tile.png");
    let white_tile = asset_server.load("textures/white_tile.png");

    let render_tile_material = |entity_type: &model::TileColor| match entity_type {
        model::TileColor::White => white_tile.clone().into(),
        model::TileColor::Black => black_tile.clone().into(),
    };
    let board = model::create_board();

    commands.spawn(Camera2dBundle::default());

    let middle_row_index: i16 = (board.tiles.len() / 2) as i16;
    for row_index in 0..board.tiles.len() as i16 {
        let row = &board.tiles[row_index as usize];
        let middle_index: i16 = (row.len() / 2) as i16;
        for tile_index in 0..row.len() as i16 {
            let tile = &row[tile_index as usize];
            let transform =
                coords_to_transform(tile_index - middle_index, row_index - middle_row_index);
            commands.spawn(SpriteBundle {
                material: materials.add(render_tile_material(&tile.color)),
                transform: transform,
                ..Default::default()
            });

            match tile.entity {
                None => {}
                Some(entity_type) => {
                    commands.spawn((entity_type, Position{x: tile_index - middle_index, y: row_index - middle_row_index}));
                }
            }
        }
    }

    let row = board.tiles[board.player.x as usize];
    let middle_index: i16 = (row.len() / 2) as i16;
    commands.spawn((board.player.entity_type, Position{x: board.player.x - middle_index, y: board.player.y - middle_row_index}, Selected));
}

fn setup_piece_transforms(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pieces: Query<(Entity, &model::EntityType, &Position)>
) {
    let white_piece = asset_server.load("textures/WhitePiece.png");
    let black_piece = asset_server.load("textures/BlackPiece.png");

    let render_entity_material = |entity_type: &model::EntityType| match entity_type {
        model::EntityType::WhitePiece => white_piece.clone().into(),
        model::EntityType::BlackPiece => black_piece.clone().into(),
    };
    for (entity, entity_type, position) in pieces.iter() {
        commands.insert(entity,
            SpriteBundle {
                material: materials.add(render_entity_material(entity_type)),
                ..Default::default()
            });
    }
}

fn update_transforms(
    mut transforms_query: Query<(&Position, &mut Transform)>,
) {
    for (position, mut transform) in transforms_query.iter_mut() {
        *transform = coords_to_transform(position.x, position.y);
    }
}

fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut selected_query: Query<Mut<Position>, With<Selected>>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        for mut position in selected_query.iter_mut() {
            position.y += 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        for mut position in selected_query.iter_mut() {
            position.y -= 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        for mut position in selected_query.iter_mut() {
            position.x -= 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        for mut position in selected_query.iter_mut() {
            position.x += 1;
        }
    }
}
