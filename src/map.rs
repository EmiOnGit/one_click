use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::LdtkEntityAppExt, LdtkEntity, LdtkPlugin, LdtkWorldBundle, LevelSelection,
};

use crate::{loading::MapAssets, GameState};
pub struct MapPlugin;

/// This plugin loads all assets in the loading game state
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .add_system(load_ldtk.in_schedule(OnEnter(GameState::Playing)))
            .register_ldtk_entity::<LevelMap>("MapEntity");
    }
}

pub fn load_ldtk(mut commands: Commands, maps: Res<MapAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: maps.main_map.clone(),
        ..Default::default()
    });
}

#[derive(Bundle, LdtkEntity)]
pub struct LevelMap {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
