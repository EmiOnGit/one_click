use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::map::TILE_SIZE;
use bevy::prelude::*;

pub struct PlayerPlugin;

/// Marker component for the player
#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                // The player should have the size of one tile
                custom_size: Some(TILE_SIZE),
                ..default()
            },
            texture: textures.texture_bevy.clone(),
            // TODO we should save the player start position with the level
            transform: Transform::from_translation(Vec3::new(7. * TILE_SIZE.x - TILE_SIZE.x / 2., TILE_SIZE.y / 2., 2.)),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
