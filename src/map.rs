use crate::loading::TextureAssets;
use crate::physics::PhysicsSystems;
use crate::{GameState, HEIGHT, WIDTH};
pub use bevy::prelude::*;

pub const PLATFORM_Z: f32 = 0.;
pub const PLATFORM_HEIGHT: f32 = 25.;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentChunk>()
            .add_system(setup_map.in_schedule(OnEnter(GameState::Playing)))
            .add_system(
                spawn_chunk_system
                    .run_if(in_state(GameState::Playing))
                    .after(PhysicsSystems::Move),
            );
    }
}

#[derive(Component)]
pub struct Collider {
    pub(crate) size: Vec2,
}

fn setup_map(mut commands: Commands, textures: Res<TextureAssets>) {
    let center = Vec2::new(50., 20.);
    let size = Vec2::new(60., PLATFORM_HEIGHT);
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

    let center_start_wall = Vec2::new(-WIDTH / 2. + PLATFORM_HEIGHT / 2., PLATFORM_HEIGHT / 2.);
    let size_start_wall = Vec2::new(PLATFORM_HEIGHT, HEIGHT - PLATFORM_HEIGHT);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size_start_wall),
                ..default()
            },
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(
                center_start_wall.x,
                center_start_wall.y,
                PLATFORM_Z,
            )),
            ..default()
        })
        .insert(Collider {
            size: size_start_wall,
        });
    spawn_chunk(&mut commands, &textures, 0);
    spawn_chunk(&mut commands, &textures, 1);
}

#[derive(Default, Resource)]
pub struct CurrentChunk(pub(crate) usize);

fn spawn_chunk(commands: &mut Commands, textures: &TextureAssets, index: usize) {
    info!("Spawning chunk {index}");
    let center = Vec2::new(index as f32 * WIDTH, -HEIGHT / 2. + PLATFORM_HEIGHT / 2.);
    let size = Vec2::new(WIDTH, PLATFORM_HEIGHT);
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

fn spawn_chunk_system(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    current_chunk: Res<CurrentChunk>,
) {
    if !current_chunk.is_changed() {
        return;
    }

    spawn_chunk(&mut commands, &textures, current_chunk.0 + 2);
}
