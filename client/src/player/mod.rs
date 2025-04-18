use crate::player::events::{OpponentStateConfirmed, PlayerInputJustPressed, PlayerStateConfirmed};
use crate::player::render::RenderPlugin;
use crate::player::systems::{
    insert_player_entity, insert_pvp_players, record_player_input, send_player_input,
    update_opponent_pos_from_server, update_player_pos_from_server,
};
use crate::{PlayerSpawned, StartPlayerVsPlayerEvent};
use ascendancy_shared::ClientGameState;
use bevy::app::{App, Plugin, PreUpdate};
use bevy::prelude::{
    Condition, FixedUpdate, IntoSystemConfigs, Update, in_state, on_event, run_once,
};

pub mod events;
mod render;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin)
            .add_event::<PlayerInputJustPressed>()
            .add_event::<PlayerStateConfirmed>()
            .add_event::<OpponentStateConfirmed>()
            .add_systems(PreUpdate, record_player_input)
            .add_systems(
                Update,
                (
                    insert_player_entity.run_if(on_event::<PlayerSpawned>.and(run_once)),
                    insert_pvp_players.run_if(on_event::<StartPlayerVsPlayerEvent>),
                    update_player_pos_from_server.run_if(
                        on_event::<PlayerStateConfirmed>.and(
                            in_state(ClientGameState::PlayerVsEnvironment)
                                .or(in_state(ClientGameState::PlayerVsPlayer)),
                        ),
                    ),
                    update_opponent_pos_from_server.run_if(
                        on_event::<OpponentStateConfirmed>.and(
                            in_state(ClientGameState::PlayerVsEnvironment)
                                .or(in_state(ClientGameState::PlayerVsPlayer)),
                        ),
                    ),
                ),
            )
            .add_systems(
                FixedUpdate,
                send_player_input.run_if(
                    in_state(ClientGameState::PlayerVsEnvironment)
                        .or(in_state(ClientGameState::PlayerVsPlayer)),
                ),
            );
    }
}
