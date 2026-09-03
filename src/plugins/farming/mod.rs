use bevy::prelude::*;

pub mod soil;

pub struct FarmIngPlugin;

impl Plugin for FarmIngPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(soil::SoilPlugin);
    }
}
