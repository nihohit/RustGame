pub enum TileColor {
    Black,
    White
}

pub enum Advice {
    GetToCornerToWin,
    DoSomething,
    DoSomethingElse
}

pub struct Tile {
    color: TileColor,
    advice: Option<Advice>,
    passable: bool
}

pub enum EntityType {
    Player,
    Crown,
    Chair
}

pub struct Entity {
    entity_type: EntityType,
    x: u32,
    y: u32
}

pub struct Game {
    tiles: [[Tile; 3]; 3],
    entities: Vec<Entity>
}