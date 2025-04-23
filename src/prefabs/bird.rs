use macroquad::prelude::*;
use crate::systems::physics::{check_collision, PhysicsBody};
use crate::GRAVITY;

pub struct Bird {
    textures: Vec<Texture2D>,
    current_frame: usize,
    frame_timer: f32,
    frame_duration: f32,
    velocity: Vec2,
    pub position: Vec2,
    pub allow_gravity: bool,
    pub alive: bool,
    pub fixed_x_position: f32,
}

impl PhysicsBody for Bird {
    fn get_collision_rect(&mut self) -> Rect {
        Rect::new(self.position.x, self.position.y, 34.0, 24.0)
    }

    fn collides_with(&mut self, obj: &Rect) -> bool {
        check_collision(&self.get_collision_rect(), obj)
    }
}

impl Bird {
    pub async fn new() -> Self {
        let texture = load_texture("./resources/bird.png").await.unwrap();
        let texture_data = texture.get_texture_data();
        let mut textures = Vec::new();
        
        for i in 0..3 {
            let sub_image = texture_data.sub_image(Rect::new(
                i as f32 * 34.0,
                0.0,
                34.0,
                24.0
            ));
            textures.push(Texture2D::from_image(&sub_image));
        }

        let fixed_x = screen_width() / 2.5;

        Bird {
            textures,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.1,
            velocity: Vec2::ZERO,
            position: vec2(fixed_x, screen_height() / 2.0),
            allow_gravity: false,
            alive: true,
            fixed_x_position: fixed_x,
        }
    }

    pub fn flap(&mut self) {
        if self.alive {
            self.velocity.y = -6.5;
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
        self.velocity = Vec2::ZERO;
    }

    pub fn reset(&mut self) {
        self.position = vec2(self.fixed_x_position, screen_height() / 2.0);
        self.velocity = Vec2::ZERO;
        self.alive = true;
    }

    pub fn update(&mut self) {
        self.frame_timer += get_frame_time();
        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            if self.alive {
                self.current_frame = (self.current_frame + 1) % self.textures.len();
            }
        }

        if self.allow_gravity {
            self.velocity.y += GRAVITY / 30.0;
            self.position.y += self.velocity.y;
            
            // Keep bird within vertical bounds
            let min_y = 12.0;
            let max_y = screen_height() - 36.0;
            self.position.y = self.position.y.clamp(min_y, max_y);
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.textures[self.current_frame],
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                pivot: Some(vec2(17.0, 12.0)),
                ..Default::default()
            },
        );
    }
}

/* 

The tests validate : 
1. Flap impulse application
2. Kill state handling
3. Collision rectangle calculation
4. Collision detection logic

*/

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;
    use macroquad::prelude::Rect;

    // Test helper to create Bird instance without Macroquad dependencies
    fn test_bird() -> Bird {
        Bird {
            textures: vec![],
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.1,
            velocity: Vec2::ZERO,
            position: Vec2::new(100.0, 300.0),
            allow_gravity: false,
            alive: true,
            fixed_x_position: 100.0,
        }
    }

    #[test]
    fn test_flap_impulse() {
        let mut bird = test_bird();
        bird.flap();
        assert_float_eq!(bird.velocity.y, -6.5, abs <= 0.001);
    }

    #[test]
    fn test_kill_resets_velocity() {
        let mut bird = test_bird();
        bird.velocity.y = -5.0;
        bird.kill();
        assert!(!bird.alive);
        assert_float_eq!(bird.velocity.y, 0.0, abs <= 0.001);
    }

    #[test]
    fn test_collision_rect_calculation() {
        let mut bird = test_bird();
        bird.position = Vec2::new(50.0, 75.0);
        let rect = bird.get_collision_rect();
        
        assert_float_eq!(rect.x, 50.0, abs <= 0.001);
        assert_float_eq!(rect.y, 75.0, abs <= 0.001);
        assert_float_eq!(rect.w, 34.0, abs <= 0.001);
        assert_float_eq!(rect.h, 24.0, abs <= 0.001);
    }

    #[test]
    fn test_collision_detection() {
        let mut bird = test_bird();
        bird.position = Vec2::new(100.0, 100.0);
        let obstacle = Rect::new(110.0, 110.0, 20.0, 20.0);
        
        assert!(bird.collides_with(&obstacle));
        
        let distant_obstacle = Rect::new(200.0, 200.0, 20.0, 20.0);
        assert!(!bird.collides_with(&distant_obstacle));
    }
}