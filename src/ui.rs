use crate::loading::FontAssets;
use crate::player::Hunger;
use crate::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_score, spawn_hunger).in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (update_score_text, update_hunger_text).in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
pub struct Score(f32);

#[derive(Component)]
struct ScoreText;

fn spawn_score(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.insert_resource(Score(0.));
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::Rgba {
                red: 0.7,
                green: 0.7,
                blue: 0.7,
                alpha: 0.7,
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb_u8(34, 32, 52),
                            },
                        }],
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}

fn update_score_text(score: Res<Score>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }
    score_text.single_mut().sections[0].value = format!("{:.0}", score.0);
}

#[derive(Component)]
struct HungerText;

fn spawn_hunger(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.insert_resource(Score(0.));
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: UiRect {
                    right: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::Rgba {
                red: 0.7,
                green: 0.7,
                blue: 0.7,
                alpha: 0.7,
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("{:.0}", Hunger::default().0),
                            style: TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb_u8(34, 32, 52),
                            },
                        }],
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(HungerText);
        });
}

fn update_hunger_text(hunger: Res<Hunger>, mut hunger_text: Query<&mut Text, With<HungerText>>) {
    if !hunger.is_changed() {
        return;
    }
    hunger_text.single_mut().sections[0].value = format!("{:.0}", hunger.0);
}
