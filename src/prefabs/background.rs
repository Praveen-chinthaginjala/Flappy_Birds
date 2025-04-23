use macroquad::prelude::*;
use crate::SCROLL_SPEED;

pub struct Background {
    forest_texture: Texture2D,
    cityscape_texture: Texture2D,
    cloud_texture: Texture2D,

    forest_pos: f32,
    cityscape_pos: f32,
    cloud_pos: f32,

    pub scroll: bool,
}

impl Background {
    pub async fn new() -> Self {
        let forest_texture = load_texture("./resources/trees.png").await.expect("trees.png not found");
        forest_texture.set_filter(FilterMode::Nearest); // optional: avoid smoothing
        let cityscape_texture = load_texture("./resources/cityscape.png").await.expect("cityscape.png not found");
        cityscape_texture.set_filter(FilterMode::Nearest);
        let cloud_texture = load_texture("./resources/clouds.png").await.expect("clouds.png not found");
        cloud_texture.set_filter(FilterMode::Nearest);

        Background {
            forest_texture,
            cityscape_texture,
            cloud_texture,
            forest_pos: 0.0,
            cityscape_pos: 0.0,
            cloud_pos: 0.0,
            scroll: true,
        }
    }

    pub fn update(&mut self) {
        if self.scroll {
            self.forest_pos = (self.forest_pos - SCROLL_SPEED * 0.75) % self.forest_texture.width();
            self.cityscape_pos = (self.cityscape_pos - SCROLL_SPEED * 0.5) % self.cityscape_texture.width();
            self.cloud_pos = (self.cloud_pos - SCROLL_SPEED * 0.25) % self.cloud_texture.width();
        }
    }

    pub fn draw(&self) {
        // Y offset from bottom — can adjust for each layer
        let forest_y_offset = 0.0;
        let cityscape_y_offset = 130.0;
        let cloud_y_offset = 130.0;
    
        self.draw_layer(&self.cloud_texture, self.cloud_pos, cloud_y_offset);
        self.draw_layer(&self.cityscape_texture, self.cityscape_pos, cityscape_y_offset);
        self.draw_layer(&self.forest_texture, self.forest_pos, forest_y_offset);
    }

    fn draw_layer(&self, texture: &Texture2D, x_pos: f32, y_offset_from_bottom: f32) {
        let texture_width = texture.width();
        let y = screen_height() - y_offset_from_bottom - texture.height();

        // Draw six copies to ensure seamless scroll
        draw_texture(texture, x_pos, y, WHITE);
        draw_texture(texture, x_pos + 1.0 * texture_width, y, WHITE);
        draw_texture(texture, x_pos + 2.0 * texture_width, y, WHITE);
        draw_texture(texture, x_pos + 3.0 * texture_width, y, WHITE);
        draw_texture(texture, x_pos + 4.0 * texture_width, y, WHITE);
        draw_texture(texture, x_pos + 5.0 * texture_width, y, WHITE);
    }

    // This function is added to production code for extensive test coverage
    pub fn calculate_positions(
        forest_pos: f32,
        cityscape_pos: f32,
        cloud_pos: f32,
        forest_width: f32,
        cityscape_width: f32,
        cloud_width: f32,
        scroll: bool
    ) -> (f32, f32, f32) {
        if scroll {
            (
                (forest_pos - SCROLL_SPEED * 0.75) % forest_width,
                (cityscape_pos - SCROLL_SPEED * 0.5) % cityscape_width,
                (cloud_pos - SCROLL_SPEED * 0.25) % cloud_width
            )
        } else {
            (forest_pos, cityscape_pos, cloud_pos)
        }
    }
}

/* 

The tests validate:
1. Basic position updates
2. Position wrapping behavior
3. Scroll enable/disable state
4. Relative parallax speeds
5. Correct modulo operations

*/ 

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    #[test]
    fn test_scroll_calculations() {
        // Test basic scrolling
        let (f, c, cl) = Background::calculate_positions(
            0.0, 0.0, 0.0,
            100.0, 150.0, 200.0,
            true
        );
        
        assert_float_eq!(f, -SCROLL_SPEED * 0.75, abs <= 0.001);
        assert_float_eq!(c, -SCROLL_SPEED * 0.5, abs <= 0.001);
        assert_float_eq!(cl, -SCROLL_SPEED * 0.25, abs <= 0.001);
    }

    #[test]
    fn test_position_wrapping() {
        // Test negative position wrapping
        let (f, _, _) = Background::calculate_positions(
            -95.0, 0.0, 0.0,
            100.0, 150.0, 200.0,
            true
        );
        
        let expected = (-95.0 - SCROLL_SPEED * 0.75) % 100.0;
        assert_float_eq!(f, expected, abs <= 0.001);
    }

    #[test] 
    fn test_scroll_disabled() {
        // Test scroll disabled state
        let (f, c, cl) = Background::calculate_positions(
            10.0, 20.0, 30.0,
            100.0, 150.0, 200.0,
            false
        );
        
        assert_float_eq!(f, 10.0, abs <= 0.001);
        assert_float_eq!(c, 20.0, abs <= 0.001);
        assert_float_eq!(cl, 30.0, abs <= 0.001);
    }

    #[test]
    fn test_parallax_speeds() {
        // Verify relative movement speeds
        let (f, c, cl) = Background::calculate_positions(
            0.0, 0.0, 0.0,
            100.0, 150.0, 200.0,
            true
        );
        
        assert!(f.abs() > c.abs());
        assert!(c.abs() > cl.abs());
    }
}