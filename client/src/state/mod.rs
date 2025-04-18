use crate::state::systems::{
    check_server_connection, send_pve_ready_message, send_pvp_ready_message,
    send_waiting_for_lobby_state_message, set_spawning_state, transition_to_pve_state,
    transition_to_pvp_state,
};
use crate::{MapSpawned, PlayerSpawned, StartPlayerVsEnvEvent, StartPlayerVsPlayerEvent};
use ascendancy_shared::{ClientGameState, Player, Tile};
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{
    Condition, IntoSystemConfigs, OnEnter, Update, any_with_component, in_state, on_event,
    run_once,
};

mod systems;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            check_server_connection.run_if(in_state(ClientGameState::ConnectingToServer)),
        )
        .add_systems(
            Update,
            (
                set_spawning_state.run_if(on_event::<MapSpawned>.and(on_event::<PlayerSpawned>)),
                send_pve_ready_message.run_if(
                    any_with_component::<Tile>
                        .and(any_with_component::<Player>)
                        .and(run_once),
                ),
                transition_to_pve_state.run_if(on_event::<StartPlayerVsEnvEvent>),
                transition_to_pvp_state.run_if(on_event::<StartPlayerVsPlayerEvent>),
            ),
        )
        .add_systems(
            OnEnter(ClientGameState::WaitingForFullLobby),
            send_waiting_for_lobby_state_message,
        )
        .add_systems(
            OnEnter(ClientGameState::PlayerVsPlayer),
            send_pvp_ready_message,
        );
    }
}
