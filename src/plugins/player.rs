use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_player);
        app.add_systems(Update, update_player_animation_state);
        app.add_systems(Update, animate_player_sprite);
    }
}

#[derive(Component, Default, PartialEq, Clone, Copy)]
enum Direction {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

#[derive(Component, Default, PartialEq, Clone, Copy)]
enum AnimationState {
    #[default]
    Idle,
    Walking,
}

#[derive(Component)]
struct AnimationConfig {
    first_index: usize,
    last_index: usize,
    time: Timer,
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

fn setup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = assets_server.load("角色/m.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 24), 14, 1, None, None);
    let atlas_layout = texture_atlas_layout.add(layout);

    let anim_config = AnimationConfig::new(0, 1, 4.0);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: atlas_layout,
                index: 0,
            },
        ),
        Transform::from_xyz(50.0, 50.0, 4.0),
        Player,
        // 碰撞
        RigidBody::Dynamic,
        Collider::ball(6.0),
        LockedAxes::ROTATION_LOCKED,
        Friction::coefficient(0.0),
        Velocity::default(),
        Ccd::enabled(),
        GravityScale(0.0),
        // 动画
        Direction::Down,
        AnimationState::Idle,
        anim_config,
    ));
}

fn move_player(mut query: Query<&mut Velocity, With<Player>>, input: Res<ButtonInput<KeyCode>>) {
    let mut dir = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }
    let dir = dir.normalize_or_zero();
    for mut vel in &mut query {
        vel.linear = dir * 120.0;
    }
}

fn update_player_animation_state(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Direction,
            &mut AnimationState,
            &mut AnimationConfig,
            &mut Sprite,
        ),
        With<Player>,
    >,
) {
    for (mut dir, mut state, mut config, mut sprite) in query.iter_mut() {
        let mut move_vec = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) {
            move_vec.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            move_vec.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            move_vec.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            move_vec.x += 1.0;
        }

        let old_state = *state;
        let old_dir = *dir;

        if move_vec == Vec2::ZERO {
            *state = AnimationState::Idle;
        } else {
            *state = AnimationState::Walking;
            if move_vec.y > 0.0 {
                *dir = Direction::Up;
            } else if move_vec.y < 0.0 {
                *dir = Direction::Down;
            } else if move_vec.x < 0.0 {
                *dir = Direction::Left;
            } else if move_vec.x > 0.0 {
                *dir = Direction::Right;
            }
        }

        if old_state != *state || old_dir != *dir {
            let (first, last, fps) = match (*state, *dir) {
                (AnimationState::Idle, _) => (0, 1, 2.0),
                (AnimationState::Walking, Direction::Up) => (10, 13, 12.0),
                (AnimationState::Walking, Direction::Down) => (2, 5, 12.0),
                (AnimationState::Walking, Direction::Left) => (6, 9, 12.0),
                (AnimationState::Walking, Direction::Right) => (2, 5, 12.0),
            };

            config.first_index = first;
            config.last_index = last;
            config.time = Timer::from_seconds(1.0 / fps, TimerMode::Repeating);

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = config.first_index;
            }
        }
    }
}

fn animate_player_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Player>>,
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
