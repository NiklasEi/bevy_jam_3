use crate::loading::TextureAssets;
use crate::map::{Collider, TILE_SIZE};
use crate::physics::PhysicsSystems;
use crate::player::{Hunger, Player};
use crate::ui::Score;
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use rand::prelude::*;

pub const FOOD_SIZE: f32 = 16.;
pub const FOOD_Z: f32 = 9.;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (eat, collect)
                .after(PhysicsSystems::Move)
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

fn eat(
    mut commands: Commands,
    player: Query<(&Transform, &Player)>,
    food: Query<(Entity, &Transform, &Collider, &Food), Without<Player>>,
    mut hunger: ResMut<Hunger>,
) {
    let (player_transform, player_collider) = player.single();
    let player_rect =
        Rect::from_center_size(player_transform.translation.xy(), player_collider.size);
    for (food, food_transform, food_collider, food_value) in &food {
        let food_rect = Rect::from_center_size(food_transform.translation.xy(), food_collider.size);
        if !food_rect.intersect(player_rect).is_empty() {
            hunger.0 += food_value.value;
            hunger.0 = hunger.0.clamp(0., 100.);
            commands.entity(food).despawn();
        }
    }
}

fn collect(
    mut commands: Commands,
    player: Query<(&Transform, &Player)>,
    food: Query<(Entity, &Transform, &Collider, &Truffle), Without<Player>>,
    mut hunger: ResMut<Hunger>,
    mut score: ResMut<Score>,
) {
    let (player_transform, player_collider) = player.single();
    let player_rect =
        Rect::from_center_size(player_transform.translation.xy(), player_collider.size);
    for (truffle, food_transform, food_collider, truffle_value) in &food {
        let food_rect = Rect::from_center_size(food_transform.translation.xy(), food_collider.size);
        if !food_rect.intersect(player_rect).is_empty() {
            score.0 += 1.;
            hunger.0 += truffle_value.value;
            hunger.0 = hunger.0.clamp(0., 100.);
            commands.entity(truffle).despawn();
        }
    }
}

#[derive(Component)]
pub struct Food {
    value: f32,
}

#[derive(Component)]
pub struct Truffle {
    value: f32,
}

pub fn spawn_random_food(
    textures: &TextureAssets,
    commands: &mut Commands,
    tile: Vec2,
    random: &mut ThreadRng,
) {
    let food_index = random.gen_range(0..textures.food.len());
    let food_texture = textures.food.get(food_index).unwrap().clone();
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                tile.x,
                tile.y + TILE_SIZE / 2. + FOOD_SIZE / 2.,
                FOOD_Z,
            )),
            texture: food_texture,
            ..default()
        })
        .insert(Collider {
            size: Vec2::splat(16.),
        })
        .insert(Food { value: 5. });
}

pub fn spawn_truffle(textures: &TextureAssets, commands: &mut Commands, tile: Vec2) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                tile.x,
                tile.y + TILE_SIZE / 2. + FOOD_SIZE / 2.,
                FOOD_Z,
            )),
            texture: textures.truffle.clone(),
            ..default()
        })
        .insert(Collider {
            size: Vec2::splat(16.),
        })
        .insert(Truffle { value: 10. });
}
