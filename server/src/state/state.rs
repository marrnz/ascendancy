use bevy::prelude::States;

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