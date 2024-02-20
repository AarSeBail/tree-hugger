use crate::graph::LapGraph;

mod brute;
mod utils;
mod graph;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let M = args.last().unwrap().parse::<usize>().unwrap();

    let mut A = LapGraph::<true>::empty(M);

    A.add_edge(0, 2);

    println!("{A}");

    // let mut L = A.eigenvalues();

    A.remove_edge(0, 2);

    println!("{A}");

    // println!("{L}");

    // L = A.eigenvalues();

    // println!("{L}");
}
