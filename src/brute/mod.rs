use crate::graph::LapGraph;
use num::integer::binomial;

pub fn partial_brute_max_recursive<const MULTIGRAPH: bool>(
    working_graph: &mut LapGraph,
    target: &mut LapGraph,
    current_best: &mut f64,
    num_comps: &mut usize,
    m: usize,
    choices: &[(usize, usize)],
) {
    if m == 0 {
        *num_comps += 1;
        let val = working_graph.count_spanning_trees();
        if val > *current_best {
            *current_best = val;
            working_graph.transfer(target);
        }
        return;
    }

    // Bad heuristic, it actually makes it slower
    /*if working_graph.count_components() > m + 1 {
        return;
    }*/

    let mut flag = false;

    for (i, (a, b)) in choices.iter().enumerate() {
        // An incredible heuristic
        if working_graph.is_adjacent(*a, *b) {
            if flag {
                continue;
            }
            flag = true
        }

        working_graph.add_edge(*a, *b);
        if MULTIGRAPH {
            partial_brute_max_recursive::<MULTIGRAPH>(
                working_graph,
                target,
                current_best,
                num_comps,
                m - 1,
                &choices,
            );
        }else {
            partial_brute_max_recursive::<MULTIGRAPH>(
                working_graph,
                target,
                current_best,
                num_comps,
                m - 1,
                &choices[(i + 1)..],
            );
        }
        working_graph.remove_edge(*a, *b);
    }
}

fn partial_brute_max(m: usize, k: usize, multigraph: bool) -> LapGraph {
    println!("Searching Complexity: K_{k}");
    let mut working_graph = LapGraph::empty(k);
    let mut target = LapGraph::empty(k);
    let mut best: f64 = 0.0;

    let mut choices = vec![];

    for i in 0..k {
        for j in (i + 1)..k {
            choices.push((i, j))
        }
    }

    let mut comp = 0;

    if multigraph {
        partial_brute_max_recursive::<true>(&mut working_graph, &mut target, &mut best, &mut comp, m, &choices);
    }else {
        partial_brute_max_recursive::<false>(&mut working_graph, &mut target, &mut best, &mut comp, m, &choices);
    }

    println!("Terminal Graphs Searched: {comp}");

    target
}

pub fn brute_max_spanning_trees(m: usize, multigraphs: bool) -> LapGraph {
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

    println!("Performing Heuristic Search");

    let lower_bound = partial_brute_max(m, h, multigraphs)
        .count_spanning_trees()
        .round() as usize;

    // let lower_bound = 0;

    let mut n = h;

    while binomial(m, n) >= lower_bound && n < m {
        n += 1;
    }

    println!("Performing Deductive Search");

    partial_brute_max(m, n, multigraphs)
}