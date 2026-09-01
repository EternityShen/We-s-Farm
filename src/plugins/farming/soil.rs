use std::collections::HashSet;

use bevy::prelude::*;

use crate::utils::pos;

#[derive(Resource, Default)]
pub struct SoilSet {
    pub map: HashSet<IVec2>,
}

impl SoilSet {
    pub fn is_soil(&self, pos: IVec2) -> bool {
        self.map.contains(&pos)
    }

    pub fn insert(&mut self, pos: IVec2) {
        self.map.insert(pos);
    }

    pub fn remove(&mut self, pos: IVec2) {
        self.map.remove(&pos);
    }
}

#[derive(Message)]
pub struct SoilMessage {
    pub pos: IVec2,
}

#[derive(Component)]
pub struct Soil;

pub struct SoilPlugin;

impl Plugin for SoilPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoilSet>();
        app.add_message::<SoilMessage>();
        app.add_systems(Update, soil_listener);
    }
}

pub fn soil_listener(
    mut commands: Commands,
    mut reader: MessageReader<SoilMessage>,
    mut soil_set: ResMut<SoilSet>,
) {
    for message in reader.read() {
        let world_pos = pos::tile_to_world(message.pos);
        if !soil_set.is_soil(message.pos) {
            soil_set.insert(message.pos);
            commands.spawn((
                Sprite::default(),
                Transform::from_xyz(world_pos.x, world_pos.y, 5.0),
                Soil,
            ));
        }
    }
}
