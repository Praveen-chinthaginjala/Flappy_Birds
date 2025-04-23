use macroquad::prelude::*;
use ::rand::Rng;
use crate::systems::physics::{check_collision, PhysicsBody};
use crate::SCROLL_SPEED;

pub struct Pipe {
    position: Vec2,
    source_rect: Rect,
}

impl Pipe {
    fn new(position: Vec2, source_rect: Rect) -> Self {
        Pipe { position, source_rect }
    }

    fn draw(&self, group_position: Vec2, texture: &Texture2D) {
        draw_texture_ex(
            texture,
            group_position.x + self.position.x,
            group_position.y + self.position.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.source_rect),
                ..Default::default()
            },
        );
    }
}

impl PhysicsBody for Pipe {
    fn get_collision_rect(&mut self) -> Rect {
        Rect::new(self.position.x, self.position.y, 54.0, 320.0)
    }

    fn collides_with(&mut self, obj: &Rect) -> bool {
        check_collision(&self.get_collision_rect(), obj)
    }
}

pub struct PipeGroup {
    top_pipe: Pipe,
    bottom_pipe: Pipe,
    pub position: Vec2,
    pub alive: bool,
    pub enabled: bool,
    pub has_scored: bool,
}

impl PipeGroup {
    const GAP_SIZE: f32 = 160.0;
    const PIPE_HEIGHT: f32 = 320.0;

    pub fn new() -> Self {
        PipeGroup {
            position: Vec2::new(0.0, 0.0),
            top_pipe: Pipe::new(
                Vec2::new(0.0, 0.0),
                Rect::new(0.0, 0.0, 54.0, 320.0),
            ),
            bottom_pipe: Pipe::new(
                Vec2::new(0.0, 0.0),
                Rect::new(54.0, 0.0, 54.0, 320.0),
            ),
            alive: false,
            enabled: false,
            has_scored: false,
        }
    }

    pub fn update(&mut self) {
        if self.alive && self.enabled {
            self.position.x -= SCROLL_SPEED;
        }
        if self.position.x < -54.0 {
            self.alive = false;
            self.enabled = false;
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        self.top_pipe.draw(self.position, texture);
        self.bottom_pipe.draw(self.position, texture);
    }

    pub fn reset(&mut self, x: f32, ground_y: f32) {
        let mut rng = ::rand::rng();
        
        // Calculate valid gap range
        let min_gap_top = 100.0;
        let max_gap_top = ground_y - Self::GAP_SIZE - 100.0; // Leave space at bottom
        
        // Ensure valid range
        let gap_top = if max_gap_top > min_gap_top {
            rng.random_range(min_gap_top..max_gap_top)
        } else {
            min_gap_top
        };

        self.position.x = x;
        self.position.y = 0.0; // Reset y position
        self.top_pipe.position.y = gap_top - Self::PIPE_HEIGHT;
        self.bottom_pipe.position.y = gap_top + Self::GAP_SIZE;
        
        self.alive = true;
        self.enabled = true;
        self.has_scored = false;
    }
}

impl PhysicsBody for PipeGroup {
    fn get_collision_rect(&mut self) -> Rect {
        Rect::new(0.0, 0.0, 0.0, 0.0)
    }

    fn collides_with(&mut self, obj: &Rect) -> bool {
        let relative_rect = Rect::new(
            obj.x - self.position.x - 27.0,
            obj.y - self.position.y - 12.0,
            obj.w,
            obj.h,
        );
        self.top_pipe.collides_with(&relative_rect)
            || self.bottom_pipe.collides_with(&relative_rect)
    }
}

pub struct PipeGenerator {
    counter: i32,
    enabled: bool,
}

impl PipeGenerator {
    pub fn new() -> Self {
        PipeGenerator {
            counter: 0,
            enabled: false,
        }
    }

    pub fn start(&mut self) {
        self.enabled = true;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn should_spawn_pipe(&mut self) -> bool {
        if self.enabled {
            self.counter += 1;
            if self.counter >= 80 {
                self.counter = 0;
                return true;
            }
        }
        false
    }
}

/*

The tests validate :
1. PipeGroup position updates
2. PipeGroup deactivation logic
3. PipeGroup reset logic
4. Collision detection delegation
5. PipeGenerator spawn logic

*/

#[cfg(test)]
mod tests {
    use super::*;
    use macroquad::prelude::Rect;
    use float_eq::assert_float_eq;

    fn test_pipe_group() -> PipeGroup {
        PipeGroup::new()
    }

    #[test]
    fn test_pipe_group_update_moves_left_when_enabled() {
        let mut group = test_pipe_group();
        group.alive = true;
        group.enabled = true;
        group.position.x = 100.0;

        group.update();

        assert_float_eq!(group.position.x, 100.0 - SCROLL_SPEED, abs <= 0.001);
    }

    #[test]
    fn test_pipe_group_disables_when_offscreen() {
        let mut group = test_pipe_group();
        group.alive = true;
        group.enabled = true;
        group.position.x = -54.1;

        group.update();

        assert!(!group.alive);
        assert!(!group.enabled);
    }

    #[test]
    fn test_pipe_group_reset_sets_pipe_positions() {
        let mut group = test_pipe_group();
        let x = 300.0;
        let ground_y = 600.0;

        group.reset(x, ground_y);

        assert!(group.top_pipe.position.y < 0.0); // should be above gap
        assert!(group.bottom_pipe.position.y > 0.0); // should be below gap
        assert!(group.alive);
        assert!(group.enabled);
        assert!(!group.has_scored);
        assert_float_eq!(group.position.x, x, abs <= 0.001);
    }

    #[test]
    fn test_pipe_group_collision_calls_both_pipes() {
        let mut group = test_pipe_group();
        group.top_pipe.position.y = 0.0;
        group.bottom_pipe.position.y = 300.0;
        group.position = Vec2::new(0.0, 0.0);

        let _hitbox = Rect::new(0.0, 0.0, 54.0, 320.0);
        let mut obj = Rect::new(27.0, 12.0, 20.0, 20.0); // Matches relative offset logic

        assert!(group.collides_with(&mut obj));
    }

    #[test]
    fn test_pipe_generator_spawning_behavior() {
        let mut generator = PipeGenerator::new();
        generator.start();

        let mut triggered = false;
        for _ in 0..80 {
            triggered = generator.should_spawn_pipe();
        }

        assert!(triggered);
        assert_eq!(generator.counter, 0);
    }

    #[test]
    fn test_pipe_generator_stop_prevents_spawn() {
        let mut generator = PipeGenerator::new();
        generator.start();
        generator.stop();

        for _ in 0..100 {
            assert!(!generator.should_spawn_pipe());
        }
    }
}