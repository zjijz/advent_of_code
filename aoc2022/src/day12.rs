use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::path::Path;
use std::str::FromStr;

use anyhow::Context as _;

type NodeId = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeClass {
    Start,
    End,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    id: NodeId,
    elevation: char,
    class: NodeClass,
}

impl Node {
    fn new(i: usize, j: usize, elevation: char, class: NodeClass) -> Self {
        Self {
            id: (i, j),
            elevation,
            class,
        }
    }
}

struct Graph<const PART: usize> {
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<NodeId, Vec<NodeId>>,
    start: NodeId,
    end: NodeId,
}

impl<const PART: usize> Graph<PART> {
    /// For PART == 1, this ensures next is at most 1 elevation taller than curr.
    /// For Part == 2, this ensures next is at most 1 elevation shorter than curr.
    fn elevation_cmp(curr: &Node, next: &Node) -> bool {
        if PART == 1 {
            (next.elevation as usize) <= (curr.elevation as usize + 1)
        } else if PART == 2 {
            (curr.elevation as usize) <= (next.elevation as usize + 1)
        } else {
            // TODO
            false
        }
    }
}

impl<const PART: usize> FromStr for Graph<PART> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        // Build all nodes from the str.
        //
        // Also record start and end Nodes.
        for (row, line) in s.lines().enumerate() {
            for (col, elev) in line.chars().enumerate() {
                if elev == 'S' {
                    nodes.insert((row, col), Node::new(row, col, 'a', NodeClass::Start));
                    start = (row, col);
                } else if elev == 'E' {
                    nodes.insert((row, col), Node::new(row, col, 'z', NodeClass::End));
                    end = (row, col);
                } else {
                    nodes.insert((row, col), Node::new(row, col, elev, NodeClass::None));
                }
            }
        }

        // Iterate over all nodes and use stored elevations to produce the edges.
        let mut edges = HashMap::new();
        for ((row, col), node) in nodes.iter() {
            let entry: &mut Vec<_> = edges.entry((*row, *col)).or_default();

            // Left.
            if *col > 0 {
                if let Some(next) = nodes.get(&(*row, *col - 1)) {
                    if Self::elevation_cmp(node, next) {
                        entry.push((*row, *col - 1));
                    }
                }
            }

            // Top.
            if *row > 0 {
                if let Some(next) = nodes.get(&(*row - 1, *col)) {
                    if Self::elevation_cmp(node, next) {
                        entry.push((*row - 1, *col));
                    }
                }
            }

            // Right.
            if let Some(next) = nodes.get(&(*row, *col + 1)) {
                if Self::elevation_cmp(node, next) {
                    entry.push((*row, *col + 1));
                }
            }

            // Bottom.
            if let Some(next) = nodes.get(&(*row + 1, *col)) {
                if Self::elevation_cmp(node, next) {
                    entry.push((*row + 1, *col));
                }
            }
        }

        Ok(Self {
            nodes,
            edges,
            start,
            end,
        })
    }
}

struct SearchNode<'a> {
    node: &'a Node,
    distance: usize,
}

impl<'a> std::fmt::Debug for SearchNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchNode")
            .field("node", &self.node)
            .field("distance", &self.distance)
            .finish()
    }
}

impl<'a> From<&'a Node> for SearchNode<'a> {
    fn from(value: &'a Node) -> Self {
        SearchNode {
            node: value,
            distance: 0,
        }
    }
}

impl<'a> PartialEq for SearchNode<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.distance == other.distance
    }
}

impl<'a> Eq for SearchNode<'a> {}

impl<'a> PartialOrd for SearchNode<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<'a> Ord for SearchNode<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl<const PART: usize> Graph<PART> {
    /// Dijkstra's Algorithm.
    fn shortest_path(&self) -> anyhow::Result<usize> {
        let start = self
            .nodes
            .get(&self.start)
            .map(SearchNode::from)
            .map(Reverse)
            .ok_or_else(|| anyhow::anyhow!("No start node available."))?;
        let mut min_heap: BinaryHeap<Reverse<SearchNode<'_>>> = vec![start].into_iter().collect();

        let mut costs: HashMap<NodeId, usize> =
            self.nodes.iter().map(|(id, _)| (*id, usize::MAX)).collect();

        while let Some(curr) = min_heap.pop() {
            let curr_distance = curr.0.distance;

            if curr.0.node.class == NodeClass::End {
                return Ok(curr_distance);
            }

            let curr_id = &curr.0.node.id;

            if curr_distance
                > *costs
                    .get(curr_id)
                    .ok_or_else(|| anyhow::anyhow!("Could not find cost with id {:#?}", curr_id))?
            {
                continue;
            }

            if let Some(edges) = self.edges.get(curr_id) {
                for edge in edges {
                    let mut node = SearchNode::from(self.nodes.get(edge).ok_or_else(|| {
                        anyhow::anyhow!("Could not find node with id {:#?}.", edge)
                    })?);
                    node.distance = curr_distance + 1;

                    if node.distance
                        < *costs.get(edge).ok_or_else(|| {
                            anyhow::anyhow!("Could not find cost with id {:#?}.", edge)
                        })?
                    {
                        costs.insert(*edge, node.distance);
                        min_heap.push(Reverse(node));
                    }
                }
            }
        }
        Err(anyhow::anyhow!("The graph has no end node specified!"))
    }

    /// Dijkstra's Algorithm to find the shortest path tree from end node to and node with elevation 'a'.
    fn min_path_to_end(&self) -> anyhow::Result<usize> {
        let end = self
            .nodes
            .get(&self.end)
            .map(SearchNode::from)
            .map(Reverse)
            .ok_or_else(|| anyhow::anyhow!("No start node available."))?;
        let mut min_heap: BinaryHeap<Reverse<SearchNode<'_>>> = vec![end].into_iter().collect();

        let mut costs: HashMap<NodeId, usize> =
            self.nodes.iter().map(|(id, _)| (*id, usize::MAX)).collect();

        while let Some(curr) = min_heap.pop() {
            let curr_distance = curr.0.distance;
            let curr_id = &curr.0.node.id;

            if curr.0.node.elevation == 'a'
                || curr_distance
                    > *costs.get(curr_id).ok_or_else(|| {
                        anyhow::anyhow!("Could not find cost with id {:#?}", curr_id)
                    })?
            {
                continue;
            }

            if let Some(edges) = self.edges.get(curr_id) {
                for edge in edges {
                    let mut node = SearchNode::from(self.nodes.get(edge).ok_or_else(|| {
                        anyhow::anyhow!("Could not find node with id {:#?}.", edge)
                    })?);
                    node.distance = curr_distance + 1;

                    if node.distance
                        < *costs.get(edge).ok_or_else(|| {
                            anyhow::anyhow!("Could not find cost with id {:#?}.", edge)
                        })?
                    {
                        costs.insert(*edge, node.distance);
                        min_heap.push(Reverse(node));
                    }
                }
            }
        }

        self.nodes
            .values()
            .filter_map(|Node { id, elevation, .. }| {
                if *elevation == 'a' {
                    costs.get(id).cloned()
                } else {
                    None
                }
            })
            .min()
            .ok_or_else(|| anyhow::anyhow!("There were no squares with elevation a in the graph!"))
    }
}

pub async fn part1(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;
    let graph = Graph::<1>::from_str(&contents).context("Could not construct graph from input.")?;
    graph.shortest_path()
}

pub async fn part2(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;
    let graph = Graph::<2>::from_str(&contents).context("Could not construct graph from input.")?;
    graph.min_path_to_end()
}
