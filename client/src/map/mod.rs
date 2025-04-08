use bevy::app::{App, Plugin};
use crate::map::render::RenderPlugin;

mod render;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin);
    }
}
