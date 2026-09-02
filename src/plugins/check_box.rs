use bevy::prelude::*;

use crate::plugins::camera::MainCamera;

use crate::plugins::farming::soil;
use crate::utils::pos;

#[derive(Component)]
pub struct CheckBox;

#[derive(Resource, Default)]
pub struct CheckBoxPos {
    pub inate: IVec2,
}

pub struct CheckBoxPlugin;

impl Plugin for CheckBoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CheckBoxPos>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, check);
        app.add_systems(Update, click_check_box);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::linear_rgba(0.8, 0.6, 0.6, 0.4),
            custom_size: Some(Vec2 { x: 16.0, y: 16.0 }),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 8.0),
        CheckBox,
    ));
}

pub fn check(
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut check_box_tf: Query<&mut Transform, With<CheckBox>>,
    mut check_box_coord: ResMut<CheckBoxPos>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    let Ok((camera, camera_trnsform)) = camera.single() else {
        return;
    };

    let Ok(mut transform) = check_box_tf.single_mut() else {
        return;
    };

    let cursor_pos = window.cursor_position().unwrap_or_default();

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_trnsform, cursor_pos) else {
        return;
    };

    let tile_pos_vec = pos::world_to_tile(world_pos);

    let new_world_pos = pos::tile_to_world(tile_pos_vec);

    transform.translation.x = new_world_pos.x;
    transform.translation.y = new_world_pos.y;

    check_box_coord.inate = tile_pos_vec;
}

pub fn click_check_box(
    mouse: Res<ButtonInput<MouseButton>>,
    check_box_pos: Res<CheckBoxPos>,
    mut soil_writer: MessageWriter<soil::SoilMessage>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    soil_writer.write(soil::SoilMessage {
        ctrl: soil::SoilCtrl::Add,
        pos: check_box_pos.inate,
    });
}
