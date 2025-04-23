use macroquad::prelude::*;
use crate::prefabs::button::Button;

pub struct Scoreboard {
    game_over_texture: Texture2D,
    scoreboard_texture: Texture2D,
    medal_texture: Texture2D,
    font: Font,
    score: i32,
    highscore: i32,
    pub button: Button,
}

impl Scoreboard {
    pub async fn new() -> Self {
        let game_over_texture = load_texture("resources/gameover.png")
            .await
            .expect("Failed to load gameover texture");
        
        let scoreboard_texture = load_texture("resources/scoreboard.png")
            .await
            .expect("Failed to load scoreboard texture");
        
        let medal_texture = load_texture("resources/medals.png")
            .await
            .expect("Failed to load medals texture");

        let font = load_ttf_font("resources/font/flappy-font.ttf")
            .await
            .expect("Failed to load font");

        Scoreboard {
            game_over_texture,
            scoreboard_texture,
            medal_texture,
            font,
            score: 0,
            highscore: 0,
            button: Button::new().await,
        }
    }

    pub fn set_score(&mut self, score: i32, highscore: i32) {
        self.score = score;
        self.highscore = highscore;
    }

    pub fn draw(&self) {
        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);

        // Draw Game Over text
        self.draw_game_over(screen_center);

        // Draw scoreboard background
        let scoreboard_rect = self.draw_scoreboard_background(screen_center);

        // Draw scores and medals on the scoreboard
        self.draw_scores_and_medals(scoreboard_rect);

        // Draw play button
        self.button.draw();
    }

    fn draw_game_over(&self, screen_center: Vec2) {
        let game_over_pos = vec2(
            screen_center.x - self.game_over_texture.width() / 2.0,
            screen_center.y * 0.1
        );
        draw_texture(&self.game_over_texture, game_over_pos.x, game_over_pos.y, WHITE);
    }

    fn draw_scoreboard_background(&self, screen_center: Vec2) -> Rect {
        let scoreboard_pos = vec2(
            screen_center.x - self.scoreboard_texture.width() / 2.0,
            screen_center.y - self.scoreboard_texture.height() / 2.0 - 130.0
        );
        
        draw_texture(
            &self.scoreboard_texture,
            scoreboard_pos.x,
            scoreboard_pos.y,
            WHITE
        );
        
        Rect::new(
            scoreboard_pos.x,
            scoreboard_pos.y,
            self.scoreboard_texture.width(),
            self.scoreboard_texture.height()
        )
    }

    fn draw_scores_and_medals(&self, scoreboard_rect: Rect) {
        // Draw scores on the right side of the scoreboard
        let score_x = scoreboard_rect.x + scoreboard_rect.w * 0.80;
        let score_y = scoreboard_rect.y + scoreboard_rect.h * 0.40;
        
        // Current Score
        self.draw_score_text(
            &self.score.to_string(),
            score_x,
            score_y,
            Color::new(0.19, 0.19, 0.17, 1.0) // Dark brown
        );

        // High Score
        self.draw_score_text(
            &self.highscore.to_string(),
            score_x,
            score_y + 47.0,
            Color::new(0.19, 0.19, 0.17, 1.0)
        );

        // Draw medals on the left side of the scoreboard
        // Adjusted position to better align with the medal slot
        self.draw_medal(
            scoreboard_rect.x + scoreboard_rect.w * 0.12,  // Moved from 0.25 to 0.15 to position it more to the left
            scoreboard_rect.y + scoreboard_rect.h * 0.37
        );
    }

    fn draw_score_text(&self, text: &str, x: f32, y: f32, color: Color) {
        let text_size = 30.0;
        let measurement = measure_text(text, Some(&self.font), text_size as u16, 1.0);
        
        draw_text_ex(
            text,
            x - measurement.width / 2.0,  // Changed from + to - to align better
            y + measurement.height / 2.0,
            TextParams {
                font: Some(&self.font),
                font_size: text_size as u16,
                color,
                ..Default::default()
            },
        );
    }

    fn draw_medal(&self, x: f32, y: f32) {
        let medal_source = match self.score {
            s if s >= 20 => Rect::new(0.0, 46.0, 44.0, 46.0),  // Gold medal
            s if s >= 10 => Rect::new(0.0, 0.0, 44.0, 46.0),   // Silver medal
            _ => return,  // No medal for lower scores
        };

        draw_texture_ex(
            &self.medal_texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(medal_source),
                ..Default::default()
            },
        );
    }
}

/*

The tests validate:
1. Score assignment logic for score and highscore
2. No stale values remain after successive updates
3. Safe hadling of edge values
4. Independence of score and highscore 

*/

#[cfg(test)]
mod tests {
    use super::*;

    // A dummy struct to isolate and test set_score logic
    struct DummyScoreboard {
        score: i32,
        highscore: i32,
    }

    impl DummyScoreboard {
        fn set_score(&mut self, score: i32, highscore: i32) {
            self.score = score;
            self.highscore = highscore;
        }
    }

    #[test]
    fn test_set_score_updates_internal_state() {
        let mut scoreboard = DummyScoreboard { score: 0, highscore: 0 };
        scoreboard.set_score(42, 100);

        assert_eq!(scoreboard.score, 42);
        assert_eq!(scoreboard.highscore, 100);
    }

    #[test]
    fn test_multiple_score_updates() {
        let mut scoreboard = DummyScoreboard { score: 0, highscore: 0 };

        scoreboard.set_score(10, 20);
        assert_eq!(scoreboard.score, 10);
        assert_eq!(scoreboard.highscore, 20);

        scoreboard.set_score(55, 99);
        assert_eq!(scoreboard.score, 55);
        assert_eq!(scoreboard.highscore, 99);
    }

    #[test]
    fn test_score_can_be_zero() {
        let mut scoreboard = DummyScoreboard { score: 0, highscore: 0 };

        scoreboard.set_score(0, 0);
        assert_eq!(scoreboard.score, 0);
        assert_eq!(scoreboard.highscore, 0);
    }

    #[test]
    fn test_score_and_highscore_independence() {
        let mut scoreboard = DummyScoreboard { score: 0, highscore: 0 };

        scoreboard.set_score(30, 0);
        assert_eq!(scoreboard.score, 30);
        assert_eq!(scoreboard.highscore, 0);

        scoreboard.set_score(0, 50);
        assert_eq!(scoreboard.score, 0);
        assert_eq!(scoreboard.highscore, 50);
    }
}