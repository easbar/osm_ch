use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::constants::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Way {
    pub source: NodeId,
    pub target: NodeId,
    pub weight: usize,
    #[serde(skip_serializing)]
    pub id: Option<EdgeId>,
    pub contrated_previous: Option<EdgeId>,
    pub contrated_next: Option<EdgeId>,
}

impl PartialOrd for Way {
    fn partial_cmp(&self, other: &Way) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Way {
    fn cmp(&self, other: &Way) -> Ordering {
        self.source
            .cmp(&other.source)
            .then(self.target.cmp(&other.target))
            .then(self.weight.cmp(&other.weight))
            .then(self.contrated_previous.cmp(&other.contrated_previous))
            .then(self.contrated_next.cmp(&other.contrated_next))
    }
}

impl Way {
    /// general constructor
    pub fn new(from: NodeId, to: NodeId, weight: Weight) -> Self {
        Way {
            source: from,
            target: to,
            weight,
            id: None,
            contrated_previous: None,
            contrated_next: None,
        }
    }

    #[allow(dead_code)]
    pub fn test(from: NodeId, to: NodeId, weight: Weight, id: NodeId) -> Self {
        Way {
            source: from,
            target: to,
            weight,
            id: Some(id),
            contrated_previous: None,
            contrated_next: None,
        }
    }
    #[allow(dead_code)]
    pub fn shortcut(
        from: NodeId,
        to: NodeId,
        weight: Weight,
        previous: NodeId,
        next: NodeId,
        id: NodeId,
    ) -> Self {
        Way {
            source: from,
            target: to,
            weight,
            id: Some(id),
            contrated_previous: Some(previous),
            contrated_next: Some(next),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub rank: Rank,
}
