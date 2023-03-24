use std::time;

use bevy::prelude::*;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const GROUND_Y: f32 = 350. - BOUNDS.y;

fn main() {
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

    let horizontal_margin = BOUNDS.x / 4.0;
    let animation_indicies = AnimationIndices {first: 0, last:5};

    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(animation_indicies.first),
        transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(-horizontal_margin, 100. -horizontal_margin, 1.)),
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
                movement_speed: 500.0
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
    transform.translation.x -= 150. * time.delta_seconds();

}

fn gravity_system(
    time: Res<Time>,
    mut player_query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = player_query.single_mut();
    if transform.translation.y > GROUND_Y + 150.0 {
        transform.translation.y -= 500. * time.delta_seconds();
    }

}
