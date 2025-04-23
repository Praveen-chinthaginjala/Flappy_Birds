use macroquad::prelude::*;

pub struct Button {
    texture: Texture2D,
}

impl Button {
    pub async fn new() -> Self {
        let texture = load_texture("./resources/start-button.png")
            .await
            .expect("Could not load button texture");
        
        Button { texture }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let button_rect = Rect::new(
            screen_center.x - self.texture.width() / 2.0,
            screen_center.y - self.texture.height() / 2.0,
            self.texture.width(),
            self.texture.height(),
        );
        button_rect.contains(point)
    }

    pub fn draw(&self) {
        let x = screen_width() / 2.0 - self.texture.width() / 2.0;
        let y = screen_height() / 2.0 - self.texture.height() / 2.0;
        draw_texture(&self.texture, x, y, WHITE);
    }
}

/*

The tests validate :
1. Button hitbox detection
2. Centered positioning logic
3. Edge checks and out-of-bounds checks 

*/

#[cfg(test)]
mod tests {
    use super::*;

    // Wrap the Button logic into a testable version that can take a texture directly
    struct TestableButton {
        texture_width: f32,
        texture_height: f32,
        screen_width: f32,
        screen_height: f32,
    }

    impl TestableButton {
        fn new(texture_width: f32, texture_height: f32, screen_width: f32, screen_height: f32) -> Self {
            Self {
                texture_width,
                texture_height,
                screen_width,
                screen_height,
            }
        }

        fn contains(&self, point: Vec2) -> bool {
            let screen_center = vec2(self.screen_width / 2.0, self.screen_height / 2.0);
            let button_rect = Rect::new(
                screen_center.x - self.texture_width / 2.0,
                screen_center.y - self.texture_height / 2.0,
                self.texture_width,
                self.texture_height,
            );
            button_rect.contains(point)
        }
    }

    #[test]
    fn test_point_inside_button() {
        let button = TestableButton::new(100.0, 50.0, 800.0, 600.0);
        let inside_point = vec2(400.0, 300.0); // center of screen
        assert!(button.contains(inside_point));
    }

    #[test]
    fn test_point_outside_button() {
        let button = TestableButton::new(100.0, 50.0, 800.0, 600.0);
        let outside_point = vec2(10.0, 10.0);
        assert!(!button.contains(outside_point));
    }

    #[test]
    fn test_point_on_edge_of_button() {
        let button = TestableButton::new(100.0, 50.0, 800.0, 600.0);
        let edge_point = vec2(400.0 + 50.0, 300.0); // right edge
        assert!(!button.contains(edge_point)); // Rect::contains is exclusive on the right edge
    }
}
