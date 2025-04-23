use crate::scenes::{title::TitleScene, Scene, Transition};

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        let initial_scene = Box::new(TitleScene::new());
        SceneManager {
            scenes: vec![initial_scene],
        }
    }

    // Summary - pre_update():
    // If the current scene is a TitleScene:
    // - If it's in the loading state → load its assets asynchronously
    // - If it's ready to start the game:
    //     → Load the GameScene asynchronously
    //     → Replace TitleScene with the GameScene in the stack
    pub async fn pre_update(&mut self) {
        // Is the scenes Vector(Stack) empty ? Returns mut ref to last(top) scene
        if let Some(scene) = self.scenes.last_mut() {
            // Is the top scene a TitleScene ? Returns mut ref to TitleScene
            if let Some(title_scene) = scene.as_any().downcast_mut::<TitleScene>() {
                if title_scene.is_loading() {
                    title_scene.load_assets().await;
                }
    
                if title_scene.is_loading_game() {
                    if let Some(game_scene) = title_scene.load_game_scene().await {
                        // Replace: pop old title scene, push new game scene
                        self.scenes.pop(); // Remove TitleScene
                        self.scenes.push(game_scene); // Add GameScene
                    }
                }
            }
        }
    }

    // Summary - update():
    // If there's an active scene:
    // - Call its update() method.
    // - If the scene requests to Pop itself, remove it from the stack.
    // - (Optional) If it wants to Push a new scene, add that scene to the stack.

    pub fn update(&mut self) {
        if let Some(active_scene) = self.scenes.last_mut() {
            match active_scene.update() {
                Transition::None => {}
                //Transition::Push(scene) => self.scenes.push(scene),
                Transition::Pop => {
                    self.scenes.pop();
                }
            }
        }
    }

    pub fn draw(&mut self) {
        if let Some(active_scene) = self.scenes.last_mut() {
            active_scene.draw();
        }
        else {
            // No more scenes left – exit the game
            std::process::exit(0);
        }
    }
}

/*

The tests validate : 
1. Scene stack pops on Transition::Pop
2. Scene draw is called: Verifies draw method is invoked for the active scene.
3. Game exits if no scenes remain.

A note : 
test_game_exits_when_no_scenes_left() is commented out as it calls manager.draw()
which ends with exiting the process.
Please uncomment this test case if needed.

*/

#[cfg(test)]
mod scenemanagement_tests {
    use super::*;
    use crate::scenes::{Transition, Scene};
    use std::cell::RefCell;
    use std::rc::Rc;

    struct MockScene {
        transition: RefCell<Transition>,
        draw_called: Rc<RefCell<bool>>,
    }

    impl MockScene {
        fn new(transition: Transition, draw_called: Rc<RefCell<bool>>) -> Self {
            MockScene {
                transition: RefCell::new(transition),
                draw_called,
            }
        }
    }

    impl Scene for MockScene {
        fn update(&mut self) -> Transition {
            self.transition.replace(Transition::None)
        }

        fn draw(&mut self) {
            *self.draw_called.borrow_mut() = true;
        }

        fn as_any(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_scene_stack_pop_on_transition() {
        let draw_called = Rc::new(RefCell::new(false));
        let mut manager = SceneManager {
            scenes: vec![
                Box::new(MockScene::new(Transition::None, draw_called.clone())),
                Box::new(MockScene::new(Transition::Pop, draw_called.clone())),
            ],
        };

        manager.update();
        assert_eq!(manager.scenes.len(), 1, "Scene stack should pop on Transition::Pop");
    }

    #[test]
    fn test_scene_draw_called() {
        let draw_called = Rc::new(RefCell::new(false));
        let mut manager = SceneManager {
            scenes: vec![Box::new(MockScene::new(Transition::None, draw_called.clone()))],
        };

        manager.draw();
        assert!(*draw_called.borrow(), "Draw should be called on the top scene");
    }

    // #[test]
    // #[should_panic(expected = "exit")]
    // fn test_game_exits_when_no_scenes_left() {
    //     let mut manager = SceneManager { scenes: vec![] };
    //     manager.draw(); // Should trigger process::exit
    // }
}
