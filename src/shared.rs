use bevy::prelude::{Component, Resource, Vec2};

pub const TILESIZE: f32 = 32.;

pub enum TileType {
    Floor,
    Wall,
}

#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub tiles: Vec<TileType>,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Position(pub Vec2);

impl Map {
    pub fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn index_from_float(&self, x: f32, y: f32) -> usize {
        let x = x.floor() as usize;
        let y = y.floor() as usize;
        self.index(x, y)
    }

    pub fn coordinates(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }
}
