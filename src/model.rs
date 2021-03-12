use std::ops::{Deref};

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
pub enum PieceType {
    BlackPiece,
    WhitePiece,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub color: TileColor,
    advice: Option<Advice>,
    passable: bool,
}

pub struct Game {
    pub tiles: [[Tile; 3]; 3],
    pub pieces: Vec<Piece>,
}

impl Game {
    const MAX_X: i16 = 2;
    const MAX_Y: i16 = 2;
}

pub enum MoveResult<PieceID> {
    Control { id_to_control: PieceID },
    Move,
    Nothing,
}

pub fn create_board() -> Game {
    return Game {
        tiles: [
            [
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                },
                Tile {
                    color: TileColor::Black,
                    advice: None,
                    passable: true,
                },
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                },
            ],
            [
                Tile {
                    color: TileColor::Black,
                    advice: None,
                    passable: true,
                },
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                },
                Tile {
                    color: TileColor::Black,
                    advice: None,
                    passable: true,
                },
            ],
            [
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                },
                Tile {
                    color: TileColor::Black,
                    advice: None,
                    passable: true,
                },
                Tile {
                    color: TileColor::White,
                    advice: None,
                    passable: true,
                },
            ],
        ],
        pieces: vec![
            Piece {
                piece_type: PieceType::WhitePiece,
                x: 1,
                y: 1,
            },
            Piece {
                piece_type: PieceType::WhitePiece,
                x: 2,
                y: 2,
            },
            Piece {
                piece_type: PieceType::BlackPiece,
                x: 2,
                y: 1,
            },
        ],
    };
}

pub fn is_diagonal_movement(x_offset: i16, y_offset: i16) -> bool {
    return x_offset != 0 && y_offset != 0;
}

pub fn can_move(player: PieceType, is_diagonal_movement: bool) -> bool {
    match player {
        PieceType::BlackPiece => return is_diagonal_movement,
        PieceType::WhitePiece => return !is_diagonal_movement,
    }
}

pub fn can_eat(player: PieceType, is_diagonal_movement: bool) -> bool {
    match player {
        PieceType::BlackPiece => return !is_diagonal_movement,
        PieceType::WhitePiece => return is_diagonal_movement,
    }
}

pub fn try_move<EntityID, I, P>(
    selected: Piece,
    x_offset: i16,
    y_offset: i16,
    pieces: I,
) -> MoveResult<EntityID>
where
    I: Iterator<Item = (EntityID, P)>,
    P: Deref<Target = Piece>
{
    let next_x = selected.x + x_offset;
    let next_y = selected.y + y_offset;

    let is_diagonal = is_diagonal_movement(x_offset, y_offset);
    if next_x > Game::MAX_X || next_x < 0 || next_y > Game::MAX_Y || next_y < 0 {
        return MoveResult::Nothing;
    }

    let item_in_point = pieces
        .filter(|(_, piece)| piece.x == next_x && piece.y == next_y)
        .next();

    match item_in_point {
        None => {
            return if can_move(selected.piece_type, is_diagonal) {
                MoveResult::Move
            } else {
                MoveResult::Nothing
            }
        }
        Some((entity, piece)) => {
            return if selected.piece_type != piece.piece_type
                && can_eat(selected.piece_type, is_diagonal)
            {
                MoveResult::Control {
                    id_to_control: entity,
                }
            } else {
                MoveResult::Nothing
            }
        }
    }
}
