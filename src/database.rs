use indradb::{
    Datastore, EdgeKey, MemoryDatastore, MemoryTransaction, SpecificVertexQuery, Transaction, Type,
    Vertex,
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
    pub datastore: MemoryDatastore,
    pub transaction: MemoryTransaction,
}

impl Database {
    pub fn new() -> Database {
        let datastore = Database::initialize_datastore();
        let transaction = Database::create_transaction(&datastore);
        Database {
            datastore: datastore,
            transaction: transaction,
        }
    }

    fn initialize_datastore() -> MemoryDatastore {
        MemoryDatastore::create("temp").expect("Initialize datastore")
    }

    fn create_transaction(datastore: &MemoryDatastore) -> indradb::MemoryTransaction {
        datastore.transaction().expect("Create transaction")
    }

    //
    // Create a vertex
    //
    pub fn create_vertex(
        &self,
        vertex_properties: &serde_json::Value,
        vertex_type: Type,
    ) -> Vertex {
        let new_vertex = Vertex::new(vertex_type.clone());

        let created = self
            .transaction
            .create_vertex(&new_vertex)
            .expect("Creating vertex");

        assert!(created, "Failed to add vertex to datastore");

        self.transaction
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
}
