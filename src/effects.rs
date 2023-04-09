use crate::player::PlayerControls;
use crate::GameState;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;
use std::collections::HashMap;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentEffects>()
            .add_system(end_effects.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Resource, Default)]
struct CurrentEffects(HashMap<Effect, f32>);

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Effect {
    Faster,
    JumpPower,
}

impl Distribution<Effect> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Effect {
        match rng.gen_range(1..=2) {
            1 => Effect::Faster,
            _ => Effect::JumpPower,
        }
    }
}

impl Effect {
    fn duration(&self) -> f32 {
        match self {
            Effect::Faster => 5.,
            Effect::JumpPower => 5.,
        }
    }
}

pub struct StartEffect(pub Effect);

impl Command for StartEffect {
    fn write(self, world: &mut World) {
        match self.0 {
            Effect::Faster => {
                let mut controls = world.get_resource_mut::<PlayerControls>().unwrap();
                controls.speed = 350.;
            }
            Effect::JumpPower => {
                let mut controls = world.get_resource_mut::<PlayerControls>().unwrap();
                controls.jump_power = 1400.;
            }
        }
        let time = world.get_resource::<Time>().unwrap().elapsed_seconds();
        let mut current_events = world.get_resource_mut::<CurrentEffects>().unwrap();
        let duration = self.0.duration();
        current_events.0.insert(self.0, time + duration);
    }
}

struct EndEffect(Effect);

impl Command for EndEffect {
    fn write(self, world: &mut World) {
        match self.0 {
            Effect::Faster => {
                let mut controls = world.get_resource_mut::<PlayerControls>().unwrap();
                controls.speed = PlayerControls::default().speed;
            }
            Effect::JumpPower => {
                let mut controls = world.get_resource_mut::<PlayerControls>().unwrap();
                controls.jump_power = PlayerControls::default().jump_power;
            }
        }
    }
}

fn end_effects(
    mut commands: Commands,
    mut current_effects: ResMut<CurrentEffects>,
    time: Res<Time>,
) {
    current_effects.0.retain(|effect, end| {
        if *end < time.elapsed_seconds() {
            commands.add(EndEffect(effect.clone()));

            false
        } else {
            true
        }
    });
}
