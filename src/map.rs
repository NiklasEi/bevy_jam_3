use crate::food::spawn_random_food;
use crate::loading::TextureAssets;
use crate::physics::PhysicsSystems;
use crate::GameState;
pub use bevy::prelude::*;
use rand::Rng;

pub const PLATFORM_Z: f32 = 0.;
pub const PLATFORM_HEIGHT: f32 = TILE_SIZE;
pub const CHUNK_TILES: usize = 16;
pub const TILE_SIZE: f32 = 32.;
pub const CHUNK_WIDTH: f32 = CHUNK_TILES as f32 * TILE_SIZE;
pub const TUTORIAL_CHUNKS: usize = 5;
pub const MAP_GEN_DOUBLE_HOLES_FROM_CHUNK: usize = 12;
pub const MAP_GEN_FOOD_ON_GROUND: f32 = 0.05;
pub const MAP_GEN_FOOD_ON_PLATFORM: f32 = 0.10;

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

#[derive(Component)]
pub struct Solid;

fn setup_map(mut commands: Commands, textures: Res<TextureAssets>) {
    for brick in -1..=1 {
        let center = Vec2::new((8 + brick) as f32 * TILE_SIZE, 4. * TILE_SIZE);
        let size = Vec2::new(TILE_SIZE, TILE_SIZE);
        spawn_tile(&mut commands, size, center, textures.platform.clone());
    }

    for brick in 1..20 {
        let center_start_wall =
            Vec2::new(TILE_SIZE / 2., TILE_SIZE / 2. + brick as f32 * TILE_SIZE);
        let size_start_wall = Vec2::new(TILE_SIZE, TILE_SIZE);

        spawn_tile(
            &mut commands,
            size_start_wall,
            center_start_wall,
            textures.wall.clone(),
        );
    }
    spawn_tutorial_chunks(&mut commands, &textures);
}

fn spawn_tutorial_chunks(commands: &mut Commands, textures: &TextureAssets) {
    for index in 0..TUTORIAL_CHUNKS {
        for tile in 0..CHUNK_TILES {
            let center = Vec2::new(
                index as f32 * CHUNK_WIDTH + TILE_SIZE / 2. + tile as f32 * TILE_SIZE,
                TILE_SIZE / 2.,
            );
            let size = Vec2::new(TILE_SIZE, TILE_SIZE);
            spawn_tile(commands, size, center, textures.ground.clone());
        }
    }
}

fn spawn_tile(commands: &mut Commands, size: Vec2, position: Vec2, texture: Handle<Image>) {
    commands
        .spawn(SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(position.x, position.y, PLATFORM_Z)),
            ..default()
        })
        .insert(Collider { size })
        .insert(Solid);
}

#[derive(Default, Resource)]
pub struct CurrentChunk(pub(crate) usize);

fn spawn_chunk(commands: &mut Commands, textures: &TextureAssets, index: usize) {
    if index < TUTORIAL_CHUNKS {
        return;
    }
    info!("Spawning chunk {index}");
    let mut random = rand::thread_rng();
    let hole1 = random.gen_range(0..CHUNK_TILES);
    let hole2 = random.gen_range(0..CHUNK_TILES);
    for tile in 0..CHUNK_TILES {
        if tile == hole1 || tile == hole2 {
            continue;
        } else if index > MAP_GEN_DOUBLE_HOLES_FROM_CHUNK
            && (tile == hole1 + 1 || tile == hole2 + 1)
        {
            continue;
        }
        let center = Vec2::new(
            index as f32 * CHUNK_WIDTH + TILE_SIZE / 2. + tile as f32 * TILE_SIZE,
            PLATFORM_HEIGHT / 2.,
        );
        let size = Vec2::new(TILE_SIZE, PLATFORM_HEIGHT);
        spawn_tile(commands, size, center, textures.ground.clone());
        if random.gen::<f32>() < MAP_GEN_FOOD_ON_GROUND {
            spawn_random_food(&textures, commands, center, &mut random);
        }
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
