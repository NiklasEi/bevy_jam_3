use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastGrunt>()
            .add_plugin(AudioPlugin)
            .add_audio_channel::<Background>()
            .add_system(start_background.in_schedule(OnEnter(GameState::Menu)))
            .add_system(random_grunting.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Resource)]
struct Background;

fn start_background(audio: Res<AudioChannel<Background>>, audio_assets: Res<AudioAssets>) {
    audio.play(audio_assets.birds.clone()).looped();
}

#[derive(Resource, Default)]
struct LastGrunt(f32);

fn random_grunting(
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut last_grunt: ResMut<LastGrunt>,
) {
    let mut random = thread_rng();
    if time.elapsed_seconds() - last_grunt.0 > 1.5
        && random.gen::<f32>() < 0.1 * time.delta_seconds()
    {
        last_grunt.0 = time.elapsed_seconds();
        audio
            .play(audio_assets.random_grunt(&mut random))
            .with_volume(0.05);
    }
}
