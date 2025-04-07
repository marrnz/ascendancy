use crate::GameState;
use crate::server::player::systems::{
    handle_player_input_events, spawn_player, validate_player_inputs,
};
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{IntoSystemConfigs, Startup, in_state};

mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            FixedUpdate,
            (validate_player_inputs, handle_player_input_events)
                .chain()
                .run_if(in_state(GameState::Main)),
        );
    }
}
