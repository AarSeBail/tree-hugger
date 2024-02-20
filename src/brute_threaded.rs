use std::sync::{Arc, RwLock};
use std::thread;
use crate::graph::LapGraph;
use num::integer::binomial;
use crate::brute::partial_brute_max_recursive;

fn partial_brute_max(m: usize, k: usize, thread_count: usize) -> LapGraph {
    let mut working_graphs = (0..thread_count).map(|_| LapGraph::empty(k)).collect::<Vec<_>>();
    let mut targets = (0..thread_count).map(|_| LapGraph::empty(k)).collect::<Vec<_>>();
    let mut bests: Vec<f64> = vec![0.0; thread_count];

    let mut choices = vec![];

    for i in 0..k {
        for j in (i + 1)..k {
            choices.push((i, j))
        }
    }

    let choices: Arc<[(usize, usize)]> = Arc::from(choices);

    let choice_index = Arc::new(RwLock::new(0usize));

    let mut thread_pool = vec![];

    for i in 0..thread_count {
        let choices_lock = choices.clone();
        let choice_index_lock = choice_index.clone();
        thread_pool.push(
            thread::spawn(move || {
                let mut working_graph = LapGraph::empty(k);
                let mut target_graph = LapGraph::empty(k);
                let mut best = 0.0;

                loop {
                    let index = {
                        if let Ok(mut cid) = choice_index_lock.write() {
                            if *cid >= choices_lock.len() {
                                break
                            }
                            *cid += 1;
                            (*cid).clone()
                        }else {
                            break;
                        }
                    }; // Drop cid
                    let choice = choices_lock[index - 1];
                    working_graph.add_edge(choice.0, choice.1);
                    partial_brute_max_recursive(
                        &mut working_graph,
                        &mut target_graph,
                        &mut best,
                        m - 1,
                        &choices_lock[index..]
                    );
                    working_graph.remove_edge(choice.0, choice.1);
                }

                (best, target_graph)
            })
        )
    }

    let mut res = LapGraph::empty(k);
    let mut best = 0.0;

    for (id, t) in thread_pool.into_iter().enumerate() {
        if let Ok((b, g)) = t.join() {
            println!("Thread {id} had {b}");
            if b > best {
                best = b;
                g.transfer(&mut res);
            }
        }
    }

    res
}

pub fn threaded_brute_max_spanning_trees(m: usize, thread_count: usize) -> LapGraph {
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

    let lower_bound = partial_brute_max(m, h, 1)
        .count_spanning_trees()
        .round() as usize;

    // let lower_bound = 0;

    let mut n = h;

    while binomial(m, n) >= lower_bound && n < m {
        n += 1;
    }

    partial_brute_max(m, n, thread_count)
}
