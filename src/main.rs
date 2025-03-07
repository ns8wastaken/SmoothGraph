mod graph;
mod visualizer;

use ggez::ContextBuilder;
use ggez::conf::{WindowSetup, WindowMode, NumSamples};
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
        .window_mode(WindowMode {
            width: 800.0,
            height: 800.0,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 1.0,
            max_width: 0.0,
            min_height: 1.0,
            max_height: 0.0,
            resizable: false,
            visible: true,
            transparent: false,
            resize_on_scale_factor_change: false,
            logical_size: None,
        })
        .build()
        .unwrap();

    let graph_visualizer = GraphVisualizer::new(&mut ctx);

    ggez::event::run(ctx, event_loop, graph_visualizer);
}
