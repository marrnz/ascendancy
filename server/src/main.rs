use ascendancy_shared::{PlayerInputAttempt, PlayerInputConfirmed};
use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::{AppExtStates, PluginGroup, States};
use bevy::MinimalPlugins;
use std::time::Duration;
use bevy::state::app::StatesPlugin;
use crate::netcode::NetcodePlugin;

mod map;
mod player;
mod netcode;

pub fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 128.0,
            ))),
        )
        .add_plugins(StatesPlugin)
        .add_plugins(LogPlugin::default())
        .add_plugins(NetcodePlugin)
        .init_state::<GameState>()
        .add_event::<PlayerInputAttempt>()
        .add_event::<PlayerInputConfirmed>()
        .run();
}

#[derive(Default, States, Eq, Clone, Debug, Hash, PartialEq)]
pub enum GameState {
    #[default]
    WaitingForFullLobby,
    GenerateWorld,
    WaitingForPlayersReady,
    PlayerVsEnvironment,
    PlayerVsPlayer,
    GameOver
}