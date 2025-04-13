use crate::player::systems::{spawn_players};
use bevy::app::{App, Plugin};
use bevy::prelude::{OnEnter};
use crate::states::state::GameState;

mod systems;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GenerateWorld), spawn_players);

    }
}
