#[cfg(test)]
mod exercise;

pub fn main() {}

// use serde_json::json;
// use std::io::{self, Read};
// pub fn main() -> io::Result<()> {
//     let stdin = io::stdin();
//     let mut app = App::default();

//     loop {
//         let mut buffer = String::new();
//         stdin.read_line(&mut buffer)?;
//         if buffer == "exit\n" {
//             break;
//         }

//         app.push(&buffer.as_str());

//         // let ev = serde_json::from_str(&buffer);
//         // match ev {
//         //     Err(err) => {
//         //         // parse as en error
//         //         app.events.push(Event::Error(err.to_string()));
//         //     }
//         //     Ok(ev) => {
//         //         app.events.push(ev);
//         //     }
//         // }

//         //app.push(buffer.trim());

//         println!("{:?}", app);
//     }

//     Ok(())
// }
