use std::collections::HashMap;

use bevy::prelude::*;

use super::soil::SoilSet;

use crate::utils::pos;

#[derive(Message)]
pub struct PlantMessage {
    pub pos: IVec2,
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_index: usize,
    pub last_index: usize,
    pub time: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: f32) -> Self {
        Self {
            first_index: first,
            last_index: last,
            time: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
        }
    }
}

pub enum CropType {
    Wheat,
}

pub struct CropStage {
    pub seed: f32,
    pub chind: f32,
    pub mid_age: f32,
    pub mature: f32,
}

pub const WHEAT_STAGE: CropStage = CropStage {
    seed: 0.0,
    chind: 10.0,
    mid_age: 20.0,
    mature: 30.0,
};

pub struct CropTime {
    pub grow: f32,
    pub age: f32,
}

#[derive(Component)]
pub struct Crop {
    pub crop_type: CropType,
    pub time: CropTime,
    pub stage: CropStage,
}

#[derive(Resource, Default)]
pub struct CropMap {
    pub map: HashMap<IVec2, Entity>,
}

impl CropMap {
    pub fn get(&self, pos: IVec2) -> Option<Entity> {
        self.map.get(&pos).cloned()
    }

    pub fn is_crop(&self, pos: IVec2) -> bool {
        self.map.contains_key(&pos)
    }

    pub fn insert(&mut self, pos: IVec2, entity: Entity) {
        self.map.insert(pos, entity);
    }

    pub fn remove(&mut self, pos: IVec2) {
        self.map.remove(&pos);
    }
}

pub struct PlantPlugin;

impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CropMap>();
        app.add_message::<PlantMessage>();
        app.add_systems(Update, plant_listener);
        app.add_systems(Update, update_crop_animation_state);
        app.add_systems(Update, animate_crop_sprite);
    }
}

pub fn plant_listener(
    mut commands: Commands,
    mut reader: MessageReader<PlantMessage>,
    soil_set: Res<SoilSet>,
    crop_map: Res<CropMap>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    for message in reader.read() {
        if soil_set.is_soil(message.pos) && !crop_map.is_crop(message.pos) {
            let texture = asset_server.load("TilesImage/小麦图集.png");
            let layout = TextureAtlasLayout::from_grid(UVec2 { x: 16, y: 24 }, 4, 4, None, None);
            let atalas_layout = texture_atlas_layout.add(layout);

            let anim_config = AnimationConfig::new(0, 3, 4.0);

            let world_pos = pos::tile_to_world(message.pos);

            let y_sort_z = pos::y_sort(world_pos.y);

            commands.spawn((
                Sprite::from_atlas_image(
                    texture,
                    TextureAtlas {
                        layout: atalas_layout,
                        index: 0,
                    },
                ),
                Transform::from_xyz(world_pos.x, world_pos.y + 5.0, y_sort_z),
                Crop {
                    crop_type: CropType::Wheat,
                    time: CropTime {
                        grow: 100.0,
                        age: 0.0,
                    },
                    stage: WHEAT_STAGE,
                },
                anim_config,
            ));
        }
    }
}

pub fn update_crop_animation_state(query: Query<(&mut AnimationConfig, &mut Crop), With<Crop>>) {
    for (mut config, crop) in query {
        if crop.time.age > crop.stage.mature {
            config.first_index = 12;
            config.last_index = 15;
        } else if crop.time.age > crop.stage.mid_age {
            config.first_index = 8;
            config.last_index = 11;
        } else if crop.time.age > crop.stage.chind {
            config.first_index = 4;
            config.last_index = 7;
        } else if crop.time.age > crop.stage.seed {
            config.first_index = 0;
            config.last_index = 3;
        }
    }
}

pub fn animate_crop_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Crop>>,
) {
    for (mut config, mut sprite) in query.iter_mut() {
        config.time.tick(time.delta());

        if config.time.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index >= config.last_index || atlas.index < config.first_index {
                atlas.index = config.first_index;
            } else {
                atlas.index += 1;
            }
        }
    }
}
