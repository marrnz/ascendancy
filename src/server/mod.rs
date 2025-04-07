pub(crate) use crate::server::map::MapPlugin;
use crate::server::player::PlayerPlugin;
use crate::shared::{PlayerInputAttempt, PlayerInputType};
use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Event};

mod map;
mod player;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapPlugin).add_plugins(PlayerPlugin);
    }
}

#[derive(Event)]
pub struct PlayerInputConfirmed {
    entity: Entity,
    input_type: PlayerInputType,
}

impl From<&PlayerInputAttempt> for PlayerInputConfirmed {
    fn from(player_input_attempt: &PlayerInputAttempt) -> PlayerInputConfirmed {
        PlayerInputConfirmed {
            entity: player_input_attempt.entity,
            input_type: player_input_attempt.input_type.clone(),
        }
    }
}
