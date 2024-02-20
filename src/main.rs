#[allow(unused)]
mod brute;
mod graph;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let m = args.last().unwrap().parse::<usize>().unwrap();

    let mut res = brute::brute_max_spanning_trees(m);

    println!("{res}");
    println!("{}", res.count_spanning_trees());
}
