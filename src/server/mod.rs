pub(crate) use crate::server::map::MapPlugin;
use bevy::app::{App, Plugin};
use crate::server::player::PlayerPlugin;

mod map;
mod player;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPlugin)
            .add_plugins(PlayerPlugin);
    }
}
