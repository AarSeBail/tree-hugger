#[allow(unused)]
mod brute;
mod graph;
mod brute_threaded;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let m = args.last().unwrap().parse::<usize>().unwrap();

    let n = if args.len() > 2 {
        args[1].parse::<usize>().unwrap()
    }else {
        4
    };

    let mut res = brute_threaded::threaded_brute_max_spanning_trees(m, n);

    println!("Graph: {res}");
    println!("# Spanning Trees: {}", res.count_spanning_trees());
    println!("Regular: {}", res.regular());
}
