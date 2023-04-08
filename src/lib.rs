#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod camera;
mod food;
mod loading;
mod map;
mod menu;
mod physics;
mod player;
mod reset;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use crate::camera::CameraPlugin;
use crate::food::FoodPlugin;
use crate::map::MapPlugin;
use crate::physics::PhysicsPlugin;
use crate::reset::ResetPlugin;
use crate::ui::UiPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 600.;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Prepare,
    Playing,
    Restart,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_system(start_level.in_set(OnUpdate(GameState::Prepare)))
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(FoodPlugin)
            .add_plugin(ResetPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(EditorPlugin::new().in_new_window(Window::default()))
                .add_system(reset);
        }
    }
}

fn start_level(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}

fn reset(mut state: ResMut<NextState<GameState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::R) {
        state.set(GameState::Restart);
    }
}
