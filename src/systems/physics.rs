use macroquad::prelude::Rect;

pub trait PhysicsBody {
    fn get_collision_rect(&mut self) -> Rect;
    fn collides_with(&mut self, obj: &Rect) -> bool;
}

pub fn check_collision(rect1: &Rect, rect2: &Rect) -> bool {
    rect1.x < rect2.x + rect2.w
        && rect1.x + rect1.w > rect2.x
        && rect1.y < rect2.y + rect2.h
        && rect1.y + rect1.h > rect2.y
}

/*

The tests validate :
1. Collision detection during overlap
2. Collision detection during no overlap
3. Physics logic when a collison happens
4. Physics logic when a collision doesn't happen

*/

#[cfg(test)]
mod physics_tests {
    use super::*;
    use macroquad::prelude::Rect;

    struct DummyBody {
        rect: Rect,
    }

    impl DummyBody {
        fn new(rect: Rect) -> Self {
            DummyBody { rect }
        }
    }

    impl PhysicsBody for DummyBody {
        fn get_collision_rect(&mut self) -> Rect {
            self.rect
        }

        fn collides_with(&mut self, other: &Rect) -> bool {
            check_collision(&self.get_collision_rect(), other)
        }
    }

    #[test]
    fn test_collision_detects_overlap() {
        let rect1 = Rect::new(0.0, 0.0, 50.0, 50.0);
        let rect2 = Rect::new(25.0, 25.0, 50.0, 50.0);
        assert!(check_collision(&rect1, &rect2), "Rectangles should collide");
    }

    #[test]
    fn test_collision_detects_no_overlap() {
        let rect1 = Rect::new(0.0, 0.0, 50.0, 50.0);
        let rect2 = Rect::new(100.0, 100.0, 50.0, 50.0);
        assert!(!check_collision(&rect1, &rect2), "Rectangles should not collide");
    }

    #[test]
    fn test_physics_body_collision_true() {
        let mut body = DummyBody::new(Rect::new(0.0, 0.0, 50.0, 50.0));
        let other = Rect::new(25.0, 25.0, 50.0, 50.0);
        assert!(body.collides_with(&other), "Body should collide with other rect");
    }

    #[test]
    fn test_physics_body_collision_false() {
        let mut body = DummyBody::new(Rect::new(0.0, 0.0, 50.0, 50.0));
        let other = Rect::new(100.0, 100.0, 50.0, 50.0);
        assert!(!body.collides_with(&other), "Body should not collide with other rect");
    }
}