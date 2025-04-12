use crate::player::systems::{spawn_player};
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{OnEnter, in_state};
use crate::state::state::GameState;

mod systems;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GenerateWorld), spawn_player)
            /*
            .add_systems(
                FixedUpdate,
                (validate_player_inputs, handle_player_input_events)
                    .chain()
                    .run_if(
                        in_state(GameState::PlayerVsEnvironment)
                            .or(in_state(GameState::PlayerVsPlayer)),
                    ),
            )*/
            ;

    }
}
