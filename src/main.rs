mod graph;
mod visualizer;

use ggez::{conf::WindowSetup, conf::NumSamples, ContextBuilder};
use visualizer::GraphVisualizer;

fn main() {
    let (
        mut ctx,
        event_loop
    ) = ContextBuilder::new(
        "hello_ggez",
        "ns8"
    )
        .window_setup(WindowSetup {
            title: "SmoothGraph".to_string(),
            samples: NumSamples::One,
            vsync: false,
            icon: "".to_string(),
            srgb: true
        })
        .build()
        .unwrap();

    let graph_visualizer = GraphVisualizer::new(&mut ctx);

    ggez::event::run(ctx, event_loop, graph_visualizer);
}
