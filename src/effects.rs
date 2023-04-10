use crate::loading::TextureAssets;
use crate::map::{Collider, Level};
use crate::physics::{Move, PhysicsSystems, Velocity};
use crate::player::{Player, PlayerControls};
use crate::{GameState, HEIGHT, WIDTH};
use bevy::ecs::query::WorldQuery;
use bevy::ecs::system::Command;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;
use std::collections::HashMap;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentEffects>()
            .add_system(end_effects.in_set(OnUpdate(GameState::Playing)))
            .add_system(move_bird.before(PhysicsSystems::CalculateVelocities));
    }
}

#[derive(Resource, Default)]
struct CurrentEffects(HashMap<Effect, f32>);

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Effect {
    Faster,
    JumpPower,
    Bird,
    Shrink,
    Grow,
}

impl Distribution<Effect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Effect {
        match rng.gen_range(1..=5) {
            1 => Effect::Faster,
            2 => Effect::JumpPower,
            3 => Effect::Shrink,
            4 => Effect::Grow,
            _ => Effect::Bird,
        }
    }
}

impl Effect {
    fn duration(&self) -> f32 {
        match self {
            Effect::Faster => 5.,
            Effect::JumpPower => 5.,
            Effect::Shrink => 10.,
            Effect::Grow => 10.,
            Effect::Bird => 30.,
        }
    }
}

pub struct StartEffect(pub Effect);

impl Command for StartEffect {
    fn write(self, world: &mut World) {
        info!("Trying start effect {:?}", self.0);
        let current_effects = world.get_resource_mut::<CurrentEffects>().unwrap();
        if !current_effects.0.contains_key(&self.0) {
            info!("Starting effect {:?}", self.0);
            match self.0 {
                Effect::Faster => {
                    let mut controls = world.get_resource_mut::<PlayerControls>().unwrap();
                    controls.speed = 350.;
                }
                Effect::JumpPower => {
                    let mut controls = world.get_resource_mut::<PlayerControls>().unwrap();
                    controls.jump_power = 1400.;
                }
                Effect::Shrink => {
                    let mut query =
                        world.query_filtered::<(&mut Transform, &mut Collider), With<Player>>();
                    let (mut transform, mut collider) = query.single_mut(world);
                    transform.scale = Vec3::splat(1.5);
                    collider.size = Vec2::new(56. * 3. / 4., 44. * 3. / 4.);
                    let mut current_events = world.get_resource_mut::<CurrentEffects>().unwrap();
                    current_events.0.remove(&Effect::Grow);
                }
                Effect::Grow => {
                    let mut query =
                        world.query_filtered::<(&mut Transform, &mut Collider), With<Player>>();
                    let (mut transform, mut collider) = query.single_mut(world);
                    transform.scale = Vec3::splat(2.5);
                    transform.translation.y += 6.5;
                    collider.size = Vec2::new(56. * 5. / 4., 44. * 5. / 4.);
                    let mut current_events = world.get_resource_mut::<CurrentEffects>().unwrap();
                    current_events.0.remove(&Effect::Shrink);
                }
                Effect::Bird => {
                    spawn_bird(world);
                }
            }
        }
        let time = world.get_resource::<Time>().unwrap().elapsed_seconds();
        let mut current_events = world.get_resource_mut::<CurrentEffects>().unwrap();
        let duration = self.0.duration();
        current_events.0.insert(self.0, time + duration);
    }
}

struct EndEffect(Effect);

impl Command for EndEffect {
    fn write(self, world: &mut World) {
        info!("Ending effect {:?}", self.0);
        match self.0 {
            Effect::Faster => {
                let mut controls = world.get_resource_mut::<PlayerControls>().unwrap();
                controls.speed = PlayerControls::default().speed;
            }
            Effect::JumpPower => {
                let mut controls = world.get_resource_mut::<PlayerControls>().unwrap();
                controls.jump_power = PlayerControls::default().jump_power;
            }
            Effect::Bird => {}
            Effect::Shrink | Effect::Grow => {
                let mut query =
                    world.query_filtered::<(&mut Transform, &mut Collider), With<Player>>();
                let (mut transform, mut collider) = query.single_mut(world);
                transform.scale = Vec3::splat(2.);
                if self.0 == Effect::Shrink {
                    transform.translation.y += 6.5;
                }
                collider.size = Vec2::new(56., 44.);
            }
        }
    }
}

fn end_effects(
    mut commands: Commands,
    mut current_effects: ResMut<CurrentEffects>,
    time: Res<Time>,
) {
    current_effects.0.retain(|effect, end| {
        if *end < time.elapsed_seconds() {
            commands.add(EndEffect(effect.clone()));

            false
        } else {
            true
        }
    });
}

#[derive(Component)]
pub struct Bird;

#[derive(WorldQuery)]
struct PlayerQuery {
    player_transform: &'static Transform,
    with_player: With<Player>,
}

pub const BIRD_Z: f32 = 11.;

fn spawn_bird(world: &mut World) {
    let bird_texture = world.get_resource::<TextureAssets>().unwrap().bird.clone();
    let mut query = world.query_filtered::<&Transform, With<Player>>();
    let player_query = query.single(world);
    world
        .spawn(SpriteBundle {
            texture: bird_texture,
            transform: Transform::from_translation(Vec3::new(
                player_query.translation.x - WIDTH / 2.,
                HEIGHT,
                BIRD_Z,
            )),
            ..default()
        })
        .insert(Collider {
            size: Vec2::splat(25.),
        })
        .insert(Velocity(Vec2::ZERO))
        .insert(Level)
        .insert(Bird)
        .insert(Move);
}

fn move_bird(
    mut bird: Query<(Entity, &mut Velocity, &Transform), With<Bird>>,
    target: Query<&Transform, (With<Player>, Without<Bird>)>,
    current_effects: ResMut<CurrentEffects>,
    mut commands: Commands,
) {
    for (bird, mut velocity, bird_transform) in bird.iter_mut() {
        if bird_transform.translation.y > HEIGHT * 1.2 {
            commands.entity(bird).despawn();
            continue;
        }
        if !current_effects.0.contains_key(&Effect::Bird) {
            velocity.0 = Vec2::splat(1.) * 200.;
        } else {
            let diff = target.single().translation.xy() - bird_transform.translation.xy();
            velocity.0 = diff.normalize() * 200.;
        }
    }
}
