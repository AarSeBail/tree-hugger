use crate::graph::LapGraph;
use num::integer::binomial;

pub fn partial_brute_max_recursive(
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
    /* Cheaply find a lower bound, then use it to choose
       a vertex count to search with.
       For m=9 and m=10, both search on K_7 and take
       0.84 and 1.05 seconds respectively.
       For m=11, the search occurs on K_8,
       and takes 1 minute and 20 seconds.
       Unfortunately, the maximizer for m=11
       has 7 vertices, so we are doing more
       work than is necessary.
    */
    let h = if m < 7 {
        (m + 4)/2
    }else {
        (m + 2)/2
    };

    let lower_bound = partial_brute_max(m, h)
        .count_spanning_trees()
        .round() as usize;

    // let lower_bound = 0;

    let mut n = h;

    while binomial(m, n) >= lower_bound && n < m {
        n += 1;
    }

    partial_brute_max(m, n)
}
