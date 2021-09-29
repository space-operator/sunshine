mod app;
mod database;
mod router;

use app::App;
use app::Event;
use database::create_vertex;
use indradb::Type;
use serde_json::json;
use std::io::{self, Read};

use crate::database::Database;

pub fn main() -> io::Result<()> {
    let database = Database::init();

    let vertex_properties = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let new_vertex = create_vertex(
        &database.transaction.unwrap(),
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

/*
// https://codeandbitters.com/lets-build-a-parser/

use escape8259::unescape;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit0, digit1, one_of},
    combinator::{map, map_res, opt, recognize, value},
    multi::many0,
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(PartialEq, Debug, Clone)]
enum Node {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Str(String),
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct JsonNull {}

fn json_bool(input: &str) -> IResult<&str, Node> {
    alt((
        value(Node::Bool(false), tag("false")),
        value(Node::Bool(true), tag("true")),
    ))(input)
}

fn json_null(input: &str) -> IResult<&str, Node> {
    value(Node::Null, tag("null"))(input)
}

fn digit1to9(input: &str) -> IResult<&str, char> {
    one_of("123456789")(input)
}

fn uint(input: &str) -> IResult<&str, &str> {
    alt((tag("0"), recognize(pair(digit1to9, digit0))))(input)
}

fn json_integer(input: &str) -> IResult<&str, Node> {
    let parser = recognize(pair(opt(tag("-")), uint));

    map(parser, |s| {
        let n = s.parse::<i64>().unwrap();
        Node::Integer(n)
    })(input)
}

fn frac(input: &str) -> IResult<&str, &str> {
    recognize(pair(tag("."), digit1))(input)
}

fn exp(input: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("e"), opt(alt((tag("-"), tag("+")))), digit1)))(input)
}

fn json_float(input: &str) -> IResult<&str, Node> {
    let parser = recognize(tuple((
        opt(tag("-")),
        uint,
        alt((recognize(pair(frac, opt(exp))), exp)),
        exp,
    )));

    map(parser, |s| {
        let n = s.parse::<f64>().unwrap();
        Node::Float(n)
    })(input)
}

fn json_literal(input: &str) -> IResult<&str, Node> {
    alt((json_float, json_integer, json_bool, json_null))(input)
}

fn is_nonescaped_string_char(c: char) -> bool {
    let cv = c as u32;
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C)
}

fn nonescaped_string(input: &str) -> IResult<&str, &str> {
    take_while1(is_nonescaped_string_char)(input)
}

fn escape_code(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        tag("\\"),
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("r"),
            tag("t"),
            tag("u"),
        )),
    ))(input)
}

fn string_body(input: &str) -> IResult<&str, &str> {
    recognize(many0(alt((nonescaped_string, escape_code))))(input)
}

fn string_literal(input: &str) -> IResult<&str, String> {
    let parser = delimited(tag("\""), string_body, tag("\""));
    map_res(parser, |s| unescape(s))(input)
}

fn json_string(input: &str) -> IResult<&str, Node> {
    map(string_literal, |s| Node::Str(s))(input)
}

// fn unescape(s: &str) -> Result<String, UnescapedError>;

#[test]
fn test_bool() {
    assert_eq!(json_bool("false"), Ok(("", Node::Bool(false))));
    assert_eq!(json_bool("true"), Ok(("", Node::Bool(true))));
    assert_eq!(json_integer("0"), Ok(("", Node::Integer(0))));
    assert_eq!(json_integer("01"), Ok(("1", Node::Integer(0))));
    assert_eq!(json_literal("78.0"), Ok(("", Node::Float(78.0))));
    assert_eq!(json_literal("56"), Ok(("", Node::Integer(56))));
}

#[test]
fn test_null() {
    assert_eq!(json_null("null"), Ok(("", Node::Null)));
}

#[test]
fn test_string() {
    // Plain Unicode strings with no escaping
    assert_eq!(json_string(r#""""#), Ok(("", Node::Str("".into()))));
    assert_eq!(
        json_string(r#""Hello""#),
        Ok(("", Node::Str("Hello".into())))
    );
    assert_eq!(json_string(r#""の""#), Ok(("", Node::Str("の".into()))));
    assert_eq!(json_string(r#""𝄞""#), Ok(("", Node::Str("𝄞".into()))));

    // valid 2-character escapes
    assert_eq!(
        json_string(r#""  \\  ""#),
        Ok(("", Node::Str("  \\  ".into())))
    );
    assert_eq!(
        json_string(r#""  \"  ""#),
        Ok(("", Node::Str("  \"  ".into())))
    );

    // valid 6-character escapes
    assert_eq!(
        json_string(r#""\u0000""#),
        Ok(("", Node::Str("\x00".into())))
    );
    assert_eq!(json_string(r#""\u00DF""#), Ok(("", Node::Str("ß".into()))));
    assert_eq!(
        json_string(r#""\uD834\uDD1E""#),
        Ok(("", Node::Str("𝄞".into())))
    );

    // Invalid because surrogate characters must come in pairs
    assert!(json_string(r#""\ud800""#).is_err());
    // Unknown 2-character escape
    assert!(json_string(r#""\x""#).is_err());
    // Not enough hex digits
    assert!(json_string(r#""\u""#).is_err());
    assert!(json_string(r#""\u001""#).is_err());
    // Naked control character
    assert!(json_string(r#""\x0a""#).is_err());
    // Not a JSON string because it's not wrapped in quotes
    assert!(json_string("abc").is_err());
}
*/
