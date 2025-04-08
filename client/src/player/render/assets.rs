use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 16, rows = 16))]
    pub atlas: Handle<TextureAtlasLayout>,
    #[asset(path = "ascii_spritesheet.png")]
    pub spritesheet: Handle<Image>,
}
