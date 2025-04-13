use crate::player::render::RenderPlugin;
use crate::player::systems::{insert_player_entity, insert_pvp_players};
use crate::{PlayerSpawned, StartPlayerVsPlayerEvent};
use bevy::app::{App, Plugin};
use bevy::prelude::{Condition, IntoSystemConfigs, Update, on_event, run_once};

mod render;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin)
            .add_systems(
                Update,
                insert_player_entity.run_if(on_event::<PlayerSpawned>.and(run_once)),
            )
            .add_systems(
                Update,
                insert_pvp_players.run_if(on_event::<StartPlayerVsPlayerEvent>),
            );
    }
}
