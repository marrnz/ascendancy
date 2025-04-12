use bevy::prelude::{Component, Dir2, KeyCode, Resource, States, Vec2};
use bevy_renet::renet::ClientId;
use bincode::config::Configuration;
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};

pub const PROTOCOL_ID: u64 = 1000;
pub const TILESIZE: f32 = 32.;

#[derive(Clone, Copy, Encode, Decode, Debug)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Clone, Copy)]
pub enum PlayerInputType {
    MoveAttempt(Dir2),
}

#[derive(Encode, Decode, Debug)]
pub enum ServerNetworkMessage {
    WaitingForPlayers {
        player_position: Position,
        map: Map,
    },
    StartPlayerVsEnvironment
}

#[derive(Encode, Decode, Debug)]
pub enum ClientNetworkMessage {
    StateTransition { target_state: ClientGameState },
    PlayerInput { key_code: EncodedKeyCode },
}

#[derive(Encode, Decode, Debug)]
pub enum EncodedKeyCode {
    KeyW,
    KeyS,
    KeyA,
    KeyD,
    Unmapped,
}

impl From<KeyCode> for EncodedKeyCode {
    fn from(key_code: KeyCode) -> Self {
        match key_code {
            KeyCode::KeyW => Self::KeyW,
            KeyCode::KeyS => Self::KeyS,
            KeyCode::KeyA => Self::KeyA,
            KeyCode::KeyD => Self::KeyD,
            _ => Self::Unmapped,
        }
    }
}

#[derive(Default, States, Eq, Clone, Debug, Hash, PartialEq, Encode, Decode)]
pub enum ClientGameState {
    #[default]
    AssetLoading,
    ConnectingToServer,
    WaitingForFullLobby,
    Spawning,
    PlayerVsEnvironment,
    PlayerVsPlayer,
    GameOver,
}

#[derive(Resource, Debug, Encode, Decode, Clone)]
pub struct Map {
    pub width: usize,
    pub tiles: Vec<Tile>,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Encode, Decode, Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: Position,
}

#[derive(Component, Debug, Clone)]
pub struct Position(pub Vec2);

impl Encode for Position {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        bincode::Encode::encode(&self.0.x, encoder)?;
        bincode::Encode::encode(&self.0.y, encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for Position {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Self(Vec2::new(
            bincode::Decode::decode(decoder)?,
            bincode::Decode::decode(decoder)?,
        )))
    }
}

impl<'de, Context> bincode::BorrowDecode<'de, Context> for Position {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        Ok(Self(Vec2::new(
            bincode::BorrowDecode::borrow_decode(decoder)?,
            bincode::BorrowDecode::borrow_decode(decoder)?,
        )))
    }
}

#[derive(Component)]
pub struct PreviousPosition(pub Vec2);

pub fn bincode_config() -> Configuration {
    bincode::config::standard()
        .with_little_endian()
        .with_variable_int_encoding()
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
                map.tiles.push(Tile {
                    tile_type,
                    position: Position(Vec2::new(x as f32, y as f32)),
                });
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
