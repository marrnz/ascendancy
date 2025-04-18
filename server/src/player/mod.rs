use crate::player::events::PlayerInput;
use crate::player::systems::{handle_player_inputs, send_update_player_state, spawn_players};
use crate::states::state::GameState;
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{in_state, Condition, FixedPostUpdate, IntoSystemConfigs, OnEnter};

pub mod events;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GenerateWorld), spawn_players)
            .add_event::<PlayerInput>()
            .add_systems(FixedUpdate, handle_player_inputs.run_if(in_state(GameState::PlayerVsEnvironment).or(in_state(GameState::PlayerVsPlayer))))
            .add_systems(FixedPostUpdate, send_update_player_state.run_if(in_state(GameState::PlayerVsEnvironment).or(in_state(GameState::PlayerVsPlayer))));
    }
}
