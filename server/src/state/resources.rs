use bevy::prelude::Resource;
use bevy::utils::HashMap;
use bevy_renet::renet::ClientId;
use ascendancy_shared::ClientGameState;

#[derive(Resource, Default)]
pub struct Lobby {
    pub players: HashMap<ClientId, LobbyState>
}

#[derive(Default)]
pub struct LobbyState {
    pub client_game_state: ClientGameState
}

impl Lobby {
    pub fn get_or_insert(&mut self, key: ClientId) -> &mut LobbyState {
        if !self.players.contains_key(&key) {
            self.players.insert(key, LobbyState::default());
        }
        self.players.get_mut(&key).unwrap()
    }
}