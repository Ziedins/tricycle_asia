use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
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
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    },
    animation_indicies,
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}


fn animate_sprite(
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
