mod autowork;
mod backend;
mod error;
mod execute;
mod generater;
mod graph;
mod logger;
mod package_manage;
mod parser;
mod vars;

use petgraph::graph::Graph;
use petgraph_evcxr::draw_graph;

fn init_rkos_builder(config_store_path: String) {
    //取得默认shell的配置文件，写入当前工作目录
}

fn main() {
    let mut ring: Graph<&str, &str> = Graph::new();
    let item1 = ring.add_node("a");
    let item2 = ring.add_node("b");
    let item3 = ring.add_node("c");
    let item4 = ring.add_node("d");
    let item5 = ring.add_node("f");
    ring.add_edge(item1, item2, "");
    ring.add_edge(item2, item3, "");
    ring.add_edge(item3, item1, "");
    draw_graph(&ring);

    //    println!("new_P {}", new_p.file_name);
    println!("Hello, world!");
}
