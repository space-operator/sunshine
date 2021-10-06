use indradb::*;

use super::{Database, NodeId};

type Node = Vertex;

pub struct NodeCoord {
    x: i32,
    y: i32,
}

impl Database {
    pub fn get_nodes_by_ids(&self, node_ids: Vec<NodeId>) -> Vec<Node> {
        self.datastore
            .transaction()
            .unwrap()
            .get_vertices(SpecificVertexQuery::new(node_ids))
            .unwrap()
    }

    pub fn get_nodes_by_coord(&self, coord: NodeCoord) -> Vec<Node> {
        todo!()
    }
}
