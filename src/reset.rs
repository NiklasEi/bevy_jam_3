use bevy::prelude::*;
use crate::{GameState, HEIGHT, WIDTH};
use crate::physics::Velocity;
use crate::player::{Grounded, Player, PLAYER_Z};

pub struct ResetPlugin;

impl Plugin for ResetPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(reset_player.in_schedule(OnExit(GameState::Restart)));
    }
}

pub fn reset_player(
    input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut Transform, &mut Velocity), With<Player>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::R) {
        let (entity, mut transform, mut velocity) = player.single_mut();
        commands.entity(entity).remove::<Grounded>();
        transform.translation = Vec3::new(WIDTH / 2., HEIGHT / 2., PLAYER_Z);
        velocity.0 = Vec2::ZERO;
    }
}
