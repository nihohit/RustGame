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
    pub color: TileColor,
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
    pub tiles: [[Tile; 3]; 3],
    entities: Vec<Entity>
}

pub fn create_board() -> Game {
    return Game{
        tiles:[
            [
                Tile{color:TileColor::White, advice:None, passable:true},
                Tile{color:TileColor::Black, advice:None, passable:true},
                Tile{color:TileColor::White, advice:None, passable:true}
            ],
            [
                Tile{color:TileColor::Black, advice:None, passable:true},
                Tile{color:TileColor::White, advice:None, passable:true},
                Tile{color:TileColor::Black, advice:None, passable:true}
            ],
            [
                Tile{color:TileColor::White, advice:None, passable:true},
                Tile{color:TileColor::Black, advice:None, passable:true},
                Tile{color:TileColor::White, advice:None, passable:true}
            ]
        ], 
        entities:vec![]
    };
}