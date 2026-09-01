use bevy::prelude::*;
use bevy_firefly::prelude::*;

#[derive(Component)]
pub struct SunHight;

pub struct SunHightPlugin;

impl Plugin for SunHightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FireflyPlugin);
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        PointLight2d {
            color: Color::srgb(1.0, 0.9, 0.7),
            intensity: 10.0,
            radius: 2000.0,
            ..Default::default()
        },
        Transform::from_xyz(1200.0, 1200.0, 10001.0),
        SunHight,
    ));

    info!("创建世界光照");
}
