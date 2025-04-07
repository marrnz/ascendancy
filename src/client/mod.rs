use crate::client::camera::CameraPlugin;
use crate::client::map::MapPlugin;
use crate::client::player::PlayerPlugin;
use bevy::app::{App, Plugin};

mod camera;
mod map;
mod player;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(CameraPlugin);
    }
}
