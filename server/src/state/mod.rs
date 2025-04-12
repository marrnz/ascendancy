use crate::state::resources::Lobby;
use crate::state::state::GameState;
use crate::state::systems::{
    check_for_waiting_for_players_transition, send_waiting_for_players_message, update_lobby_state,
};
use ascendancy_shared::{Map, Player};
use bevy::app::App;
use bevy::prelude::{
    AppExtStates, Condition, IntoSystemConfigs, OnEnter, Plugin, Update, any_with_component,
    in_state, resource_exists,
};

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
                    check_for_waiting_for_players_transition
                        .run_if(
                            in_state(GameState::GenerateWorld)
                                .and(any_with_component::<Player>)
                                .and(resource_exists::<Map>),
                        )
                        .chain(),
                ),
            )
            .add_systems(
                OnEnter(GameState::WaitingForPlayersReady),
                send_waiting_for_players_message,
            );
    }
}
