use crate::graph::Graph;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;
use ggez::glam::Vec2;

pub struct GraphVisualizer {
    // Your state here...
}

impl GraphVisualizer {
    pub fn new(_ctx: &mut Context) -> GraphVisualizer {
        // Load/create resources such as images here.
        GraphVisualizer {
            // ...
        }
    }
}

impl EventHandler for GraphVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        let text = ggez::graphics::Text::new("yippee!!");

        let mut params = ggez::graphics::DrawParam::new();
        params.color = Color {
            r: 150.0,
            g: 150.0,
            b: 150.0,
            a: 150.0,
        };
        params.dest([20.0, 20.0]);

        canvas.draw(
            &text,
            Vec2::new(20.0, 20.0),
        );

        canvas.finish(ctx)
    }
}
