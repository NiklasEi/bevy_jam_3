use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_system(set_movement_actions.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: f32,
    pub attempt_jump: bool,
}

pub fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    actions.player_movement = get_movement(GameControl::Right, &keyboard_input)
        - get_movement(GameControl::Left, &keyboard_input);

    actions.attempt_jump = GameControl::Jump.just_pressed(&keyboard_input);
}
