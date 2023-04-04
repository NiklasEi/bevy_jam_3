use crate::physics::PhysicsSystems;
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            follow_player
                .run_if(in_state(GameState::Playing))
                .after(PhysicsSystems::Move),
        );
    }
}

const THRESHOLD: f32 = 100.;

#[derive(Component)]
pub struct GameCamera;

fn follow_player(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let mut camera_transform = camera.single_mut();
    let delta = player.single().translation.x - camera_transform.translation.x;
    if delta.abs() > THRESHOLD {
        let move_by = if delta > 0. {
            delta - THRESHOLD
        } else {
            delta + THRESHOLD
        };
        camera_transform.translation.x += move_by;
        if camera_transform.translation.x < 0. {
            camera_transform.translation.x = 0.;
        }
    }
}
