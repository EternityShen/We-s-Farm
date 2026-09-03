use bevy::prelude::*;

pub mod plant;
pub mod runtime;
pub mod soil;

pub struct FarmIngPlugin;

impl Plugin for FarmIngPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(soil::SoilPlugin);
        app.add_plugins(plant::PlantPlugin);
        app.add_plugins(runtime::FarmRunTimePlugin);
    }
}
