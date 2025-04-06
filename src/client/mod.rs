use crate::client::map::MapPlugin;
use bevy::app::{App, Plugin};

mod map;
mod player;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPlugin);
    }
}
