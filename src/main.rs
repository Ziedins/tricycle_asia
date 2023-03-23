use std::path::PathBuf;

use rusty_engine::prelude::*;
use rand::prelude::*;

const PLAYER_SPRITE_1: &str = "sprite/racing/tricycle-1.png";
const PLAYER_SPRITE_2: &str = "sprite/racing/tricycle-2.png";
const PLAYER_SPRITE_3: &str = "sprite/racing/tricycle-3.png";

#[derive(Debug)]
enum PlayerImage {
    Image1,
    Image2,
    Image3
}

#[derive(Debug)]
struct GameState {
    score: u32,
    spawn_timer: Timer,
    player_image: PlayerImage
}

fn main() {
    let mut game = Game::new();

    let game_state = GameState {
        score: 0,
        spawn_timer: Timer::from_seconds(0.00, false),
        player_image: PlayerImage::Image1
    };

    let player = game.add_sprite("player", PLAYER_SPRITE_1);
    player.translation.x = -500.0;
    player.translation.y = -200.0;
    player.collision = true;
    player.scale = 1.0;

    game.add_logic(game_logic);

    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let player = engine.sprites.get_mut("player").unwrap();

    println!("player_image : {:?}", game_state.player_image);
    match game_state.player_image {
        PlayerImage::Image1 => {
            player.filepath = PathBuf::from(PLAYER_SPRITE_2);
            player.translation.x = -400.0;
            player.scale = 1.1;
            game_state.player_image = PlayerImage::Image2;
        }
        PlayerImage::Image2 => {
            player.filepath = PathBuf::from(PLAYER_SPRITE_3);
            player.translation.x = -300.0;
            game_state.player_image = PlayerImage::Image3;
        }
        PlayerImage::Image3 => {
            player.filepath = PathBuf::from(PLAYER_SPRITE_1);
            player.translation.x = -200.0;
            game_state.player_image = PlayerImage::Image1;
        }
    }
    println!("player: {:?}", player);
}
