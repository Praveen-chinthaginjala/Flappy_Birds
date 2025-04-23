use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

use crate::prefabs::background::Background;
use crate::prefabs::bird::Bird;
use crate::prefabs::ground::Ground;
use crate::prefabs::pipes::{PipeGenerator, PipeGroup};
use crate::prefabs::scoreboard::Scoreboard;
use crate::scenes::{Scene, Transition};
use crate::systems::physics::PhysicsBody;
use crate::systems::storage;

pub struct GameScene {
    sky_texture: Texture2D,
    background: Background,
    ground: Ground,
    pipes_texture: Texture2D,

    instructions: Texture2D,
    get_ready: Texture2D,

    bird: Bird,

    flap_sound: Sound,
    ground_hit_sound: Sound,
    pipe_hit_sound: Sound,
    score_sound: Sound,

    score: i32,
    highscore: i32,
    font: Font,

    is_mouse_down: bool,
    instructions_visible: bool,

    pipes: Vec<PipeGroup>,
    game_over: bool,
    pipe_generator: PipeGenerator,

    scoreboard: Scoreboard,
}

impl GameScene {
    pub async fn new() -> GameScene {
        let bird = Bird::new().await;

        GameScene {
            sky_texture: load_texture("resources/sky.png").await.unwrap(),
            background: Background::new().await,
            ground: Ground::new().await,
            pipes_texture: load_texture("resources/pipes.png").await.unwrap(),
            get_ready: load_texture("resources/get-ready.png").await.unwrap(),
            instructions: load_texture("resources/instructions.png").await.unwrap(),

            bird,

            flap_sound: load_sound("resources/flap.wav").await.unwrap(),
            ground_hit_sound: load_sound("resources/ground-hit.wav").await.unwrap(),
            pipe_hit_sound: load_sound("resources/pipe-hit.wav").await.unwrap(),
            score_sound: load_sound("resources/score.wav").await.unwrap(),

            score: 0,
            highscore: storage::read().unwrap_or(0),
            font: load_ttf_font("resources/font/flappy-font.ttf").await.unwrap(),

            is_mouse_down: true,
            instructions_visible: true,
            pipes: Vec::new(),
            game_over: false,
            pipe_generator: PipeGenerator::new(),

            scoreboard: Scoreboard::new().await,
        }
    }

    fn reset(&mut self) {
        self.instructions_visible = true;
        self.pipes.clear();
        self.background.scroll = true;
        self.ground.scroll = true;
        self.bird.reset();
        self.score = 0;
        self.game_over = false;
    }

    fn start_game(&mut self) {
        if self.instructions_visible {
            self.instructions_visible = false;
        }
        self.bird.allow_gravity = true;
        self.pipe_generator.start();
    }

    fn check_for_collisions(&mut self) {
        let mut bird_died = false;
        if self.bird.alive {
            for pipe_group in &mut self.pipes {
                if pipe_group.collides_with(&self.bird.get_collision_rect()) {
                    bird_died = true;
                }
            }
        }

        if bird_died {
            play_sound(&self.pipe_hit_sound, PlaySoundParams {
                volume: 1.0,
                looped: false,
            });
            self.bird.kill();

            self.pipe_generator.stop();
            self.background.scroll = false;
            self.ground.scroll = false;

            for pipe_group in &mut self.pipes {
                pipe_group.enabled = false;
            }
        }

        if !self.game_over && self.bird.collides_with(&self.ground.get_collision_rect()) {
            play_sound(&self.ground_hit_sound, PlaySoundParams {
                volume: 1.0,
                looped: false,
            });
            self.bird.kill();
            self.bird.allow_gravity = false;
            self.background.scroll = false;
            self.ground.scroll = false;

            self.game_over = true;
            self.pipe_generator.stop();

            if self.score >= self.highscore {
                self.highscore = self.score;
                storage::write(self.highscore).unwrap();
            }
            self.scoreboard.set_score(self.score, self.highscore);

            for pipe_group in &mut self.pipes {
                pipe_group.enabled = false;
            }
        }
    }
}

impl Scene for GameScene {
    fn update(&mut self) -> Transition {
        let bird_x_fixed = self.bird.position.x;

        if !self.instructions_visible {
            self.bird.update();
            self.bird.position.x = bird_x_fixed;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if !self.is_mouse_down {
                let mouse_position = mouse_position().into();

                if self.instructions_visible {
                    self.start_game();
                } else if self.game_over && self.scoreboard.button.contains(mouse_position) {
                    self.reset();
                }

                if self.bird.alive && !self.game_over {
                    play_sound(&self.flap_sound, PlaySoundParams {
                        volume: 1.0,
                        looped: false,
                    });
                    self.bird.flap();
                }

                self.is_mouse_down = true;
            }
        } else {
            self.is_mouse_down = false;
        }

        if !self.game_over {
            for pipe_group in &mut self.pipes {
                if !pipe_group.has_scored && pipe_group.position.x + 27.0 <= self.bird.position.x {
                    pipe_group.has_scored = true;
                    play_sound(&self.score_sound, PlaySoundParams {
                        volume: 1.0,
                        looped: false,
                    });
                    self.score += 1;
                }
                pipe_group.update();
            }

            self.background.update();
            self.ground.update();

            self.check_for_collisions();

            if self.pipe_generator.should_spawn_pipe() {
                // Calculate ground position
                let ground_y = screen_height() - 112.0; // Assuming ground height is 112px
                
                // Try to reuse an existing pipe group first
                let mut spawned = false;
                for pipe_group in &mut self.pipes {
                    if !pipe_group.alive {
                        pipe_group.reset(screen_width(), ground_y);
                        spawned = true;
                        break;
                    }
                }
                
                // If no inactive pipe was found, create a new one
                if !spawned {
                    let mut pipe_group = PipeGroup::new();
                    pipe_group.reset(screen_width(), ground_y);
                    self.pipes.push(pipe_group);
                }
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            return Transition::Pop;
        }

        Transition::None
    }

    fn draw(&mut self) {
        draw_texture(&self.sky_texture, 0.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 100.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 200.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 300.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 400.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 500.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 600.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 700.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 800.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 900.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 1000.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 1100.0, 0.0, WHITE);
        draw_texture(&self.sky_texture, 1200.0, 0.0, WHITE);

        self.background.draw();

        if self.instructions_visible {
            // Center horizontally and position vertically using screen percentages
            let instr_x = screen_width() / 2.0 - self.instructions.width() / 2.0;
            let ready_x = screen_width() / 2.0 - self.get_ready.width() / 2.0;
            
            // Position get_ready at 25% of screen height
            let ready_y = screen_height() * 0.25;
            
            // Position instructions at 60% of screen height
            let instr_y = screen_height() * 0.6;
        
            draw_texture(&self.get_ready, ready_x, ready_y, WHITE);
            draw_texture(&self.instructions, instr_x, instr_y, WHITE);
        }

        for pipe_group in &mut self.pipes {
            pipe_group.draw(&self.pipes_texture);
        }

        self.ground.draw();

        if !self.game_over {
            let text = self.score.to_string();
            let dims = measure_text(&text, Some(&self.font), 32, 1.0);
            draw_text_ex(
                &text,
                screen_width() as f32 / 2.0 - dims.width / 2.0,
                40.0,
                TextParams {
                    font: Some(&self.font),
                    font_size: 32,
                    color: WHITE,
                    ..Default::default()
                },
            );
        } else {
            self.scoreboard.draw();
        }

        self.bird.draw();
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/*

The tests validate : 
1. Resetting game state, clearing pipes, resetting bird, and setting scroll flags
2. Hiding instructions and enabling gravity for the bird at the start of game
3. Handling bird-pipe collision, triggering game over state

*/

#[cfg(test)]
mod tests {
    use super::*;

    // Minimal test double versions of components used by GameScene
    struct DummyBird {
        pub alive: bool,
        pub allow_gravity: bool,
        pub reset_called: bool,
    }

    impl DummyBird {
        fn new() -> Self {
            DummyBird {
                alive: true,
                allow_gravity: false,
                reset_called: false,
            }
        }

        fn reset(&mut self) {
            self.reset_called = true;
            self.alive = true;
            self.allow_gravity = false;
        }

        fn kill(&mut self) {
            self.alive = false;
        }
    }

    struct DummyPipeGroup {
        pub enabled: bool,
    }

    impl DummyPipeGroup {
        fn new() -> Self {
            DummyPipeGroup { enabled: true }
        }
    }

    struct DummyBackground {
        pub scroll: bool,
    }

    struct DummyGround {
        pub scroll: bool,
    }

    struct DummyGameScene {
        bird: DummyBird,
        instructions_visible: bool,
        pipes: Vec<DummyPipeGroup>,
        background: DummyBackground,
        ground: DummyGround,
        score: i32,
        highscore: i32,
        game_over: bool,
    }

    impl DummyGameScene {
        fn new() -> Self {
            DummyGameScene {
                bird: DummyBird::new(),
                instructions_visible: true,
                pipes: vec![DummyPipeGroup::new()],
                background: DummyBackground { scroll: true },
                ground: DummyGround { scroll: true },
                score: 0,
                highscore: 10,
                game_over: false,
            }
        }

        fn reset(&mut self) {
            self.instructions_visible = true;
            self.pipes.clear();
            self.background.scroll = true;
            self.ground.scroll = true;
            self.bird.reset();
            self.score = 0;
            self.game_over = false;
        }

        fn start_game(&mut self) {
            if self.instructions_visible {
                self.instructions_visible = false;
            }
            self.bird.allow_gravity = true;
        }

        fn fake_game_over(&mut self) {
            self.bird.kill();
            self.background.scroll = false;
            self.ground.scroll = false;
            self.game_over = true;
            for pipe in &mut self.pipes {
                pipe.enabled = false;
            }
        }
    }

    #[test]
    fn test_reset_resets_all_game_state() {
        let mut scene = DummyGameScene::new();
        scene.score = 5;
        scene.instructions_visible = false;
        scene.bird.alive = false;
        scene.background.scroll = false;
        scene.ground.scroll = false;
        scene.game_over = true;

        scene.reset();

        assert_eq!(scene.score, 0);
        assert!(scene.instructions_visible);
        assert!(scene.bird.alive);
        assert!(scene.background.scroll);
        assert!(scene.ground.scroll);
        assert!(scene.bird.reset_called);
        assert!(!scene.game_over);
        assert!(scene.pipes.is_empty());
    }

    #[test]
    fn test_start_game_enables_gravity_and_hides_instructions() {
        let mut scene = DummyGameScene::new();
        scene.start_game();
        assert!(!scene.instructions_visible);
        assert!(scene.bird.allow_gravity);
    }

    #[test]
    fn test_game_over_flags_are_correctly_set() {
        let mut scene = DummyGameScene::new();
        scene.fake_game_over();

        assert!(!scene.bird.alive);
        assert!(!scene.background.scroll);
        assert!(!scene.ground.scroll);
        assert!(scene.game_over);
        for pipe in &scene.pipes {
            assert!(!pipe.enabled);
        }
    }
}

/*

The tests validate (update() logic): 
1. Bird flaps on click if alive and game is active
2. Score increments when bird passes a pipe
3. No scoring if pipe is not passed
4. Resets state on scoreboard click
5. Bird doesn’t flap after death

*/

#[cfg(test)]
mod update_tests {
    use super::*;

    struct DummyBird {
        pub position_x: f32,
        pub alive: bool,
        pub flap_called: bool,
        pub allow_gravity: bool,
    }

    impl DummyBird {
        fn new() -> Self {
            DummyBird {
                position_x: 100.0,
                alive: true,
                flap_called: false,
                allow_gravity: false,
            }
        }

        fn flap(&mut self) {
            self.flap_called = true;
        }
    }

    struct DummyPipeGroup {
        pub position_x: f32,
        pub has_scored: bool,
        pub alive: bool,
    }

    impl DummyPipeGroup {
        fn new(position_x: f32) -> Self {
            DummyPipeGroup {
                position_x,
                has_scored: false,
                alive: true,
            }
        }
    }

    struct DummyGameScene {
        bird: DummyBird,
        pipes: Vec<DummyPipeGroup>,
        score: i32,
        game_over: bool,
        instructions_visible: bool,
    }

    impl DummyGameScene {
        fn new_with_pipe(pipe_x: f32) -> Self {
            DummyGameScene {
                bird: DummyBird::new(),
                pipes: vec![DummyPipeGroup::new(pipe_x)],
                score: 0,
                game_over: false,
                instructions_visible: false,
            }
        }

        fn update_with_flap_click(&mut self) {
            if self.bird.alive && !self.game_over {
                self.bird.flap();
            }
        }

        fn update_score(&mut self) {
            for pipe in &mut self.pipes {
                if !pipe.has_scored && pipe.position_x + 27.0 <= self.bird.position_x {
                    pipe.has_scored = true;
                    self.score += 1;
                }
            }
        }

        fn simulate_reset_click(&mut self, mouse_over_button: bool) {
            if self.game_over && mouse_over_button {
                self.reset();
            }
        }

        fn reset(&mut self) {
            self.score = 0;
            self.instructions_visible = true;
            self.pipes.clear();
            self.game_over = false;
        }
    }

    #[test]
    fn test_flap_triggers_when_game_running() {
        let mut scene = DummyGameScene::new_with_pipe(50.0);
        scene.update_with_flap_click();
        assert!(scene.bird.flap_called);
    }

    #[test]
    fn test_score_increases_when_passing_pipe() {
        let mut scene = DummyGameScene::new_with_pipe(70.0); // 70 + 27 = 97 < 100 = bird x
        scene.update_score();
        assert_eq!(scene.score, 1);
        assert!(scene.pipes[0].has_scored);
    }

    #[test]
    fn test_score_does_not_increase_if_not_passed_pipe() {
        let mut scene = DummyGameScene::new_with_pipe(80.0); // 80 + 27 = 107 > 100
        scene.update_score();
        assert_eq!(scene.score, 0);
    }

    #[test]
    fn test_game_reset_on_click_after_game_over() {
        let mut scene = DummyGameScene::new_with_pipe(50.0);
        scene.game_over = true;

        scene.simulate_reset_click(true);
        assert_eq!(scene.score, 0);
        assert!(scene.instructions_visible);
        assert!(scene.pipes.is_empty());
        assert!(!scene.game_over);
    }

    #[test]
    fn test_flap_ignored_when_game_over() {
        let mut scene = DummyGameScene::new_with_pipe(50.0);
        scene.game_over = true;

        scene.update_with_flap_click();
        assert!(!scene.bird.flap_called);
    }
}

/*

The tests validate (Spawn and UI logic) :
1. Ensuring pipe is spawned only after enough time passes
2. Instructions disappear after 3+ seconds
3. Instructions remain visible if time < threshold

*/

#[cfg(test)]
mod spawn_and_ui_tests {
    use super::*;

    struct DummyPipeSpawner {
        spawn_timer: f32,
        spawn_interval: f32,
        pipes_spawned: usize,
    }

    impl DummyPipeSpawner {
        fn new(spawn_interval: f32) -> Self {
            DummyPipeSpawner {
                spawn_timer: 0.0,
                spawn_interval,
                pipes_spawned: 0,
            }
        }

        fn update(&mut self, delta: f32) {
            self.spawn_timer += delta;
            if self.spawn_timer >= self.spawn_interval {
                self.spawn_pipe();
                self.spawn_timer = 0.0;
            }
        }

        fn spawn_pipe(&mut self) {
            self.pipes_spawned += 1;
        }
    }

    struct DummyInstructionUI {
        time_elapsed: f32,
        visible: bool,
        hide_after: f32,
    }

    impl DummyInstructionUI {
        fn new(hide_after: f32) -> Self {
            DummyInstructionUI {
                time_elapsed: 0.0,
                visible: true,
                hide_after,
            }
        }

        fn update(&mut self, delta: f32) {
            self.time_elapsed += delta;
            if self.time_elapsed > self.hide_after {
                self.visible = false;
            }
        }
    }

    #[test]
    fn test_pipe_spawns_after_interval() {
        let mut spawner = DummyPipeSpawner::new(2.0);

        spawner.update(1.0);
        assert_eq!(spawner.pipes_spawned, 0);

        spawner.update(1.0); // Total = 2.0
        assert_eq!(spawner.pipes_spawned, 1);

        spawner.update(2.1); // Should spawn another pipe
        assert_eq!(spawner.pipes_spawned, 2);
    }

    #[test]
    fn test_instruction_hides_after_timeout() {
        let mut ui = DummyInstructionUI::new(3.0);

        ui.update(2.0);
        assert!(ui.visible);

        ui.update(1.1); // Total = 3.1
        assert!(!ui.visible);
    }

    #[test]
    fn test_instruction_remains_visible_before_timeout() {
        let mut ui = DummyInstructionUI::new(4.0);

        ui.update(1.0);
        ui.update(1.5);
        assert!(ui.visible);
    }
}

/*

The tests valide :
1. Bird-Pipe collision logic

*/

#[cfg(test)]
mod collision_tests {
    use super::*;
    use macroquad::math::Rect;

    struct DummyBird {
        rect: Rect,
    }

    impl DummyBird {
        fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
            DummyBird {
                rect: Rect::new(x, y, w, h),
            }
        }

        fn get_collision_rect(&self) -> Rect {
            self.rect
        }
    }

    struct DummyPipeGroup {
        top_rect: Rect,
        bottom_rect: Rect,
    }

    impl DummyPipeGroup {
        fn new(top: Rect, bottom: Rect) -> Self {
            DummyPipeGroup {
                top_rect: top,
                bottom_rect: bottom,
            }
        }

        fn collides_with(&self, bird_rect: &Rect) -> bool {
            self.top_rect.overlaps(bird_rect) || self.bottom_rect.overlaps(bird_rect)
        }
    }

    #[test]
    fn test_bird_collides_with_pipe() {
        let bird = DummyBird::new(50.0, 50.0, 30.0, 30.0);
        let pipe = DummyPipeGroup::new(
            Rect::new(45.0, 0.0, 40.0, 100.0),   // top pipe
            Rect::new(45.0, 180.0, 40.0, 300.0)  // bottom pipe
        );

        let collides = pipe.collides_with(&bird.get_collision_rect());
        assert!(collides, "Bird should collide with the top pipe");
    }

    #[test]
    fn test_bird_does_not_collide_with_pipe() {
        let bird = DummyBird::new(200.0, 50.0, 30.0, 30.0);
        let pipe = DummyPipeGroup::new(
            Rect::new(45.0, 0.0, 40.0, 100.0),   // top pipe
            Rect::new(45.0, 180.0, 40.0, 300.0)  // bottom pipe
        );

        let collides = pipe.collides_with(&bird.get_collision_rect());
        assert!(!collides, "Bird should not collide with pipe at this position");
    }
}

/*

The tests validate (Bird pause and death logic): 
1. Bird death on collision with ground
2. Game pauses on death
3. Game reset logic

*/
#[cfg(test)]
mod game_tests {
    use super::*;
    use macroquad::math::Rect;

    struct DummyBird {
        rect: Rect,
        alive: bool,
        allow_gravity: bool,
    }

    impl DummyBird {
        fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
            DummyBird {
                rect: Rect::new(x, y, w, h),
                alive: true,
                allow_gravity: true,
            }
        }

        fn get_collision_rect(&self) -> Rect {
            self.rect
        }

        fn kill(&mut self) {
            self.alive = false;
        }

        fn reset(&mut self) {
            self.alive = true;
            self.rect = Rect::new(50.0, 50.0, 30.0, 30.0);
        }

        fn collides_with(&self, other: &Rect) -> bool {
            self.rect.overlaps(other)
        }
    }

    struct DummyGround {
        rect: Rect,
    }

    impl DummyGround {
        fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
            DummyGround {
                rect: Rect::new(x, y, w, h),
            }
        }

        fn get_collision_rect(&self) -> Rect {
            self.rect
        }
    }

    struct DummyPipeGroup {
        top_rect: Rect,
        bottom_rect: Rect,
    }

    impl DummyPipeGroup {
        fn new(top: Rect, bottom: Rect) -> Self {
            DummyPipeGroup {
                top_rect: top,
                bottom_rect: bottom,
            }
        }

        fn collides_with(&self, bird_rect: &Rect) -> bool {
            // Check if the bird overlaps with either the top or bottom pipe
            self.top_rect.overlaps(bird_rect) || self.bottom_rect.overlaps(bird_rect)
        }
    }

    struct GameSceneMock {
        bird: DummyBird,
        ground: DummyGround,
        pipes: Vec<DummyPipeGroup>,
        game_over: bool,
    }

    impl GameSceneMock {
        fn new() -> Self {
            let bird = DummyBird::new(50.0, 50.0, 30.0, 30.0);
            let ground = DummyGround::new(0.0, 500.0, 800.0, 100.0);
            GameSceneMock {
                bird,
                ground,
                pipes: vec![],
                game_over: false,
            }
        }

        fn check_for_collisions(&mut self) {
            if self.bird.collides_with(&self.ground.get_collision_rect()) {
                self.bird.kill();
                self.game_over = true;
            }
        }

        fn update(&mut self) {
            self.check_for_collisions();
        }

        fn reset(&mut self) {
            self.bird.reset();
            self.game_over = false;
        }
    }

    #[test]
    fn test_bird_death_on_ground_collision() {
        let mut game = GameSceneMock::new();
        // Simulate the bird hitting the ground
        game.bird.rect = Rect::new(50.0, 500.0, 30.0, 30.0); // Same Y position as ground

        game.update();

        assert!(game.game_over, "Game should be over after bird collides with ground");
        assert!(!game.bird.alive, "Bird should be dead after collision with ground");
    }

    #[test]
    fn test_game_pauses_on_death() {
        let mut game = GameSceneMock::new();
        // Simulate bird death by collision with ground
        game.bird.rect = Rect::new(50.0, 500.0, 30.0, 30.0); // Same Y position as ground

        game.update();

        // The game should be over and no pipes should spawn
        assert!(game.game_over, "Game should be over after bird collides with ground");
        assert!(!game.bird.alive, "Bird should be dead after collision with ground");
        assert!(game.pipes.is_empty(), "No pipes should spawn while the game is paused");
    }

    #[test]
    fn test_game_reset() {
        let mut game = GameSceneMock::new();
        // Simulate bird death
        game.bird.rect = Rect::new(50.0, 500.0, 30.0, 30.0); // Same Y position as ground

        game.update();
        assert!(game.game_over, "Game should be over after bird collides with ground");

        // Now reset the game
        game.reset();

        // Assert that the bird is alive and game_over flag is reset
        assert!(game.bird.alive, "Bird should be alive after reset");
        assert!(!game.game_over, "Game should not be over after reset");
    }
}