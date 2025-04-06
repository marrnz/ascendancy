use bevy::app::{App, Plugin};
use crate::client::player::render::RenderPlugin;

mod render;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin);
    }
}
