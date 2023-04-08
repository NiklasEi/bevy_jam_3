use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::map::TILE_SIZE;
use crate::physics::{PhysicsSystems, Velocity};
use crate::{GameState, HEIGHT, WIDTH};
use bevy::prelude::*;

pub const PLAYER_Z: f32 = 10.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub(crate) size: Vec2,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TakeInputs(pub(crate) bool);

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Falling(pub(crate) bool);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Hunger(pub(crate) f32);

impl Default for Hunger {
    fn default() -> Self {
        Hunger(100.)
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct HungerPerSecond(f32);

impl Default for HungerPerSecond {
    fn default() -> Self {
        HungerPerSecond(1.)
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
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

#[derive(Component)]
struct AnimationTimer(Timer, usize);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hunger>()
            .init_resource::<PlayerControls>()
            .init_resource::<HungerPerSecond>()
            .insert_resource(TakeInputs(true))
            .insert_resource(Falling(false))
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Prepare)))
            .add_system(apply_actions.in_set(PhysicsSystems::CalculateVelocities))
            .add_systems(
                (lose_on_falling.after(PhysicsSystems::Move), process_food)
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(animate_player.after(apply_actions));
    }
}

fn lose_on_falling(player: Query<&Transform, With<Player>>) {
    if player.single().translation.y < TILE_SIZE {
        println!("Fell!");
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Velocity), With<Player>>,
) {
    for (mut timer, mut sprite, velocity) in &mut query {
        if velocity.0.length() < f32::EPSILON || velocity.0.y.abs() > 0. {
            sprite.index = 0;
            timer.0.reset();
            continue;
        }
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % timer.1;
        }
    }
}

fn process_food(
    time: Res<Time>,
    mut state: ResMut<NextState<GameState>>,
    hunger_per_second: Res<HungerPerSecond>,
    mut hunger: ResMut<Hunger>,
) {
    hunger.0 -= hunger_per_second.0 * time.delta_seconds();
    if hunger.0 < 0. {
        state.set(GameState::Restart);
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.pig.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(WIDTH / 2., HEIGHT / 2., PLAYER_Z)),
            ..Default::default()
        })
        .insert(Player {
            size: Vec2::new(30., 23.),
        })
        .insert(Velocity(Vec2::ZERO))
        .insert(AnimationTimer(
            Timer::from_seconds(0.15, TimerMode::Repeating),
            4,
        ));
}

fn apply_actions(
    actions: Res<Actions>,
    player_controls: Res<PlayerControls>,
    mut player_query: Query<(Entity, &mut Velocity, &mut TextureAtlasSprite), With<Player>>,
    can_jump: Query<&Grounded, With<Player>>,
) {
    let (player, mut velocity, mut sprite) = player_query.single_mut();
    velocity.0.x = actions.player_movement;
    if velocity.0.x.abs() > 0. {
        sprite.flip_x = velocity.0.x < 0.;
    }
    if actions.attempt_jump && can_jump.contains(player) {
        velocity.0.y = player_controls.jump_power;
    }
}
