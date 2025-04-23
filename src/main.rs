use macroquad::prelude::*;
use crate::systems::scenemanagement::SceneManager;

mod scenes;
mod prefabs;
mod systems;

pub const GRAVITY: f32 = 9.1;
pub const SCROLL_SPEED: f32 = 3.0;
pub const FILE_NAME: &str = "highscore.txt";

// Summary - main() :
// 1. Create scene manager.
// 2. Enter game loop:
//     - Run pre-update to load assets or switch scenes.
//     - Update game logic and handle scene transitions.
//     - Clear screen and draw current scene.
//     - Wait for next frame.
#[macroquad::main("Flappy Bird")]
async fn main() {
    let mut scene_manager = SceneManager::new();

    loop {
        // Load assets or transition to new scene if current scene is a TitleScene
        scene_manager.pre_update().await;

        // Update current scene (handle transitions)
        scene_manager.update();

        // Clear and draw
        clear_background(BLACK);
        scene_manager.draw();

        next_frame().await;
    }
}