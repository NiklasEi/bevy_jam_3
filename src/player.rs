use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::physics::{PhysicsSystems, Velocity};
use crate::GameState;
use bevy::prelude::*;

pub const PLAYER_Z: f32 = 1.;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub(crate) size: Vec2,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

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
        app.init_resource::<PlayerControls>()
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(apply_actions.in_set(PhysicsSystems::CalculateVelocities));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(25.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., PLAYER_Z)),
            ..Default::default()
        })
        .insert(Player {
            size: Vec2::new(10., 10.),
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
