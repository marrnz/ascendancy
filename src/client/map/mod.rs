use crate::client::map::render::RenderPlugin;
use bevy::app::{App, Plugin};

mod render;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin);
    }
}
