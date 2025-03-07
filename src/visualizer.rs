use crate::graph::Graph;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;
use ggez::glam::Vec2;

pub struct GraphVisualizer {
    graph: Graph,
    unit_length: f32,
    graph_origin: Vec2,
}

impl GraphVisualizer {
    pub fn new(_ctx: &mut Context) -> GraphVisualizer {
        GraphVisualizer {
            graph: Graph::new(
                [0.0; 1920],
                Color::new(0.0, 0.369, 1.0, 1.0)
            ),
            unit_length: 20.0,
            graph_origin: _ctx.gfx.drawable_size().into(),
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

        let (width, height) = ctx.gfx.drawable_size();

        let params = graphics::DrawParam::new()
            .dest(Vec2::new(0.0, 0.0));

        // Draw grid
        let mut i: i32 = self.graph_origin.x as i32;

        // Up
        while i != 0 {
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(0.0, i as f32),
                    Vec2::new(width, i as f32)
                ],
                1.0,
                Color::WHITE
            )?;

            canvas.draw(&line, params);

            i -= self.unit_length as i32;
        }
        // =========

        // Draw graph
        let mut points: Vec<Vec2> = Vec::new();
        for (i, p) in self.graph.get_points().iter().enumerate() {
            points.push(
                Vec2::new(
                    i as f32,
                    height - (*p as f32) / self.unit_length
                )
            );
        }

        let line = graphics::Mesh::new_line(
            ctx,
            points.as_slice(),
            4.0,
            self.graph.color,
        )?;

        canvas.draw(&line, params);
        // ==========

        canvas.finish(ctx)
    }
}
