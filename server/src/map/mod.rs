use bevy::app::{App, Plugin, Startup};
use bevy::prelude::OnEnter;
use ascendancy_shared::Map;
use crate::GameState::GenerateWorld;
use crate::map::systems::spawn_map;

mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new(500, 500))
            .add_systems(OnEnter(GenerateWorld), spawn_map);
    }
}
