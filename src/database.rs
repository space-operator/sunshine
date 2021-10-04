use indradb::{
    Datastore, Edge, EdgeKey, EdgePropertyQuery, MemoryDatastore, MemoryTransaction,
    RangeVertexQuery, SpecificEdgeQuery, SpecificVertexQuery, Transaction, Type, Vertex,
    VertexQuery,
};
use lazy_static::lazy_static;
use serde_json::json;

// how to use in another file?
lazy_static! {
    pub static ref TYPE_DATA: Type = Type::new("data").expect("creating vertex type");
    pub static ref WIDGET_TYPE: Type = Type::new("widget").expect("creating vertex type");
}

#[derive(Debug)]
pub struct Database {
    pub datastore: MemoryDatastore,
}

impl Database {
    pub fn new() -> Database {
        let datastore = MemoryDatastore::create("temp").expect("Initialize datastore");
        Database {
            datastore: datastore,
        }
    }

    //
    // Create a vertex
    //
    pub fn create_vertex(
        &self,
        vertex_type: Type,
        vertex_properties: &serde_json::Value,
    ) -> Vertex {
        let new_vertex = Vertex::new(vertex_type.clone());

        let new_vertex = match self
            .datastore
            .transaction()
            .unwrap()
            .create_vertex(&new_vertex)
        {
            Ok(_) => new_vertex,
            Err(_) => {
                todo!()
            }
        };

        match self.datastore.transaction().unwrap().set_vertex_properties(
            indradb::VertexPropertyQuery::new(
                SpecificVertexQuery::single(new_vertex.id).into(),
                String::from("properties"),
            ),
            vertex_properties,
        ) {
            Ok(_) => new_vertex,
            Err(_) => {
                todo!()
            }
        }
    }

    pub fn create_edge(
        &self,
        from_vertex: Vertex,
        to_vertex: Vertex,
        edge_type: Type,
        edge_properties: &serde_json::Value,
    ) -> EdgeKey {
        let new_edge_key = EdgeKey::new(from_vertex.id, edge_type, to_vertex.id);

        let new_edge_key = match self
            .datastore
            .transaction()
            .unwrap()
            .create_edge(&new_edge_key)
        {
            Ok(_) => new_edge_key,
            Err(_) => {
                todo!()
            }
        };

        match self.datastore.transaction().unwrap().set_edge_properties(
            EdgePropertyQuery::new(
                SpecificEdgeQuery::single(new_edge_key.clone()).into(),
                String::from("edge_name"),
            ),
            edge_properties,
        ) {
            Ok(_) => new_edge_key,
            Err(_) => {
                todo!()
            }
        }
    }
}

#[test]
fn test_database() {
    let dummy_database = Database::new();
    dbg!(&dummy_database);
    let node_type = Type::new("data").unwrap();

    let vertex_1_props = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let vertex_1 = dummy_database.create_vertex(node_type.clone(), &vertex_1_props);

    let vertex_2_props = json!({
        "name": "Jane Doe",
        "age": 45,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let vertex_2 = dummy_database.create_vertex(node_type.clone(), &vertex_1_props);
    dbg!(&vertex_1);
    dbg!(&vertex_2);
    dbg!(&dummy_database);

    let edge_props = json!({
        "relationship": "spouse",

    });
    let edge_type = Type::new("data").unwrap();
    dummy_database.create_edge(vertex_1, vertex_2, edge_type.clone(), &edge_props);
    dbg!(&dummy_database);

    let query: VertexQuery = RangeVertexQuery::new().t(node_type).into();
    let result = dummy_database
        .datastore
        .transaction()
        .unwrap()
        .get_vertices(query);
    dbg!(result);
}
