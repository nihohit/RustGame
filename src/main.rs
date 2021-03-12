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
        .add_startup_system_to_stage("post_startup", setup_piece_sprites.system())
        .add_system(keyboard_input.system())
        .add_system(update_transforms.system())
        .run();
}

struct InputText;

struct Selected;

const TILE_SIZE: f32 = 100.0;
const MAX_X_POSITION: i16 = 2;
const MAX_Y_POSITION: i16 = 2;
const MIN_X_POSITION: i16 = 0;
const MIN_Y_POSITION: i16 = 0;

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
            x: f32::from(x - 1) * TILE_SIZE,
            y: f32::from(y - 1) * TILE_SIZE,
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

    let render_tile_material = |piece_type: &model::TileColor| match piece_type {
        model::TileColor::White => white_tile.clone().into(),
        model::TileColor::Black => black_tile.clone().into(),
    };
    let board = model::create_board();

    commands.spawn(Camera2dBundle::default());

    for row_index in 0..board.tiles.len() as i16 {
        let row = &board.tiles[row_index as usize];
        for tile_index in 0..row.len() as i16 {
            let tile = &row[tile_index as usize];
            let transform = coords_to_transform(tile_index, row_index);
            commands.spawn(SpriteBundle {
                material: materials.add(render_tile_material(&tile.color)),
                transform: transform,
                ..Default::default()
            });
        }
    }

    let middle_index: i16 = (board.tiles[0].len() / 2) as i16;
    for (i, piece) in board.pieces.iter().enumerate() {
        commands.spawn((piece.clone(),));
        if i == 0 {
            commands.with(Selected);
        }
    }
}

fn setup_piece_sprites(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pieces: Query<(Entity, &model::Piece)>,
) {
    let white_piece = asset_server.load("textures/WhitePiece.png");
    let black_piece = asset_server.load("textures/BlackPiece.png");

    let render_piece_material = |piece_type: model::PieceType| match piece_type {
        model::PieceType::WhitePiece => white_piece.clone().into(),
        model::PieceType::BlackPiece => black_piece.clone().into(),
    };
    for (entity, piece) in pieces.iter() {
        commands.insert(
            entity,
            SpriteBundle {
                material: materials.add(render_piece_material(piece.piece_type)),
                ..Default::default()
            },
        );
    }
}

fn update_transforms(mut transforms_query: Query<(&model::Piece, &mut Transform)>) {
    for (piece, mut transform) in transforms_query.iter_mut() {
        *transform = coords_to_transform(piece.x, piece.y);
    }
}

fn keyboard_input(
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    selected_query: Query<(Entity,), With<Selected>>,
    mut pieces_query: Query<(Entity, &mut model::Piece)>,
) {
    let mut x_offset: i16 = 0;
    let mut y_offset: i16 = 0;
    if keyboard_input.just_pressed(KeyCode::Up) {
        y_offset = y_offset + 1;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        y_offset = y_offset - 1;
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        x_offset = x_offset - 1;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        x_offset = x_offset + 1;
    }
    if x_offset == 0 && y_offset == 0 {
        return;
    }
    let selected_entity = selected_query.iter().next().unwrap().0;
    let selected_piece = pieces_query.get_mut(selected_entity).unwrap().1.clone();

    let move_result = model::try_move(
        selected_piece,
        x_offset,
        y_offset,
        pieces_query
            .iter_mut()
    );

    match move_result {
        model::MoveResult::Nothing => {}
        model::MoveResult::Move => {
            let mut piece = pieces_query.get_mut(selected_entity).unwrap().1;
            piece.x += x_offset;
            piece.y += y_offset;
        }
        model::MoveResult::Control { id_to_control } => {
            commands.despawn(selected_entity);
            commands.insert_one(id_to_control, Selected);
        }
    }
}
