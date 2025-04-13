use bevy::prelude::Resource;
use bevy::utils::HashMap;
use bevy_renet::renet::ClientId;
use ascendancy_shared::ClientGameState;

const FULL_LOBBY_COUNT: usize = 2;

#[derive(Resource, Default, Debug)]
pub struct Lobby {
    pub players: HashMap<ClientId, LobbyState>,
    pub full: bool
}

#[derive(Default, Debug)]
pub struct LobbyState {
    pub client_game_state: ClientGameState
}

impl Lobby {
    pub fn get_or_insert(&mut self, key: ClientId) -> &mut LobbyState {
        if !self.players.contains_key(&key) {
            self.players.insert(key, LobbyState::default());
        }
        if self.players.len() == FULL_LOBBY_COUNT {
            self.full = true;
        }
        self.players.get_mut(&key).unwrap()
    }
}