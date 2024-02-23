#![allow(dead_code)]
mod brute;
mod graph;


// m = 20    S = 33000
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let m = args.last().unwrap().parse::<usize>().unwrap();

    let n = if args.len() > 2 {
        args[1].parse::<usize>().unwrap()
    }else {
        4
    };

    let mut res = brute::brute_multigraph_spanning_trees(m);

    println!("Graph: {res}");
    println!("# Spanning Trees: {}", res.count_spanning_trees());
    println!("Regular: {}", res.regular());
}