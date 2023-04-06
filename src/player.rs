use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::map::TILE_SIZE;
use crate::physics::{PhysicsSystems, Velocity};
use crate::{GameState, HEIGHT, WIDTH};
use bevy::prelude::*;

pub const PLAYER_Z: f32 = 5.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub(crate) size: Vec2,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Resource)]
pub struct Hunger(pub(crate) f32);

impl Default for Hunger {
    fn default() -> Self {
        Hunger(100.)
    }
}

#[derive(Resource)]
pub struct HungerPerSecond(f32);

impl Default for HungerPerSecond {
    fn default() -> Self {
        HungerPerSecond(1.)
    }
}

#[derive(Resource)]
pub struct PlayerControls {
    pub jump_power: f32,
    pub speed: f32,
}

impl Default for PlayerControls {
    fn default() -> Self {
        PlayerControls {
            jump_power: 3.5,
            speed: 250.,
        }
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hunger>()
            .init_resource::<PlayerControls>()
            .init_resource::<HungerPerSecond>()
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(apply_actions.in_set(PhysicsSystems::CalculateVelocities))
            .add_systems(
                (lose_on_falling.after(PhysicsSystems::Move), process_food)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

fn lose_on_falling(player: Query<&Transform, With<Player>>) {
    if player.single().translation.y < TILE_SIZE {
        println!("Fell!");
    }
}

fn process_food(
    time: Res<Time>,
    hunger_per_second: Res<HungerPerSecond>,
    mut hunger: ResMut<Hunger>,
) {
    hunger.0 -= hunger_per_second.0 * time.delta_seconds();
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(25.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(WIDTH / 2., HEIGHT / 2., PLAYER_Z)),
            ..Default::default()
        })
        .insert(Player {
            size: Vec2::new(25., 25.),
        })
        .insert(Velocity(Vec2::ZERO));
}

fn apply_actions(
    actions: Res<Actions>,
    player_controls: Res<PlayerControls>,
    mut player_query: Query<(Entity, &mut Velocity), With<Player>>,
    can_jump: Query<&Grounded, With<Player>>,
) {
    let (player, mut velocity) = player_query.single_mut();
    velocity.0.x = actions.player_movement;

    if actions.attempt_jump && can_jump.contains(player) {
        velocity.0.y = player_controls.jump_power;
    }
}
