// pub struct Graph {
//     edges: BTreeSet<(usize, usize)>,
// }

// // edges: {
// //     (1, 2)
// //     (2, 3)
// //     (3, 1)
// //     (3, 4)
// //     (4, 3)
// // }

// // edges.range((3, 0)..(4, 0)).collect() == vec![(3, 1), (3, 4)]

// // Vec
// // HashSet { 5, 2, 100, 23, 1 }
// // BTreeSet { 1, 2, 5, 23, 100 }

// // HashMap { 5: ab, 2: asd, 100: qwe }

// pub struct NodeId(usize);

// pub struct EdgeId(usize);

// impl Graph {
//     fn new_node(&mut self, name: String) -> NodeId {
//         let data = NodeData {
//             name,
//             // nodes: vec![],
//         };
//         let current_length = self.nodes.len();
//         self.nodes.push(data);
//         NodeId(current_length)
//     }

//     fn get_node(&self, node_id: NodeId) -> &NodeData {
//         &self.nodes[node_id.0]
//     }

//     //fn new_edge(node_from: NodeId, node_to: NodeId) -> EdgeId {}
// }

// pub struct NodeData {
//     name: String,
// }

// pub struct EdgeData {
//     name: String,
// }
