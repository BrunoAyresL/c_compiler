struct ControlFlowGraph {
    nodes: Vec<Block>,
    edges: Vec<Vec<usize>>,
}

struct Block {
    first: usize,
    last: usize,
}