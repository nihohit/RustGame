#[derive(Clone, Copy)]
pub enum TileColor {
    Black,
    White,
}

#[derive(Clone, Copy)]
pub enum Advice {
    GetToCornerToWin,
    DoSomething,
    DoSomethingElse,
}

#[derive(PartialEq, Clone, Copy)]
pub enum EntityType {
    BlackPiece,
    WhitePiece,
}

#[derive(Clone, Copy)]
pub struct Player {
    pub entity_type: EntityType,
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub color: TileColor,
    pub entity: Option<EntityType>,
    advice: Option<Advice>,
    passable: bool,
}

pub struct Game {
    pub tiles: [[Tile; 3]; 3],
    pub player: Player,
}

pub fn create_board() -> Game {
    return Game {
        tiles: [
            [
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                    entity: None,
                },
                Tile {
                    color: TileColor::Black,
                    advice: None,
                    passable: true,
                    entity: None,
                },
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                    entity: None,
                },
            ],
            [
                Tile {
                    color: TileColor::Black,
                    advice: None,
                    passable: true,
                    entity: None,
                },
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                    entity: None,
                },
                Tile {
                    color: TileColor::Black,
                    advice: None,
                    passable: true,
                    entity: Some(EntityType::WhitePiece),
                },
            ],
            [
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                    entity: Some(EntityType::BlackPiece),
                },
                Tile {
                    color: TileColor::Black,
                    advice: None,
                    passable: true,
                    entity: None,
                },
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                    entity: None,
                },
            ],
        ],
        player: Player {
            entity_type: EntityType::WhitePiece,
            x: 1,
            y: 1,
        },
    };
}

fn can_move(player: EntityType, is_diagonal_movement: bool) -> bool {
    match player {
        EntityType::BlackPiece => return is_diagonal_movement,
        EntityType::WhitePiece => return !is_diagonal_movement,
    }
}

fn can_eat(player: EntityType, is_diagonal_movement: bool) -> bool {
    match player {
        EntityType::BlackPiece => return !is_diagonal_movement,
        EntityType::WhitePiece => return is_diagonal_movement,
    }
}

fn try_move(board: Game, x_offset: i16, y_offset: i16) -> Option<Game> {
    let is_diagonal = x_offset != 0 && y_offset != 0;
    let player = board.player;
    let x_target_signed = player.x + x_offset;
    let y_target_signed = player.y + y_offset;
    if x_target_signed < 0 || y_target_signed < 0 {
        return None;
    }
    let x_target = x_target_signed as usize;
    let y_target = y_target_signed as usize;
    if x_target > board.tiles.len() || y_target >= board.tiles[x_target].len() {
        return None;
    }

    let entity_in_point = board.tiles[x_target][y_target].entity;
    let empty_and_can_move = entity_in_point.is_none() && can_move(player.entity_type, is_diagonal);
    let contains_other_color_and_can_eat = !entity_in_point.is_none()
        && entity_in_point.unwrap() != player.entity_type
        && can_eat(player.entity_type, is_diagonal);
    if !empty_and_can_move && !contains_other_color_and_can_eat {
        return None;
    }

    let mut new_game = Game {
        tiles: board.tiles,
        player: Player {
            entity_type: player.entity_type,
            x: x_target_signed,
            y: y_target_signed,
        },
    };
    new_game.tiles[x_target][y_target].entity = None;
    return Some(new_game);
}
