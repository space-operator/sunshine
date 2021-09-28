use indradb::{
    Datastore, EdgeKey, MemoryDatastore, SpecificVertexQuery, Transaction, Type, Vertex,
};
use lazy_static::lazy_static;
use serde_json::json;

lazy_static! {
    static ref data_type: Type = Type::new("data").expect("creating vertex type");
    static ref widget_type: Type = Type::new("widget").expect("creating vertex type");
}

#[derive(Debug)]
pub struct Database {
    datastore: MemoryDatastore,
    transaction: indradb::MemoryTransaction,
}

impl Database {
    pub fn new(&mut self) -> Database {
        let datastore = self.initialize_datastore();
        self.transaction = self.create_transaction(&datastore);
        self
    }

    fn initialize_datastore(&mut self) -> MemoryDatastore {
        MemoryDatastore::create("temp").expect("Initialize datastore")
    }

    fn create_transaction(&mut self, datastore: &MemoryDatastore) -> indradb::MemoryTransaction {
        datastore.transaction().expect("Create transaction")
    }
}

fn create_vertex(
    datastore: &MemoryDatastore,
    transaction: &indradb::MemoryTransaction,
    vertex_properties: &serde_json::Value,
    vertex_type: Type,
) -> Vertex {
    let new_vertex = Vertex::new(vertex_type.clone());

    let created = transaction
        .create_vertex(&new_vertex)
        .expect("Creating vertex");

    assert!(created, "Failed to add vertex to datastore");

    transaction
        .set_vertex_properties(
            indradb::VertexPropertyQuery::new(
                SpecificVertexQuery::single(new_vertex.id).into(),
                String::from("properties"),
            ),
            vertex_properties,
        )
        .expect("setting vertex properties");

    new_vertex
}
