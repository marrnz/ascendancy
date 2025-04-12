use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::{Event, PluginGroup};
use bevy::MinimalPlugins;
use std::time::Duration;
use bevy::state::app::StatesPlugin;
use bevy_renet::renet::ClientId;
use ascendancy_shared::ClientGameState;
use crate::map::MapPlugin;
use crate::netcode::NetcodePlugin;
use crate::player::PlayerPlugin;
use crate::state::StatePlugin;

mod map;
mod player;
mod netcode;
mod state;

pub fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 128.0,
            ))),
        )
        .add_plugins(StatesPlugin)
        .add_plugins(LogPlugin::default())
        // custom plugins
        .add_plugins(NetcodePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MapPlugin)
        .add_event::<ClientStateTransitionEvent>()
        .run();
}

#[derive(Event)]
pub struct ClientStateTransitionEvent {
    client_id: ClientId,
    target_state: ClientGameState
}