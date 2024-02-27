use crate::graph::{graph_type, LapGraph};
use num::integer::binomial;
use crate::graph::graph_type::Erased;

pub fn partial_brute_max_recursive<T: graph_type::GraphType>(
    working_graph: &mut LapGraph<T>,
    target: &mut LapGraph<T>,
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
    // Lower gates on the values of m are certainly too aggressive
    /*if working_graph.count_components() > m {
        return;
    }*/

    let mut flag = false;

    for (i, (a, b)) in choices.iter().enumerate() {
        // A not-incredible heuristic
        // This marks the site of a dead and buried heuristic
        // Contractions are the future, I've exhausted
        // all reasonable heuristics
        if !working_graph.is_adjacent(*a, *b) {
            if flag {
                continue;
            }
            flag = true;
        }

        working_graph.add_edge(*a, *b);
        if T::MULTI_EDGES {
            partial_brute_max_recursive::<T>(
                working_graph,
                target,
                current_best,
                num_comps,
                m - 1,
                &choices,
            );
        }else {
            partial_brute_max_recursive::<T>(
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

fn partial_brute_max<T: graph_type::GraphType>(m: usize, k: usize) -> LapGraph<T> {
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

    partial_brute_max_recursive::<T>(&mut working_graph, &mut target, &mut best, &mut comp, m, &choices);

    println!("Terminal Graphs Searched: {comp}");

    target
}

pub fn brute_max_spanning_trees(m: usize, multigraphs: bool) -> LapGraph<Erased> {
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

    let mut estimate = if multigraphs {
        partial_brute_max::<graph_type::Multigraph>(m, h).erase_type()
    }else {
        partial_brute_max::<graph_type::Simple>(m, h).erase_type()
    };

    let lower_bound = estimate
        .count_spanning_trees()
        .round() as usize;

    let mut n = h;

    while binomial(m, n) >= lower_bound && n < m {
        n += 1;
    }

    println!("Performing Deductive Search");

    if multigraphs {
        partial_brute_max::<graph_type::Multigraph>(m, n).erase_type()
    }else {
        partial_brute_max::<graph_type::Simple>(m, n).erase_type()
    }
}