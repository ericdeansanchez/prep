use std::collections::VecDeque;
use std::collections::HashMap;

pub struct Edge {
    vi: u8,
    vj: u8,
}

#[derive(Debug)]
pub struct Graph {
    v: u8,
    e: u8,
    adj: HashMap<u8, Vec<u8>>,
}

impl Graph {
    pub fn new(v: u8) -> Graph {
        Graph {
            v,
            e: 0,
            adj: HashMap::new(),
        }
    }

    pub fn build_graph(v: u8, edges: &Vec<Edge>) -> Graph {
        let mut graph = Graph::new(v);

        for edge in edges {
            graph.adj.insert(edge.vi, Vec::new());
        }

        for edge in edges {
            graph.adj.get_mut(&edge.vi)
                .unwrap()
                .push(edge.vj);
        }
        // each edge, e = {u, v}, is incident on 
        // two vertices u and v
        graph.e = 2 * v;
        graph
    }
}


#[derive(Debug)]
pub struct BreadthFirstPaths {
    marked: HashMap<u8, bool>,
    edge_to: HashMap<u8, u8>,
    source: u8,
}


impl BreadthFirstPaths {
    pub fn new(source: u8) -> BreadthFirstPaths {
        BreadthFirstPaths {
            marked: HashMap::new(),
            edge_to: HashMap::new(),
            source,
        }
    }

    pub fn build_bfp(&mut self, graph: &Graph, source: u8) {
        BreadthFirstPaths::bfs(self, graph, source);
    }

    fn bfs(bfp: &mut BreadthFirstPaths, graph: &Graph, src: u8) {
        let mut queue: VecDeque<u8> = VecDeque::new();

        for k in graph.adj.keys() {
            bfp.marked.insert(*k, false);
        }

        if let Some(source) = bfp.marked.get_mut(&src) {
            *source = true;
        }

        queue.push_back(src);

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();
            let adj = graph.adj.get(&v).unwrap();
            for w in adj {
                if !bfp.marked.get(w).unwrap() {
                    queue.push_back(*w);
                    bfp.marked.insert(*w, true);
                    bfp.edge_to.insert(*w, v);
                }
            }
        }
    }

    pub fn has_path(&self, v: u8) -> bool {
        *self.marked.get(&v).unwrap_or(&false)
    }

    pub fn path_to(&self, v: u8) -> Vec<u8> {
        let mut pv = v;
        let mut path: Vec<u8> = Vec::new();

        if !self.has_path(pv) {
            return path;
        }

        while pv != self.source {
            path.push(pv);
            pv = *self.edge_to.get(&pv).unwrap();
        }
        path.push(self.source);
        path.reverse();
        path
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /*----------------------*
     *  Graph
     *
     *    [1]-----[2]
     *            /|   [5]
     *           / |  /
     *          /  | /
     *       [4]--[3]
     *----------------------*/
    #[test]
    fn test_bfs_graph() {
        let edges: Vec<Edge> = vec![Edge { vi: 1, vj: 2 },
                                    Edge { vi: 2, vj: 3 },
                                    Edge { vi: 2, vj: 4 },
                                    Edge { vi: 3, vj: 2 },
                                    Edge { vi: 3, vj: 4 },
                                    Edge { vi: 3, vj: 5 },
                                    Edge { vi: 4, vj: 3 },
                                    Edge { vi: 5, vj: 3 }];

        let g: Graph = Graph::build_graph(5, &edges);
        let mut bfs_graph: BreadthFirstPaths = BreadthFirstPaths::new(1);
        bfs_graph.build_bfp(&g, 1);

        let left: Vec<u8> = vec![1, 2, 3, 5];
        let right: Vec<u8> = bfs_graph.path_to(5);
        assert_eq!(left, right);


        let left: Vec<u8> = vec![1, 2, 4];
        let right: Vec<u8> = bfs_graph.path_to(4);
        assert_eq!(left, right);
    }
}
