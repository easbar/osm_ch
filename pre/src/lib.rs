mod bidijkstra;
mod constants;
mod contraction;
mod dijkstra;
mod graph_helper;
mod min_heap;
mod offset;
mod ordering;
mod structs;
mod visited_list;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::sync::RwLock;
use std::time::Instant;

pub use crate::constants::*;
pub use crate::structs::*;
use crate::visited_list::*;

#[derive(Serialize, Deserialize)]
pub struct Output {
    nodes: Vec<Node>,
    edges: Vec<Way>,
    up_offset: Vec<EdgeId>,
    down_offset: Vec<EdgeId>,
    down_index: Vec<EdgeId>,
}

pub fn build_ch(mut nodes: Vec<Node>, mut edges: Vec<Way>) -> Output {
    let mut up_offset = Vec::<EdgeId>::new();
    let mut down_offset = Vec::<EdgeId>::new();

    let amount_nodes = nodes.len();
    // generate offset arrays
    let mut down_index =
        offset::generate_offsets(&mut edges, &mut up_offset, &mut down_offset, amount_nodes);

    println!("original #nodes: {:?}", nodes.len());
    println!("original #edges: {:?}", edges.len());

    // contraction hierarchies
    contraction::run_contraction(
        &mut nodes,
        &mut edges,
        &mut up_offset,
        &mut down_offset,
        &mut down_index,
    );

    println!("new #nodes: {:?}", nodes.len());
    println!("new #edges: {:?}", edges.len());

    Output {
        nodes,
        edges,
        up_offset,
        down_index,
        down_offset,
    }
}

impl Output {
    pub fn query(&self, start: NodeId, end: NodeId) -> Option<(Vec<NodeId>, f32)> {
        // TODO Cache this per thread
        let mut dijkstra = bidijkstra::BiDijkstra::new(self.nodes.len());
        dijkstra.find_path(
            start,
            end,
            &self.nodes,
            &self.edges,
            &self.up_offset,
            &self.down_offset,
            &self.down_index,
        )
    }
}
