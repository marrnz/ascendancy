use crate::map::MapPlugin;
use crate::netcode::NetcodePlugin;
use crate::player::PlayerPlugin;
use crate::states::StatePlugin;
use ascendancy_shared::ClientGameState;
use bevy::MinimalPlugins;
use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::{Event, Fixed, PluginGroup, Time};
use bevy::state::app::StatesPlugin;
use bevy_renet::renet::ClientId;
use std::time::Duration;

mod map;
mod netcode;
mod player;
mod states;

pub fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 128.0,
            ))),
        )
        .add_plugins(StatesPlugin)
        .add_plugins(LogPlugin::default())
        // TODO: Tweak if players feel lag
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
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
    target_state: ClientGameState,
}
