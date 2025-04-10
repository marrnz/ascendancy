use bevy::prelude::{Component, Dir2, Entity, Event, Resource, States, Vec2};
use bincode::config::Configuration;

pub const PROTOCOL_ID: u64 = 1000;
pub const TILESIZE: f32 = 32.;

pub enum TileType {
    Floor,
    Wall,
}

#[derive(Clone, Copy)]
pub enum PlayerInputType {
    MoveAttempt(Dir2),
}

pub enum NetworkMessageType {
    StateTransition { target_state: ClientGameState },
}

#[derive(Default, States, Eq, Clone, Debug, Hash, PartialEq)]
pub enum ClientGameState {
    #[default]
    AssetLoading,
    ConnectingToServer,
    WaitingForFullLobby,
    Spawning,
    PvePhase,
}

#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub tiles: Vec<TileType>,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct PreviousPosition(pub Vec2);

#[derive(Event)]
pub struct PlayerInputAttempt {
    pub entity: Entity,
    pub input_type: PlayerInputType,
}

#[derive(Event)]
pub struct PlayerInputConfirmed {
    pub entity: Entity,
    pub input_type: PlayerInputType,
}

pub fn bincode_config() -> Configuration {
    bincode::config::standard()
        .with_little_endian()
        .with_variable_int_encoding()
}

impl From<&PlayerInputAttempt> for PlayerInputConfirmed {
    fn from(player_input_attempt: &PlayerInputAttempt) -> PlayerInputConfirmed {
        PlayerInputConfirmed {
            entity: player_input_attempt.entity,
            input_type: player_input_attempt.input_type.clone(),
        }
    }
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Self {
            width,
            tiles: Vec::with_capacity(width * height),
        };
        for x in 0..width {
            for y in 0..height {
                let mut tile_type = TileType::Floor;
                if x == 0 || x == width || y == 0 || y == height {
                    tile_type = TileType::Wall;
                }
                map.tiles.push(tile_type);
            }
        }
        map
    }
    pub fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn index_from_float(&self, x: f32, y: f32) -> usize {
        let x = x.floor() as usize;
        let y = y.floor() as usize;
        self.index(x, y)
    }

    pub fn coordinates(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }
}
