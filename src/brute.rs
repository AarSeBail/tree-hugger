use crate::graph::LapGraph;
use num::integer::binomial;

fn partial_brute_max_recursive(
    working_graph: &mut LapGraph,
    target: &mut LapGraph,
    current_best: &mut f64,
    m: usize,
    choices: &[(usize, usize)],
) {
    if m == 0 {
        let val = working_graph.count_spanning_trees();
        if val > *current_best {
            *current_best = val;
            working_graph.transfer(target);
        }
        return;
    }

    for (i, (a, b)) in choices.iter().enumerate() {
        working_graph.add_edge(*a, *b);
        partial_brute_max_recursive(
            working_graph,
            target,
            current_best,
            m - 1,
            &choices[(i + 1)..],
        );
        working_graph.remove_edge(*a, *b);
    }
}

fn partial_brute_max(m: usize, k: usize) -> LapGraph {
    let mut working_graph = LapGraph::empty(k);
    let mut target = LapGraph::empty(k);
    let mut best: f64 = 0.0;

    let mut choices = vec![];

    for i in 0..k {
        for j in (i + 1)..k {
            choices.push((i, j))
        }
    }

    partial_brute_max_recursive(&mut working_graph, &mut target, &mut best, m, &choices);

    target
}

pub fn brute_max_spanning_trees(m: usize) -> LapGraph {
    // This heuristic actually fails for numbers under 9, but that is of minor consequence
    // Regarding its efficacy, however, it is worth noting that
    // During testing, m=8 performed in 8 seconds
    // While m=9 performed in 0.7 seconds
    let lower_bound = partial_brute_max(m, (m + 1) / 2)
        .count_spanning_trees()
        .round() as usize;

    let mut n = m / 2;

    while binomial(m, n) >= lower_bound && n < m {
        n += 1;
    }

    partial_brute_max(m, n)
}
