use bevy::app::{App, Plugin};
use ascendancy_shared::Map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new(500, 500));
    }
}
