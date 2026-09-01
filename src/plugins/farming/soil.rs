use std::collections::HashMap;

use bevy::prelude::*;

use crate::utils::pos;

#[derive(Resource, Default)]
pub struct SoilSet {
    pub map: HashMap<IVec2, Entity>,
}

// 4 个方向的偏移与对应的二进制权值 (North=1, East=2, South=4, West=8)
const NEIGHBORS: [(IVec2, usize); 4] = [
    (IVec2::new(0, 1), 1 << 0),  // 上 (North)
    (IVec2::new(1, 0), 1 << 1),  // 右 (East)
    (IVec2::new(0, -1), 1 << 2), // 下 (South)
    (IVec2::new(-1, 0), 1 << 3), // 左 (West)
];

impl SoilSet {
    pub fn get(&self, pos: IVec2) -> Option<Entity> {
        self.map.get(&pos).cloned()
    }

    pub fn is_soil(&self, pos: IVec2) -> bool {
        self.map.contains_key(&pos)
    }

    pub fn insert(&mut self, pos: IVec2, entity: Entity) {
        self.map.insert(pos, entity);
    }

    pub fn remove(&mut self, pos: IVec2) {
        self.map.remove(&pos);
    }

    // 计算某个坐标周围 4 邻居的 4-bit 掩码
    pub fn calculate_mask(&self, pos: IVec2) -> usize {
        let mut mask = 0;
        for (offset, bit) in NEIGHBORS {
            if self.is_soil(pos + offset) {
                mask |= bit;
            }
        }
        mask
    }
}

// 存储泥土贴图图集与 16 态掩码映射表
#[derive(Resource)]
pub struct SoilAssets {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
    pub mask_to_index: [usize; 16],
}

// 初始化泥土图集
fn setup_soil_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("TilesImage/土地图集.png");

    // 泥土图集是 16x16 切片，共 16 帧（例如 4x4 网格排布）
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 4, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let mut mask_to_index = [0; 16];
    (0..16).for_each(|mask| {
        mask_to_index[mask] = mask;
    });

    commands.insert_resource(SoilAssets {
        layout: layout_handle,
        texture,
        mask_to_index,
    });
}
pub enum SoilCtrl {
    Add,
    ReMove,
}

#[derive(Message)]
pub struct SoilMessage {
    pub ctrl: SoilCtrl,
    pub pos: IVec2,
}

#[derive(Component)]
pub struct Soil;

pub struct SoilPlugin;

impl Plugin for SoilPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoilSet>();
        app.add_message::<SoilMessage>();
        app.add_systems(Startup, setup_soil_assets);
        app.add_systems(Update, soil_listener);
    }
}

pub fn soil_listener(
    mut commands: Commands,
    mut reader: MessageReader<SoilMessage>,
    mut soil_set: ResMut<SoilSet>,
    soil_assets: Res<SoilAssets>,
    mut sprite_query: Query<&mut Sprite, With<Soil>>,
) {
    let mut dirty_positions = std::collections::HashSet::new();

    for message in reader.read() {
        let world_pos = pos::tile_to_world(message.pos);

        match message.ctrl {
            SoilCtrl::Add => {
                if !soil_set.is_soil(message.pos) {
                    let entity = commands
                        .spawn((
                            Sprite::from_atlas_image(
                                soil_assets.texture.clone(),
                                TextureAtlas {
                                    layout: soil_assets.layout.clone(),
                                    index: 0,
                                },
                            ),
                            Transform::from_xyz(world_pos.x, world_pos.y, 5.0),
                            Soil,
                        ))
                        .id();

                    soil_set.insert(message.pos, entity);

                    dirty_positions.insert(message.pos);

                    for (offset, _) in NEIGHBORS {
                        dirty_positions.insert(message.pos + offset);
                    }
                }
            }
            SoilCtrl::ReMove => {
                if soil_set.is_soil(message.pos) {
                    let option = soil_set.get(message.pos);
                    if let Some(entity) = option {
                        commands.entity(entity).despawn();
                        soil_set.remove(message.pos);

                        for (offset, _) in NEIGHBORS {
                            dirty_positions.insert(message.pos + offset);
                        }
                    }
                }
            }
        }
    }

    for pos in dirty_positions {
        if let Some(entity) = soil_set.get(pos) {
            let mask = soil_set.calculate_mask(pos);
            let frame_index = soil_assets.mask_to_index[mask];

            if let Ok(mut sprite) = sprite_query.get_mut(entity)
                && let Some(atlas) = &mut sprite.texture_atlas
            {
                atlas.index = frame_index;
            }
        }
    }
}
