use crate::client::ClientPlugin;
use crate::server::ServerPlugin;
use bevy::prelude::*;

mod client;
mod server;
pub mod shared;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ServerPlugin)
        .add_plugins(ClientPlugin)
        .add_systems(Startup, spawn_camera)
        .init_state::<GameState>()
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Default, States, Eq, Clone, Debug, Hash, PartialEq)]
pub enum GameState {
    #[default]
    AssetLoading,
    Main,
}
