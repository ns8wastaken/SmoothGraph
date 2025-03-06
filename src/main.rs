mod graph;
mod visualizer;

use graph::Graph;
use visualizer::GraphVisualizer;

use ggez::ContextBuilder;


fn main() {
    let (
        mut ctx,
        event_loop
    ) = ContextBuilder::new(
        "hello_ggez",
        "ns8"
    )
        .build()
        .unwrap();

    //let graph = Graph::new(
    //    [0.0; 1920],
    //    2.0,
    //    (255, 255, 255)
    //);

    let graph_visualizer = GraphVisualizer::new(&mut ctx);


    ggez::event::run(ctx, event_loop, graph_visualizer);
}
