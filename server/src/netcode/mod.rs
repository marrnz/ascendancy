use bevy::app::{App, Plugin};
use bevy::prelude::{in_state, IntoSystemConfigs, Update};
use crate::GameState;
use crate::netcode::systems::{handle_server_events, receive_unreliable_client_messages};

mod systems;
mod events;

pub struct NetcodePlugin;

impl Plugin for NetcodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_server_events)
            .add_systems(Update, receive_unreliable_client_messages.run_if(in_state(GameState::WaitingForPlayersReady)));
    }
}
