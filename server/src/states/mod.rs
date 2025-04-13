use crate::states::resources::Lobby;
use crate::states::state::GameState;
use crate::states::systems::{
    check_for_generate_world_transition, check_for_player_vs_env_transition,
    check_for_player_vs_player_transition, send_player_vs_env_message,
    send_player_vs_player_message, send_waiting_for_players_message,
    transition_to_waiting_for_players_ready, update_lobby_state,
};
use ascendancy_shared::{Map, Player};
use bevy::app::App;
use bevy::prelude::{
    AppExtStates, Condition, IntoSystemConfigs, OnEnter, Plugin, Update, any_with_component,
    in_state, resource_changed, resource_exists, run_once,
};
use bevy::time::common_conditions::once_after_real_delay;
use std::time::Duration;

pub mod resources;
pub mod state;
mod systems;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(Lobby::default())
            .add_systems(
                Update,
                (
                    update_lobby_state,
                    check_for_generate_world_transition
                        .run_if(in_state(GameState::WaitingForFullLobby)),
                    transition_to_waiting_for_players_ready.run_if(
                        any_with_component::<Player>
                            .and(resource_exists::<Map>)
                            .and(run_once),
                    ),
                    check_for_player_vs_env_transition.run_if(
                        in_state(GameState::WaitingForPlayersReady).and(resource_changed::<Lobby>),
                    ),
                    check_for_player_vs_player_transition.run_if(
                        in_state(GameState::PlayerVsEnvironment)
                            .and(once_after_real_delay(Duration::from_secs(2))),
                    ),
                ),
            )
            .add_systems(
                OnEnter(GameState::WaitingForPlayersReady),
                send_waiting_for_players_message,
            )
            .add_systems(
                OnEnter(GameState::PlayerVsEnvironment),
                send_player_vs_env_message,
            )
            .add_systems(
                OnEnter(GameState::PlayerVsPlayer),
                send_player_vs_player_message,
            );
    }
}
