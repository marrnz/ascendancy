use bevy::app::{App, Plugin};
use bevy::prelude::Update;
use crate::netcode::systems::handle_server_events;

mod systems;

pub struct NetcodePlugin;

impl Plugin for NetcodePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_server_events);
    }
}
