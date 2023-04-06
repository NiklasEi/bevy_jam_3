use crate::GameState;
use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_system(configure_samplers.in_schedule(OnExit(GameState::Loading)));
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
    #[asset(path = "textures/ground.png")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/platform.png")]
    pub platform: Handle<Image>,
    #[asset(path = "textures/wall.png")]
    pub wall: Handle<Image>,
    #[asset(path = "textures/truffle.png")]
    pub truffle: Handle<Image>,
    #[asset(
        collection(typed),
        paths(
            "textures/food/food1.png",
            "textures/food/food2.png",
            "textures/food/food3.png"
        )
    )]
    pub food: Vec<Handle<Image>>,
}

fn configure_samplers(texture_assets: Res<TextureAssets>, mut textures: ResMut<Assets<Image>>) {
    let mut repeat_descriptor = SamplerDescriptor::default();
    repeat_descriptor.address_mode_u = AddressMode::Repeat;
    repeat_descriptor.address_mode_v = AddressMode::Repeat;
    repeat_descriptor.address_mode_w = AddressMode::Repeat;

    let mut ground = textures.get_mut(&texture_assets.ground).unwrap();
    ground.sampler_descriptor = ImageSampler::Descriptor(repeat_descriptor.clone());

    let mut platform = textures.get_mut(&texture_assets.platform).unwrap();
    platform.sampler_descriptor = ImageSampler::Descriptor(repeat_descriptor.clone());

    let mut wall = textures.get_mut(&texture_assets.wall).unwrap();
    wall.sampler_descriptor = ImageSampler::Descriptor(repeat_descriptor);
}
