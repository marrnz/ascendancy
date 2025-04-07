use crate::GameState;
use crate::client::player::render::RenderPlugin;
use crate::client::player::systems::player_input;
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{IntoSystemConfigs, in_state};

mod render;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenderPlugin)
            .add_systems(FixedUpdate, player_input.run_if(in_state(GameState::Main)));
    }
}
