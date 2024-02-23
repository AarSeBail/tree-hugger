#![allow(dead_code)]
mod brute;
mod graph;


use std::cmp::max;
use std::error::Error;
use clap::{Args, Parser, Subcommand};
use crate::graph::LapGraph;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn parse_edge(s: &str) -> Result<(usize, usize), Box<dyn Error + Send + Sync + 'static>>
{
    let paren_1 = s.find('(')
        .ok_or_else(|| format!("Malformed edge"))?;
    let paren_2 = s.find(')')
        .ok_or_else(|| format!("Malformed edge"))?;
    let pos = s.find(',')
        .ok_or_else(|| format!("Malformed edge"))?;
    Ok((s[paren_1+1..pos].parse()?, s[pos + 1..paren_2].parse()?))
}

#[derive(Subcommand)]
enum Commands {
    /// Compute the spanning tree maximizer for some edge count
    #[clap(visible_alias("mst"))]
    MaximizeSpanningTrees {
        edge_count: usize,

        #[arg(short, long)]
        multigraphs: bool
    },

    /// Count the number of spanning trees for a given edge list
    #[clap(visible_alias("count"))]
    CountSpanningTrees {
        #[arg(short = 'E', value_parser = parse_edge)]
        edges: Vec<(usize, usize)>
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::MaximizeSpanningTrees { edge_count, multigraphs}) => {
            let mut res = brute::brute_max_spanning_trees(*edge_count, *multigraphs);
            println!("Graph: {res}");
            println!("Spanning Tree Count: {}", res.count_spanning_trees());
            println!("Regular: {}", res.regular());
        }

        Some(Commands::CountSpanningTrees { edges }) => {
            let mut max_vertex = 0;
            for (a, b) in edges.iter() {
                max_vertex = max(*a, max_vertex);
                max_vertex = max(*b, max_vertex);
            }
            let mut graph = LapGraph::empty(max_vertex + 1);

            for (a, b) in edges.iter() {
                graph.add_edge(*a, *b);
            }

            println!("Spanning Tree Count: {}", graph.count_spanning_trees());
        }

        None => {}
    }
}