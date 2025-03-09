use crate::graph::Graph;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, Mesh};
use ggez::event::EventHandler;
use ggez::glam::Vec2;


pub struct GraphVisualizer {
    graph: Graph,
    unit_length: f32,
    graph_origin: Vec2,
    function: fn(f64) -> f64,
}


impl GraphVisualizer {
    pub fn new(_ctx: &mut Context) -> GraphVisualizer {
        let (width, height) = _ctx.gfx.drawable_size();

        GraphVisualizer {
            graph: Graph::new(
                width as usize,
                Color::new(0.0, 0.369, 1.0, 1.0)
            ),
            unit_length: 100.0,
            graph_origin: Vec2::new(0.0, height),
            function: |x: f64| (2.0 * x.sin() * x.cos()) / x.powi(2),
        }
    }

    fn get_screen_y(&self, y: f32) -> f32{
        self.graph_origin.y - (y * self.unit_length)
    }

    fn draw_grid(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        draw_param: graphics::DrawParam,
        width: f32,
        height: f32
    ) -> Result<(), ggez::GameError> {
        let mut i: i32 = self.graph_origin.y as i32;

        // Up
        while i > 0 {
            let line = Mesh::new_line(
                ctx,
                &[
                    Vec2::new(0.0, i as f32),
                    Vec2::new(width, i as f32)
                ],
                1.0,
                Color::new(0.5, 0.5, 0.5, 1.0),
            )?;

            canvas.draw(&line, draw_param);

            i -= self.unit_length as i32;
        }

        // Down
        i = self.graph_origin.y as i32;
        while i < height as i32 {
            let line = Mesh::new_line(
                ctx,
                &[
                    Vec2::new(0.0, i as f32),
                    Vec2::new(width, i as f32)
                ],
                1.0,
                Color::new(0.5, 0.5, 0.5, 1.0),
            )?;

            canvas.draw(&line, draw_param);

            i += self.unit_length as i32;
        }

        // Left
        i = self.graph_origin.x as i32;
        while i > 0 {
            let line = Mesh::new_line(
                ctx,
                &[
                    Vec2::new(i as f32, 0.0),
                    Vec2::new(i as f32, height)
                ],
                1.0,
                Color::new(0.5, 0.5, 0.5, 1.0),
            )?;

            canvas.draw(&line, draw_param);

            i -= self.unit_length as i32;
        }

        // Right
        i = self.graph_origin.x as i32;
        while i < width as i32 {
            let line = Mesh::new_line(
                ctx,
                &[
                    Vec2::new(i as f32, 0.0),
                    Vec2::new(i as f32, height)
                ],
                1.0,
                Color::new(0.5, 0.5, 0.5, 1.0),
            )?;

            canvas.draw(&line, draw_param);

            i += self.unit_length as i32;
        }

        Ok(())
    }
}


impl EventHandler for GraphVisualizer {
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> Result<(), ggez::GameError> {
        if y > 0.0 {
            self.unit_length = 200.0f32.min(self.unit_length + 10.0);
        } else if y < 0.0 {
            self.unit_length = 20.0f32.max(self.unit_length - 10.0);
        }
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.graph.update_func(
            self.graph_origin.x as f64,
            self.unit_length as f64,
            self.function
        );
        self.graph.lerp_points();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::BLACK
        );

        // Move graph
        let mouse_delta = ctx.mouse.delta();

        if ctx.mouse.button_pressed(ggez::event::MouseButton::Left) {
            self.graph_origin.x += mouse_delta.x;
            self.graph_origin.y += mouse_delta.y;
        }
        // ==========

        let (width, height) = ctx.gfx.drawable_size();

        let params = graphics::DrawParam::new()
            .dest(Vec2::new(0.0, 0.0));

        self.draw_grid(ctx, &mut canvas, params, width, height)?;

        // Draw graph
        let mut points: Vec<Vec2> = Vec::new();
        for (i, p) in self.graph.get_points().iter().enumerate() {
            points.push(
                Vec2::new(
                    i as f32,
                    self.get_screen_y(*p as f32)
                )
            );
        }

        let line = Mesh::new_line(
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
