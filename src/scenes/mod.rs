pub mod game;
pub mod title;
use std::any::Any;

/// A trait representing a game scene.
pub trait Scene {
    /// Updates the scene. Returns a `Transition` to indicate what to do next.
    fn update(&mut self) -> Transition;

    /// Draws the scene.
    fn draw(&mut self);

    fn as_any(&mut self) -> &mut dyn Any;

    // Add async versions of update and draw that may be used in the future
    /* 
    fn update_async(&mut self) -> TransitionFuture {
        Box::pin(async move {
            self.update()
        })
    }
    
    fn draw_async(&mut self) -> DrawFuture {
        Box::pin(async move {
            self.draw();
        })
    }
    */
}

/// Represents what the game loop should do next with the scene stack.
pub enum Transition {
    /// Do nothing, keep the current scene.
    None,

    /// Pop the current scene off the stack.
    Pop,
}
