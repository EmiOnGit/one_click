use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

use crate::{loading::MapAssets, GameState};
pub struct MapPlugin;
/// The size of one tile
pub const TILE_SIZE: Vec2 = Vec2::splat(32.);
/// This plugin loads all assets in the loading game state
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            // Indicates the current level
            // Currently, we always start at level 0
            .insert_resource(LevelSelection::Index(0))
            .add_system(load_ldtk.in_schedule(OnEnter(GameState::Playing)));
    }
}
/// Spawns the map entity
///
/// The ldtk map is already loaded in the LoadingPlugin
pub fn load_ldtk(mut commands: Commands, maps: Res<MapAssets>) {
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle: maps.main_map.clone(),
            ..Default::default()
        })
        .insert(Name::new("Map"));
}
