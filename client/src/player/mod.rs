use crate::GameState;
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{IntoSystemConfigs, in_state};
use crate::player::render::RenderPlugin;
use crate::player::systems::player_input;

mod render;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin)
            .add_systems(FixedUpdate, player_input.run_if(in_state(GameState::Main)));
    }
}
