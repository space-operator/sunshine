mod app;
mod database;
mod event;
mod graph;
mod router;

use app::App;
use app::AppEvent;
use event::Event;
use indradb::Type;
use serde_json::json;
use std::io::{self, Read};

use crate::database::Database;

pub fn main() -> io::Result<()> {
    let database = Database::new();

    let vertex_properties = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let new_vertex = database.create_vertex(
        &vertex_properties,
        Type::new("data").expect("creating vertex type"),
    );
    println!("{:?}", new_vertex);

    let stdin = io::stdin();
    let mut app = App::default();

    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer == "exit\n" {
            break;
        }

        app.push(&buffer.as_str());

        // let ev = serde_json::from_str(&buffer);
        // match ev {
        //     Err(err) => {
        //         // parse as en error
        //         app.events.push(Event::Error(err.to_string()));
        //     }
        //     Ok(ev) => {
        //         app.events.push(ev);
        //     }
        // }

        //app.push(buffer.trim());

        println!("{:?}", app);
    }

    Ok(())

    // let vertex_type = indradb::Type::new("type1").unwrap();

    // let mem = MemoryDatastore::create("temp").expect("err");
    // let transaction = mem.transaction().expect("starting transaction");
    // let vertex1 = Vertex::new(vertex_type.clone());
    // let vertex2 = Vertex::new(vertex_type);

    // transaction
    //     .create_vertex(&vertex1)
    //     .expect("Creating vertex 1");
    // transaction
    //     .create_vertex(&vertex2)
    //     .expect("Creating vertex 2");
    // transaction
    //     .set_vertex_properties(
    //         indradb::VertexPropertyQuery::new(
    //             SpecificVertexQuery::single(vertex1.id).into(),
    //             String::from("contact_info"),
    //         ),
    //         &json!({
    //             "name": "John Doe",
    //             "age": 43,
    //             "phones": [
    //                 "+44 1234567",
    //                 "+44 2345678"
    //             ]
    //         }),
    //     )
    //     .expect("setting vertex properties");

    // let etype = Type::new("edge_type").unwrap();
    // let edge_key = EdgeKey::new(vertex1.id, etype, vertex2.id);

    // transaction.create_edge(&edge_key).expect("Creating edge");
}
