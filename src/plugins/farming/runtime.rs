use bevy::prelude::*;

use super::plant::Crop;

pub struct FarmRunTimePlugin;

impl Plugin for FarmRunTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_crop_age);
    }
}

pub fn update_crop_age(time: Res<Time>, query: Query<&mut Crop>) {
    for mut crop in query {
        crop.time.age += time.delta_secs();
    }
}
