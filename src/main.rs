use bevy::prelude::*;

mod model;

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 500.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_ui.system())
        .add_startup_system(setup_board.system())
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_piece_sprites.system())
        .add_system(keyboard_input.system())
        .add_system(update_transforms.system())
        .run();
}

struct InputText;
struct Selected;

const TILE_SIZE: f32 = 100.0;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
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
            text: Text::with_section(
                "This is where the text appears".to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
                TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(InputText);
}

fn coords_to_transform(x: i16, y: i16, z: f32) -> Transform {
    return Transform {
        translation: Vec3::new(
            f32::from(x - 1) * TILE_SIZE,
            f32::from(y - 1) * TILE_SIZE,
            0.0,
        ),
        ..Default::default()
    };
}

fn setup_board(
    mut commands: Commands,
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

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for row_index in 0..board.tiles.len() as i16 {
        let row = &board.tiles[row_index as usize];
        for tile_index in 0..row.len() as i16 {
            let tile = &row[tile_index as usize];
            let transform = coords_to_transform(tile_index, row_index, 0.0);
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(render_tile_material(&tile.color)),
                transform: transform,
                ..Default::default()
            });
        }
    }

    for (i, piece) in board.pieces.iter().enumerate() {
        let mut entity_commands = commands.spawn_bundle((piece.clone(),));
        if i == 0 {
            entity_commands.insert(Selected);
        }
    }
}

fn setup_piece_sprites(
    mut commands: Commands,
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
        commands.entity(entity).insert_bundle(SpriteBundle {
            material: materials.add(render_piece_material(piece.piece_type)),
            ..Default::default()
        });
    }
}

fn update_transforms(mut transforms_query: Query<(&model::Piece, &mut Transform)>) {
    for (piece, mut transform) in transforms_query.iter_mut() {
        *transform = coords_to_transform(piece.x, piece.y, 1.0);
    }
}

fn keyboard_input(
    mut commands: Commands,
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

    let move_result = model::try_move(selected_piece, x_offset, y_offset, pieces_query.iter_mut());

    match move_result {
        model::MoveResult::Nothing => {}
        model::MoveResult::Move => {
            let mut piece = pieces_query.get_mut(selected_entity).unwrap().1;
            piece.x += x_offset;
            piece.y += y_offset;
        }
        model::MoveResult::Control { id_to_control } => {
            commands.entity(selected_entity).despawn();
            commands.entity(id_to_control).insert(Selected);
        }
    }
}
