mod actions;
mod audio;
mod loading;
mod map;
mod menu;
mod physics;
mod player;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::{Grounded, Player, PlayerPlugin, PLAYER_Z};

use crate::map::MapPlugin;
use crate::physics::{PhysicsPlugin, Velocity};
use crate::ui::UiPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(UiPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_system(reset_player);
        }
    }
}

fn reset_player(
    input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut Transform, &mut Velocity), With<Player>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::R) {
        let (entity, mut transform, mut velocity) = player.single_mut();
        commands.entity(entity).remove::<Grounded>();
        transform.translation = Vec3::new(0., 0., PLAYER_Z);
        velocity.0 = Vec2::ZERO;
    }
}

pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 600.;
