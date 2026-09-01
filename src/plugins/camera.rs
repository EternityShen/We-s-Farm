use bevy::prelude::*;
use bevy_firefly::prelude::*;

use crate::plugins::player::Player;

#[derive(Component)]
pub struct MainCamera;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, follow_player);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(120.0, 120.0, 1000.0),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.3,
            ..OrthographicProjection::default_2d()
        }),
        FireflyConfig::default(),
        MainCamera,
    ));

    info!("创建世界主相机");
}

// fn move_camera(
//     time: Res<Time>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<MainCamera>>,
// ) {
//     let Ok(mut transform) = query.single_mut() else {
//         return;
//     };
//
//     let mut direction = Vec3::ZERO;
//
//     let speed = if keyboard_input.pressed(KeyCode::ControlLeft) {
//         800.0
//     } else {
//         400.0
//     };
//
//     if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
//         direction.y += 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
//         direction.y -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
//         direction.x -= 1.0;
//     }
//     if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
//         direction.x += 1.0;
//     }
//
//     if direction.length_squared() > 0.0 {
//         transform.translation += direction.normalize() * speed * time.delta_secs();
//     }
// }

fn follow_player(
    query_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut query_camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(mut transform_camera) = query_camera.single_mut() else {
        return;
    };

    let Ok(transform_player) = query_player.single() else {
        return;
    };

    let player_x = transform_player.translation.x;
    let player_y = transform_player.translation.y;

    transform_camera.translation.x = player_x;
    transform_camera.translation.y = player_y;
}
