pub type NodeId = usize;
pub type EdgeId = usize;
pub type Weight = usize;
pub type Rank = usize;

pub const INVALID_NODE: NodeId = std::usize::MAX;
pub const INVALID_EDGE: NodeId = std::usize::MAX;
pub const WEIGHT_MAX: Weight = std::usize::MAX;
pub const INVALID_RANK: Rank = std::usize::MAX;

pub const DIST_MULTIPLICATOR: usize = 262144; // 2^18
