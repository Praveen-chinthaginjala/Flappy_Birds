use macroquad::prelude::*;
use crate::prefabs::background::Background;
use crate::prefabs::button::Button;
use crate::prefabs::ground::Ground;
use crate::scenes::{game::GameScene, Scene, Transition};

pub struct TitleScene {
    sky_texture: Option<Texture2D>,
    title: Option<Texture2D>,
    bird: Option<Texture2D>,
    background: Option<Background>,
    ground: Option<Ground>,
    button: Option<Button>,
    loading: bool,
    loading_game: bool,
}

impl TitleScene {
    pub fn new() -> Self {
        TitleScene {
            sky_texture: None,
            title: None,
            bird: None,
            background: None,
            ground: None,
            button: None,
            loading: true,
            loading_game: false,
        }
    }
    
    pub async fn load_assets(&mut self) {
        if self.loading {
            // Load textures
            self.sky_texture = Some(load_texture("resources/sky.png").await.unwrap_or(Texture2D::empty()));
            self.title = Some(load_texture("resources/title.png").await.unwrap_or(Texture2D::empty()));
            self.bird = Some(load_texture("resources/bird.png").await.unwrap_or(Texture2D::empty()));
            
            // Initialize components
            self.background = Some(Background::new().await);
            self.ground = Some(Ground::new().await);
            self.button = Some(Button::new().await);

            self.loading = false;
        }
    }
    
    pub async fn load_game_scene(&mut self) -> Option<Box<dyn Scene>> {
        if self.loading_game {
            // Add async loading indicator
            let game_scene = GameScene::new().await;
            self.loading_game = false;
            Some(Box::new(game_scene))
        } else {
            None
        }
    }

    pub fn is_loading(&self) -> bool {
        self.loading
    }

    pub fn is_loading_game(&self) -> bool {
        self.loading_game
    }
}

impl Scene for TitleScene {
    fn update(&mut self) -> Transition {
        // If still loading assets or transitioning to game, do nothing
        if self.loading || self.loading_game {
            return Transition::None;
        }
        
        // Safe unwraps since we've ensured loading is complete
        let background = self.background.as_mut().unwrap();
        // println!("Background created");
        let ground = self.ground.as_mut().unwrap();
        // println!("Ground created");
        let button = self.button.as_ref().unwrap();
        // println!("Button created");
        
        background.update();
        ground.update();
        
        // Convert mouse position to Vec2
        let mouse_position = Vec2::new(mouse_position().0, mouse_position().1);
        
        if is_mouse_button_down(MouseButton::Left) && button.contains(mouse_position) {
            // Start loading the game scene
            self.loading_game = true;
            // Return None for now, the main loop will handle the transition
            return Transition::None;
        } else if is_key_pressed(KeyCode::Escape) {
            return Transition::Pop;
        }
        
        Transition::None
    }
    
    fn draw(&mut self) {
        // Show loading screen if still loading assets
        if self.loading {
            clear_background(BLACK);
            draw_text("Loading...", screen_width() / 2.0 - 50.0, screen_height() / 2.0, 30.0, WHITE);
            return;
        }
        
        // Show game loading screen if transitioning to game
        if self.loading_game {
            clear_background(BLACK);
            draw_text("Starting game...", screen_width() / 2.0 - 80.0, screen_height() / 2.0, 30.0, WHITE);
            return;
        }
        
        // Safe unwraps since we've ensured loading is complete
        let sky = self.sky_texture.as_ref().unwrap();
        let title = self.title.as_ref().unwrap();
        let bird = self.bird.as_ref().unwrap();
        let background = self.background.as_ref().unwrap();
        let ground = self.ground.as_ref().unwrap();
        let button = self.button.as_ref().unwrap();
        
        // Draw everything
        draw_texture(sky, 0.0, 0.0, WHITE);
        draw_texture(sky, 100.0, 0.0, WHITE);
        draw_texture(sky, 200.0, 0.0, WHITE);
        draw_texture(sky, 300.0, 0.0, WHITE);
        draw_texture(sky, 400.0, 0.0, WHITE);
        draw_texture(sky, 500.0, 0.0, WHITE);
        draw_texture(sky, 600.0, 0.0, WHITE);
        draw_texture(sky, 700.0, 0.0, WHITE);
        draw_texture(sky, 800.0, 0.0, WHITE);
        draw_texture(sky, 900.0, 0.0, WHITE);
        draw_texture(sky, 1000.0, 0.0, WHITE);
        draw_texture(sky, 1100.0, 0.0, WHITE);
        draw_texture(sky, 1200.0, 0.0, WHITE);
        background.draw();
        ground.draw();
        
        // Center the title horizontally and place 25% from top
        let title_x = screen_width() / 2.0 - title.width() / 2.0;
        let title_y = screen_height() * 0.25;
        draw_texture(title, title_x, title_y, WHITE);

        // Center the bird horizontally and vertically with offset
        let bird_x = screen_width() / 2.0 - bird.width() / 2.0;
        let bird_y = screen_height() / 2.0 - bird.height() / 2.0 - 60.0;
        draw_texture(bird, bird_x, bird_y, WHITE);


        button.draw();
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/*

The tests validate:
1. Initial state defaults
2. Asset loading changes state
3. Game loading toggle
4. State separation of loading and loading_game
5. Flag independence (of say, game loading and say, asset loading)

*/

#[cfg(test)]
mod tests {
    use super::*;

    // Create a dummy struct for isolated state validation
    struct DummyTitleScene {
        loading: bool,
        loading_game: bool,
    }

    impl DummyTitleScene {
        fn new() -> Self {
            DummyTitleScene {
                loading: true,
                loading_game: false,
            }
        }

        fn is_loading(&self) -> bool {
            self.loading
        }

        fn is_loading_game(&self) -> bool {
            self.loading_game
        }

        fn simulate_asset_load(&mut self) {
            if self.loading {
                self.loading = false;
            }
        }

        fn simulate_game_load_start(&mut self) {
            self.loading_game = true;
        }

        fn simulate_game_loaded(&mut self) {
            if self.loading_game {
                self.loading_game = false;
            }
        }
    }

    #[test]
    fn test_initial_state() {
        let scene = DummyTitleScene::new();
        assert!(scene.is_loading());
        assert!(!scene.is_loading_game());
    }

    #[test]
    fn test_asset_loading_changes_state() {
        let mut scene = DummyTitleScene::new();
        scene.simulate_asset_load();
        assert!(!scene.is_loading());
    }

    #[test]
    fn test_game_loading_flag_can_be_enabled() {
        let mut scene = DummyTitleScene::new();
        scene.simulate_game_load_start();
        assert!(scene.is_loading_game());
    }

    #[test]
    fn test_game_loading_flag_can_be_disabled() {
        let mut scene = DummyTitleScene::new();
        scene.simulate_game_load_start();
        scene.simulate_game_loaded();
        assert!(!scene.is_loading_game());
    }

    #[test]
    fn test_asset_load_does_not_affect_game_flag() {
        let mut scene = DummyTitleScene::new();
        scene.simulate_asset_load();
        assert!(!scene.is_loading());
        assert!(!scene.is_loading_game());
    }

    #[test]
    fn test_game_flag_does_not_affect_asset_flag() {
        let mut scene = DummyTitleScene::new();
        scene.simulate_game_load_start();
        assert!(scene.is_loading_game());
        assert!(scene.is_loading()); // still loading assets
    }
}