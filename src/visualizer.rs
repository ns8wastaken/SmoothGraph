use crate::graph::Graph;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;
use ggez::glam::Vec2;

pub struct GraphVisualizer {
    graph: Graph,
}

impl GraphVisualizer {
    pub fn new(_ctx: &mut Context) -> GraphVisualizer {
        GraphVisualizer {
            graph: Graph::new(
                [0.0; 1920],
                (255, 255, 255)
            )
        }
    }
}

impl EventHandler for GraphVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let func = |x: i64| (x as f64).powf(2.0);

        self.graph.update_func(func);
        self.graph.lerp_points();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::BLACK
        );

        let params = graphics::DrawParam::new()
            .color(Color::new(
                1.0,
                1.0,
                1.0,
                1.0
            ))
            .dest(Vec2::new(20.0, 20.0));

        let mut points: Vec<Vec2> = Vec::new();
        for (i, p) in self.graph.get_points().iter().enumerate() {
            points.push(Vec2::new(i as f32, *p as f32));
        }

        let line = graphics::Mesh::new_line(
            ctx,
            points.as_slice(),
            2.0,
            Color::BLUE
        )?;

        canvas.draw(&line, params);

        canvas.finish(ctx)
    }
}
