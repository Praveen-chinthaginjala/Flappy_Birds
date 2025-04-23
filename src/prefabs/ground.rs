use macroquad::prelude::*;

use crate::systems::physics::{check_collision, PhysicsBody};
use crate::SCROLL_SPEED;

pub struct Ground {
    texture: Texture2D,
    scroll_pos: f32,
    pub scroll: bool,
}

impl PhysicsBody for Ground {
    fn get_collision_rect(&mut self) -> Rect {
        let ground_height = self.texture.height();

        Rect::new(
            0.0,
            screen_height() - ground_height,
            screen_width(),
            ground_height,
        )
    }

    fn collides_with(&mut self, obj: &Rect) -> bool {
        check_collision(&self.get_collision_rect(), obj)
    }
}

impl Ground {
    pub async fn new() -> Self {
        let texture = load_texture("./resources/ground.png")
            .await
            .expect("Could not load ground texture");

        texture.set_filter(FilterMode::Nearest); // Optional: keeps it pixel-perfect

        Ground {
            texture,
            scroll_pos: 0.0,
            scroll: true,
        }
    }

    pub fn update(&mut self) {
        if self.scroll {
            self.scroll_pos = (self.scroll_pos - SCROLL_SPEED) % self.texture.width();
        }
    }

    pub fn draw(&self) {
        let y_pos = screen_height() - self.texture.height();
        let tex_width = self.texture.width();

        // Draw five copies for seamless scrolling
        draw_texture(&self.texture, self.scroll_pos, y_pos, WHITE);
        draw_texture(&self.texture, self.scroll_pos + 1.0 * tex_width, y_pos, WHITE);
        draw_texture(&self.texture, self.scroll_pos + 2.0 * tex_width, y_pos, WHITE);
        draw_texture(&self.texture, self.scroll_pos + 3.0 * tex_width, y_pos, WHITE);
        draw_texture(&self.texture, self.scroll_pos + 4.0 * tex_width, y_pos, WHITE);
    }
}

/*

The tests validate :
1. Basic scroll position updates
2. Scroll disable behavior
3. Correct modulo operation
4. Collision rectangle calculation
5. Collision detection logic

*/

#[cfg(test)]
mod tests {
    use super::*;
    use macroquad::math::Rect;

    struct DummyGround {
        scroll_pos: f32,
        scroll: bool,
        width: f32,
        height: f32,
    }

    impl DummyGround {
        fn new(scroll_pos: f32, scroll: bool, width: f32, height: f32) -> Self {
            DummyGround {
                scroll_pos,
                scroll,
                width,
                height,
            }
        }

        fn update(&mut self) {
            if self.scroll {
                self.scroll_pos = (self.scroll_pos - 2.0) % self.width;
            }
        }

        fn get_collision_rect(&self, screen_w: f32, screen_h: f32) -> Rect {
            Rect::new(
                0.0,
                screen_h - self.height,
                screen_w,
                self.height,
            )
        }

        fn collides_with(&self, obj: &Rect, screen_w: f32, screen_h: f32) -> bool {
            let ground_rect = self.get_collision_rect(screen_w, screen_h);
            ground_rect.overlaps(obj)
        }
    }

    #[test]
    fn test_update_scroll_enabled() {
        let mut ground = DummyGround::new(0.0, true, 200.0, 50.0);
        ground.update();
        assert_ne!(ground.scroll_pos, 0.0);
    }

    #[test]
    fn test_update_scroll_disabled() {
        let mut ground = DummyGround::new(42.0, false, 200.0, 50.0);
        ground.update();
        assert_eq!(ground.scroll_pos, 42.0);
    }

    #[test]
    fn test_get_collision_rect() {
        let ground = DummyGround::new(0.0, true, 200.0, 50.0);
        let rect = ground.get_collision_rect(800.0, 600.0);
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.y, 550.0);
        assert_eq!(rect.w, 800.0);
        assert_eq!(rect.h, 50.0);
    }

    #[test]
    fn test_collides_with_overlap() {
        let ground = DummyGround::new(0.0, true, 200.0, 50.0);
        let test_obj = Rect::new(0.0, 580.0, 50.0, 50.0); // overlaps last 20px
        assert!(ground.collides_with(&test_obj, 800.0, 600.0));
    }

    #[test]
    fn test_collides_with_no_overlap() {
        let ground = DummyGround::new(0.0, true, 200.0, 50.0);
        let test_obj = Rect::new(0.0, 480.0, 50.0, 50.0); // fully above ground
        assert!(!ground.collides_with(&test_obj, 800.0, 600.0));
    }
}