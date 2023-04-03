use crate::loading::TextureAssets;
use crate::GameState;
pub use bevy::prelude::*;

pub const PLATFORM_Z: f32 = 0.;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_map.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Collider {
    pub(crate) size: Vec2,
}

fn setup_map(mut commands: Commands, textures: Res<TextureAssets>) {
    let center = Vec2::new(50., 20.);
    let size = Vec2::new(60., 25.);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(center.x, center.y, PLATFORM_Z)),
            ..default()
        })
        .insert(Collider { size });

    let center = Vec2::new(0., -100.);
    let size = Vec2::new(400., 25.);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(center.x, center.y, PLATFORM_Z)),
            ..default()
        })
        .insert(Collider { size });
}
