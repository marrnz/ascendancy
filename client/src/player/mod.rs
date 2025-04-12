use crate::player::render::RenderPlugin;
use crate::player::systems::insert_player_entity;
use crate::PlayerSpawned;
use bevy::app::{App, Plugin};
use bevy::prelude::{on_event, run_once, Condition, IntoSystemConfigs, Update};

mod render;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin)
            /*
            .add_systems(
            FixedUpdate,
            player_input.run_if(
                in_state(ClientGameState::PlayerVsEnvironment)
                    .or(in_state(ClientGameState::PlayerVsPlayer)),
            ),
        )*/
            .add_systems(Update, insert_player_entity.run_if(on_event::<PlayerSpawned>.and(run_once)))
        ;

    }
}
