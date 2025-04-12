use ascendancy_shared::ClientGameState;
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{OnEnter, in_state, IntoSystemConfigs};
use crate::state::systems::{check_server_connection, send_waiting_for_lobby_state_message};

mod systems;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            check_server_connection
                .run_if(in_state(ClientGameState::ConnectingToServer)),
        )
        .add_systems(
            OnEnter(ClientGameState::WaitingForFullLobby),
            send_waiting_for_lobby_state_message,
        );
    }
}
