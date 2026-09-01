use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{
    dynamics::RigidBody, geometry::Collider, picking_backend::RapierPickingPlugin,
    plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin);
        app.add_plugins(RapierPhysicsPlugin::<()>::pixels_per_meter(16.0));
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_plugins(RapierPickingPlugin);
        app.insert_resource(LevelSelection::index(0));
        app.add_systems(Startup, setup);
        app.add_systems(Update, build_col_map);
    }
}

fn setup(mut commands: Commands, asset_servet: Res<AssetServer>) {
    let ldtk_handle = asset_servet.load("MapLDtk/farm.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: ldtk_handle.into(),
        ..Default::default()
    });

    info!("创建世界地图");
}

fn build_col_map(mut commands: Commands, query: Query<(Entity, &IntGridCell), Added<IntGridCell>>) {
    for (entity, cell) in query {
        if cell.value == 1 {
            commands
                .entity(entity)
                .insert((Collider::cuboid(8.0, 8.0), RigidBody::Fixed));
        }
    }
}
