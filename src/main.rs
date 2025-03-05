mod graph;

use graph::Graph;


fn main() {
    let graph = Graph::new(
        [0.0; 1920],
        2.0,
        (255, 255, 255)
    );

    println!("{}", graph.get_points()[0]);
}
