use crate::physics::PhysicsSystems;
use crate::player::Player;
use crate::{GameState, HEIGHT, WIDTH};
use bevy::prelude::*;
use bevy_parallax::{
    LayerData, LayerSpeed, ParallaxMoveEvent, ParallaxPlugin, ParallaxResource, ParallaxSystems,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ParallaxPlugin)
            .add_system(
                follow_player
                    .run_if(in_state(GameState::Playing))
                    .after(PhysicsSystems::Move)
                    .before(ParallaxSystems),
            )
            .insert_resource(ParallaxResource {
                layer_data: vec![
                    LayerData {
                        speed: LayerSpeed::Horizontal(0.9),
                        path: "textures/back.png".to_string(),
                        tile_size: Vec2::new(1024.0, 600.0),
                        position: Vec2::new(WIDTH / 2., HEIGHT / 2.),
                        z: 1.0,
                        ..Default::default()
                    },
                    LayerData {
                        speed: LayerSpeed::Horizontal(0.7),
                        path: "textures/middle.png".to_string(),
                        tile_size: Vec2::new(1024.0, 600.0),
                        position: Vec2::new(WIDTH / 2., HEIGHT / 2.),
                        z: 2.0,
                        ..Default::default()
                    },
                    LayerData {
                        speed: LayerSpeed::Horizontal(0.4),
                        path: "textures/front.png".to_string(),
                        tile_size: Vec2::new(1024.0, 600.0),
                        position: Vec2::new(WIDTH / 2., HEIGHT / 2.),
                        z: 3.0,
                        ..Default::default()
                    },
                    LayerData {
                        speed: LayerSpeed::Horizontal(0.1),
                        path: "textures/very_front.png".to_string(),
                        tile_size: Vec2::new(1024.0, 600.0),
                        position: Vec2::new(WIDTH / 2., HEIGHT / 2.),
                        z: 4.0,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            });
    }
}

const THRESHOLD: f32 = 0.;

#[derive(Component)]
pub struct GameCamera;

fn follow_player(
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, (With<GameCamera>, Without<Player>)>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    let camera_transform = camera.single();
    let delta = player.single().translation.x - camera_transform.translation.x;
    if delta.abs() > THRESHOLD {
        let mut move_by = if delta > 0. {
            delta - THRESHOLD
        } else {
            delta + THRESHOLD
        };
        if camera_transform.translation.x + move_by < WIDTH / 2. {
            move_by = (WIDTH / 2.) - camera_transform.translation.x;
        }
        move_event_writer.send(ParallaxMoveEvent {
            camera_move_speed: Vec2::new(move_by, 0.0),
        });
    }
}
