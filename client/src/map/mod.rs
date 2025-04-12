use crate::MapSpawned;
use crate::map::render::RenderPlugin;
use crate::map::systems::insert_map_tile_entities;
use bevy::app::{App, Plugin};
use bevy::prelude::{Condition, IntoSystemConfigs, Update, on_event, run_once, in_state};
use ascendancy_shared::ClientGameState;

mod render;
mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin).add_systems(
            Update,
            insert_map_tile_entities.run_if(in_state(ClientGameState::Spawning).and(run_once)),
        );
    }
}
