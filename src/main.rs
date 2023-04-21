use std::collections::LinkedList;
use rand::prelude::*;
use bevy::{prelude::{*, system_adapter::new}, sprite::collide_aabb::{collide, Collision}, time::Stopwatch};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const PLAYER_SPAWN_X: f32 = 100. -BOUNDS.x / 2.;
const GROUND_Y: f32 = -BOUNDS.y / 2.;
const PLAYER_SIZE: Vec2 = Vec2::new(140., 90.);
const GROUND_SIZE: Vec2 = Vec2::new(2700., 30.);
const GRAVITY: f32 = 500.;

#[derive(Resource, Debug)]
struct GameState {
    ground_list: LinkedList<Entity>,
    enemy_list: LinkedList<Entity>
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct Ground {
    movement_speed : f32,
    length: f32,
}

#[derive(Component)]
struct Enemy {
    movement_speed : f32,
    length: f32
}

#[derive(Component)]
struct Player {
    is_jumping : bool,
    on_ground: bool,
    jump_duration: Stopwatch
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(GameState {
           ground_list: LinkedList::new(),
           enemy_list: LinkedList::new()
        })
        .add_system(animate_sprite_system)
        .add_system(move_ground_system)
        .add_system(move_enemies_system)
        .add_system(gravity_system)
        .add_system(jump_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
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
            is_jumping : false,
            on_ground: false,
            jump_duration: Stopwatch::new()
        }
    ));

    let ground_texture_handle = asset_server.load("textures/objects/ground_long.png");
    let ground_id = commands.spawn((
            SpriteBundle {
                texture: ground_texture_handle,
                transform: Transform::from_scale(Vec3::splat(3.)).with_translation(Vec3::new(0., GROUND_Y, 0.)),
                ..default()
            },
            Ground {
                movement_speed: 300.0,
                length: 900. * 3.
            }
            )).id();
    game_state.ground_list.push_front(ground_id);

    let enemy_texture_handle = asset_server.load("textures/chars/trash-animated.png");
    let enemy_texture_atlas = TextureAtlas::from_grid(enemy_texture_handle, Vec2::new(70., 70.), 6, 1, None, None);
    let enemy_texture_handle = texture_atlases.add(enemy_texture_atlas);
    let animation_indicies = AnimationIndices {first: 0, last:5};
    let enemy_id = commands.spawn((
            SpriteSheetBundle {
                texture_atlas: enemy_texture_handle,
                sprite: TextureAtlasSprite::new(animation_indicies.first),
                transform: Transform::from_scale(Vec3::splat(1.5)).with_translation(Vec3::new(BOUNDS.x, GROUND_Y + 45., 1.)),
                ..default()
            },
        animation_indicies,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Enemy {
            movement_speed: 300.0,
            length: 70. * 1.5
        }
    )).id();
    game_state.enemy_list.push_front(enemy_id);
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

fn move_ground_system(time: Res<Time>,mut game_state : ResMut<GameState>,mut commands: Commands,  mut query: Query<(Entity,
&Ground,
&mut Transform,
&Handle<Image>
)>) {
    query.for_each_mut(|(ground_entity, ground, mut transform, texture_handle)| {
        transform.translation.x -= ground.movement_speed * time.delta_seconds();
        let transform_end_x = transform.translation.x + ground.length;
        let ground_spawn_x = BOUNDS.x * 2.;

        //Remove Ground when it's off screent
        if transform_end_x < 0. {
            game_state.ground_list.pop_front();
            commands.entity(ground_entity).despawn();
        }

        //Spawn new Ground after current ground
        if transform_end_x < ground_spawn_x && game_state.ground_list.len() < 2 {
            let ground_id = commands.spawn((
                    SpriteBundle {
                        texture: texture_handle.clone(),
                        transform: Transform::from_scale(Vec3::splat(3.)).with_translation(Vec3::new(ground_spawn_x, GROUND_Y, 0.)),
                        ..default()
                    },
                    Ground {
                        movement_speed: 300.0,
                        length: 900. * 3.
                    }
            )).id();
            game_state.ground_list.push_front(ground_id);
         }
    });

}

fn move_enemies_system(time: Res<Time>,mut game_state : ResMut<GameState>,mut commands: Commands,
mut query: Query<(Entity,
    &Enemy,
    &mut Transform,
    )>
) {

    query.for_each_mut(|(enemy_entity, enemy, mut transform)| {
        let transform_end_x = transform.translation.x + enemy.length + BOUNDS.x / 2.;
        if transform_end_x < 0. {
            commands.entity(enemy_entity).despawn();
            game_state.ground_list.pop_front();
        }

        transform.translation.x -= enemy.movement_speed * time.delta_seconds();
    });
}


fn gravity_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    ground_query: Query<&Transform, (With<Ground>, Without<Player>)>
) {
    let (mut player, mut player_transform) = player_query.single_mut();

    let mut collisions:Vec<Option<Collision>> = Vec::new();
    ground_query.for_each(|ground_transform|{
        collisions.push(collide(
            player_transform.translation,
            PLAYER_SIZE,
            ground_transform.translation,
            GROUND_SIZE
        ));
    });

    let mut has_collided: bool = false;

    //If there's any player to ground collisions has_collided = true;
    for collision in collisions {
        if collision != None {
            has_collided = true;
        }
    }

    //If has_collided is false, player falls
    if !has_collided && !player.is_jumping {
        player_transform.translation.y -= GRAVITY * time.delta_seconds();
        player.on_ground = false;
    } else {
        player.on_ground = true;
    }
}

fn jump_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    player.jump_duration.tick(time.delta());

    if keyboard_input.just_pressed(KeyCode::Space) && player.on_ground {
        player.is_jumping = true;
        player.on_ground = false;
        player.jump_duration.reset();
    }

    if player.jump_duration.elapsed_secs() < 0.3 && player.is_jumping {
        player_transform.translation.y += 700. * time.delta_seconds();
    } else if player.jump_duration.elapsed_secs() < 0.4 {
        player_transform.translation.y += 100. * time.delta_seconds();
    }
    else {
        player.is_jumping = false;
    }
}


