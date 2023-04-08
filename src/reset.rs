use crate::loading::FontAssets;
use crate::menu::ButtonColors;
use crate::physics::Velocity;
use crate::player::{Grounded, Hunger, Player, PLAYER_Z};
use crate::ui::Score;
use crate::{GameState, HEIGHT, WIDTH};
use bevy::prelude::*;

pub struct ResetPlugin;

impl Plugin for ResetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (reset_player, cleanup_restart, reset_hunger, reset_score)
                .in_schedule(OnExit(GameState::Restart)),
        )
        .add_system(setup_restart.in_schedule(OnEnter(GameState::Restart)))
        .add_system(click_restart_button.in_set(OnUpdate(GameState::Restart)));
    }
}

fn reset_player(
    mut player: Query<(Entity, &mut Transform, &mut Velocity), With<Player>>,
    mut commands: Commands,
) {
    let (entity, mut transform, mut velocity) = player.single_mut();
    commands.entity(entity).remove::<Grounded>();
    transform.translation = Vec3::new(WIDTH / 2., HEIGHT / 2., PLAYER_Z);
    velocity.0 = Vec2::ZERO;
}

fn reset_hunger(mut hunger: ResMut<Hunger>) {
    *hunger = Hunger::default();
}

fn reset_score(mut score: ResMut<Score>) {
    *score = Score::default();
}

fn setup_restart(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Again!",
                TextStyle {
                    font: font_assets.fira_sans.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn click_restart_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    input: Res<Input<KeyCode>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
    if input.just_pressed(KeyCode::Return) {
        state.set(GameState::Playing);
    }
}

fn cleanup_restart(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}
