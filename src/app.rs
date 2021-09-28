use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct App {
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Event {
    KeyPressed(String),
    KeyReleased(String),
    MouseMove(u32, u32),
    Error(String),
}

impl App {
    pub fn push(&mut self, event: &str) {
        let event = event.split_once(":").unwrap();

        let event = match event.0 {
            "p" => Event::KeyPressed(event.1.to_owned()),
            "r" => Event::KeyReleased(event.1.to_owned()),
            "m" => {
                let coords = event.1.split_once(":").unwrap();
                Event::MouseMove(coords.0.parse().unwrap(), coords.1.parse().unwrap())
            }
            _ => todo!(),
        };
        self.events.push(event);
    }

    pub fn how_much_spaces(&self) -> u32 {
        let mut counter = 0;
        let mut is_pressed = false;

        for event in &self.events {
            match event {
                Event::KeyPressed(key) => {
                    if key == "Space" {
                        assert!(!is_pressed);
                        is_pressed = true;
                    }
                }
                Event::KeyReleased(key) => {
                    if key == "Space" {
                        assert!(is_pressed);
                        is_pressed = false;
                        counter += 1;
                    }
                }
                _ => {}
            }
        }

        counter
    }
}

#[test]
fn test_1() {
    let data = r#" { "MouseMove": [123, 234] } "#;

    let ev: Event = serde_json::from_str(data).unwrap();
    panic!("{:?}", ev);
}

#[test]
fn test_some_test() {
    let mut app = App::default();

    app.push("p:Space");
    app.push("r:Space");
    app.push("p:Escape");
    app.push("r:Escape");
    app.push("m:1234:1023");

    // assert_eq!(app.how_much_spaces(), 1);

    assert_eq!(
        app.events,
        vec![
            Event::KeyPressed("Space".to_owned()),
            Event::KeyReleased("Space".to_owned()),
            Event::KeyPressed("Escape".to_owned()),
            Event::KeyReleased("Escape".to_owned()),
            Event::MouseMove(1234, 1023),
        ]
    )
}
