use crate::loading::TextureAssets;
use crate::physics::PhysicsSystems;
use crate::{GameState, HEIGHT, WIDTH};
pub use bevy::prelude::*;
use rand::Rng;

pub const PLATFORM_Z: f32 = 0.;
pub const PLATFORM_HEIGHT: f32 = TILE_SIZE;
pub const CHUNK_TILES: usize = 16;
pub const TILE_SIZE: f32 = 32.;
pub const CHUNK_WIDTH: f32 = CHUNK_TILES as f32 * TILE_SIZE;
pub const TUTORIAL_CHUNKS: usize = 5;

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
    let center = Vec2::new(50., -150.);
    let size = Vec2::new(64., PLATFORM_HEIGHT);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            texture: textures.platform.clone(),
            transform: Transform::from_translation(Vec3::new(center.x, center.y, PLATFORM_Z)),
            ..default()
        })
        .insert(Collider { size });

    for brick in 1..20 {
        let center_start_wall = Vec2::new(
            -WIDTH / 2. + TILE_SIZE / 2.,
            -HEIGHT / 2. + TILE_SIZE / 2. + brick as f32 * TILE_SIZE,
        );
        let size_start_wall = Vec2::new(TILE_SIZE, TILE_SIZE);
        commands
            .spawn(SpriteBundle {
                texture: textures.wall.clone(),
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
    }
    spawn_tutorial_chunks(&mut commands, &textures);
}

fn spawn_tutorial_chunks(commands: &mut Commands, textures: &TextureAssets) {
    for index in 0..TUTORIAL_CHUNKS {
        for tile in 0..CHUNK_TILES {
            let center = Vec2::new(
                index as f32 * CHUNK_WIDTH - WIDTH / 2. + TILE_SIZE / 2. + tile as f32 * TILE_SIZE,
                -HEIGHT / 2. + TILE_SIZE / 2.,
            );
            let size = Vec2::new(TILE_SIZE, TILE_SIZE);
            commands
                .spawn(SpriteBundle {
                    texture: textures.ground.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        center.x, center.y, PLATFORM_Z,
                    )),
                    ..default()
                })
                .insert(Collider { size });
        }
    }
}

#[derive(Default, Resource)]
pub struct CurrentChunk(pub(crate) usize);

fn spawn_chunk(commands: &mut Commands, textures: &TextureAssets, index: usize) {
    if index < TUTORIAL_CHUNKS {
        return;
    }
    info!("Spawning chunk {index}");
    let hole1 = rand::thread_rng().gen_range(0..CHUNK_TILES);
    let hole2 = rand::thread_rng().gen_range(0..CHUNK_TILES);
    for tile in 0..CHUNK_TILES {
        if tile == hole1 || tile == hole2 {
            continue;
        }
        let center = Vec2::new(
            index as f32 * CHUNK_WIDTH - WIDTH / 2. + TILE_SIZE / 2. + tile as f32 * TILE_SIZE,
            -HEIGHT / 2. + PLATFORM_HEIGHT / 2.,
        );
        let size = Vec2::new(TILE_SIZE, PLATFORM_HEIGHT);
        commands
            .spawn(SpriteBundle {
                texture: textures.ground.clone(),
                transform: Transform::from_translation(Vec3::new(center.x, center.y, PLATFORM_Z)),
                ..default()
            })
            .insert(Collider { size });
    }
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
