use bevy::prelude::*;

use we_s_fram::plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(plugins::world::WorldPlugin)
        .add_plugins(plugins::camera::MainCameraPlugin)
        .add_plugins(plugins::sunhight::SunHightPlugin)
        .add_plugins(plugins::player::PlayerPlugin)
        .run();
}
