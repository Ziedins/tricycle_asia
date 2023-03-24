use std::time;

use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const PLAYER_SPAWN_X: f32 = 100. -BOUNDS.x / 2.;
const GROUND_Y: f32 = 50. -BOUNDS.y / 2.;
const PLAYER_SIZE: Vec2 = Vec2::new(140., 90.);
const GROUND_SIZE: Vec2 = Vec2::new(2700., 30.);


fn main() {
    println!("{PLAYER_SPAWN_X}, {GROUND_Y}");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_sprite_system)
        .add_system(move_ground_system)
        .add_system(gravity_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct Ground {
    movement_speed : f32
}

#[derive(Component)]
struct Player {
    y: f32
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {
    let texture_handle = asset_server.load("textures/chars/tricycle-idle-move.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(75.0, 45.0), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indicies = AnimationIndices {first: 0, last:5};

    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(animation_indicies.first),
        transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(PLAYER_SPAWN_X, 0., 1.)),
        ..default()
    },
        animation_indicies,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {
            y: 0.
        }
    ));

    let ground_texture_handle = asset_server.load("textures/objects/ground_long.png");
    commands.spawn((
            SpriteBundle {
                texture: ground_texture_handle,
                transform: Transform::from_scale(Vec3::splat(3.)).with_translation(Vec3::new(0., GROUND_Y, 0.)),
                ..default()
            },
            Ground {
                movement_speed: 300.0
            }
            ));
}


fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite
    )>) {
    for (indicies, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indicies.last {
                indicies.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn move_ground_system(time: Res<Time> ,mut query: Query<(
&Ground, &mut Transform
)>) {
    let (ground, mut transform) = query.single_mut();
    transform.translation.x -= ground.movement_speed * time.delta_seconds();

}

fn gravity_system(
    time: Res<Time>,
    mut player_query: Query<(&Player, &mut Transform)>,
    ground_query: Query<&Transform, (With<Ground>, Without<Player>)>
) {
    let (player, mut player_transform) = player_query.single_mut();
    let ground_transform = ground_query.single();
    let collision = collide(
        player_transform.translation,
        PLAYER_SIZE,
        ground_transform.translation,
        GROUND_SIZE
    );

    if let Some(collision) = collision {

    } else {
        player_transform.translation.y -= 500. * time.delta_seconds();
    }

}
