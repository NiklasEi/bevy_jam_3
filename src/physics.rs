use crate::map::{Collider, CurrentChunk, Solid, CHUNK_WIDTH};
use crate::player::{Grounded, Player, PlayerControls};
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

const GRAVITY: f32 = 12.;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PhysicsSystems::CalculateVelocities
                .run_if(in_state(GameState::Playing))
                .before(PhysicsSystems::Move),
        )
        .configure_set(PhysicsSystems::Move.run_if(in_state(GameState::Playing)))
        .configure_set(PhysicsSystems::Move.run_if(in_state(GameState::Playing)))
        .add_system(gravity.in_set(PhysicsSystems::CalculateVelocities))
        .add_system(move_player.in_set(PhysicsSystems::Move));
    }
}

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub enum PhysicsSystems {
    CalculateVelocities,
    Move,
}

#[derive(Component)]
pub struct Velocity(pub(crate) Vec2);

fn gravity(time: Res<Time>, mut falling: Query<&mut Velocity, (With<Player>, Without<Grounded>)>) {
    for mut velocity in &mut falling {
        velocity.0.y -= GRAVITY * time.delta_seconds();
    }
}

fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    player_controls: Res<PlayerControls>,
    mut player: Query<(Entity, &mut Transform, &mut Velocity, &Player)>,
    colliders: Query<(&Transform, &Collider), (Without<Player>, With<Solid>)>,
    mut current_chunk: ResMut<CurrentChunk>,
) {
    let (player_entity, mut player_transform, mut player_velocity, player) = player.single_mut();

    if player_velocity.0 == Vec2::ZERO {
        return;
    }
    info!("Checking player collisions");
    let mut grounded = false;

    let mut movement_y =
        Vec3::new(0., player_velocity.0.y, 0.) * time.delta_seconds() * player_controls.speed;
    let mut potential_new_position_y = player_transform.translation + movement_y;
    let mut new_rect_y = Rect::from_center_size(potential_new_position_y.xy(), player.size);
    for (collider_transform, collider) in &colliders {
        let collider_rect =
            Rect::from_center_size(collider_transform.translation.xy(), collider.size);
        let mut intersect = new_rect_y.intersect(collider_rect);
        while !intersect.is_empty() {
            if player_velocity.0.y < 0. {
                grounded = true;
            }
            player_velocity.0.y = 0.;

            if movement_y.y.abs() < intersect.height() {
                movement_y.y = 0.;
            } else if movement_y.y > 0. {
                movement_y.y -= intersect.height();
            } else if movement_y.y < 0. {
                movement_y.y += intersect.height();
            }

            potential_new_position_y = player_transform.translation + movement_y;
            new_rect_y = Rect::from_center_size(potential_new_position_y.xy(), player.size);
            intersect = new_rect_y.intersect(collider_rect);
        }
    }

    let mut movement_x =
        Vec3::new(player_velocity.0.x, 0., 0.) * time.delta_seconds() * player_controls.speed;
    let mut potential_new_position_x = potential_new_position_y + movement_x;
    let mut new_rect_x = Rect::from_center_size(potential_new_position_x.xy(), player.size);
    for (collider_transform, collider) in &colliders {
        let collider_rect =
            Rect::from_center_size(collider_transform.translation.xy(), collider.size);
        let mut intersect = new_rect_x.intersect(collider_rect);
        while !intersect.is_empty() {
            if movement_x.x.abs() < intersect.width() {
                movement_x.x = 0.;
            } else if movement_x.x > 0. {
                movement_x.x -= intersect.width();
            } else if movement_x.x < 0. {
                movement_x.x += intersect.width();
            }

            potential_new_position_x = potential_new_position_y + movement_x;
            new_rect_x = Rect::from_center_size(potential_new_position_x.xy(), player.size);
            intersect = new_rect_x.intersect(collider_rect);
        }
    }
    player_transform.translation = potential_new_position_x;
    let chunk = (player_transform.translation.x.abs() / CHUNK_WIDTH).floor() as usize;
    if chunk > current_chunk.0 {
        current_chunk.0 = chunk;
    }

    // let rect = Rect::from_center_size(player_transform.translation.xy(), player.size);
    // for (collider_transform, collider) in &colliders {
    //     let collider_rect = Rect::from_center_size(collider_transform.translation.xy(), collider.size);
    //     let intersect = rect.intersect(collider_rect);
    //     if !intersect.is_empty() {
    //         println!("stuck!!! {intersect:?}");
    //     }
    // }
    if grounded {
        commands.entity(player_entity).insert(Grounded);
    } else if movement_x.y.abs() > 0. {
        commands.entity(player_entity).remove::<Grounded>();
    } else {
        let new_rect = Rect::from_center_size(
            player_transform.translation.xy() + Vec2::new(0., -0.1),
            player.size,
        );
        for (collider_transform, collider) in &colliders {
            let collider_rect =
                Rect::from_center_size(collider_transform.translation.xy(), collider.size);
            let intersect = new_rect.intersect(collider_rect);
            if !intersect.is_empty() {
                return;
            }
        }
        commands.entity(player_entity).remove::<Grounded>();
    }
}
