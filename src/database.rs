use indradb::{
    Datastore, EdgeKey, MemoryDatastore, SpecificVertexQuery, Transaction, Type, Vertex,
};
use lazy_static::lazy_static;
use serde_json::json;

// how to use in another file?
lazy_static! {
    pub static ref DATA_TYPE: Type = Type::new("data").expect("creating vertex type");
    pub static ref WIDGET_TYPE: Type = Type::new("widget").expect("creating vertex type");
}

#[derive(Debug)]
pub struct Database {
    pub datastore: Option<MemoryDatastore>,
    pub transaction: Option<indradb::MemoryTransaction>,
}

impl Database {
    pub fn init() -> Database {
        let mut database = Database {
            datastore: None,
            transaction: None,
        };
        let datastore = database.initialize_datastore();
        let transaction = database.create_transaction(&datastore);
        Database {
            datastore: Some(datastore),
            transaction: Some(transaction),
        }
    }

    fn initialize_datastore(&mut self) -> MemoryDatastore {
        MemoryDatastore::create("temp").expect("Initialize datastore")
    }

    fn create_transaction(&mut self, datastore: &MemoryDatastore) -> indradb::MemoryTransaction {
        datastore.transaction().expect("Create transaction")
    }
}

//
// Create a vertex
//
pub fn create_vertex(
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
