use core::borrow::Borrow;
use core::ops::Range;
use nom::error::Error as NomError;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take, take_until, take_while},
    character::complete::{alphanumeric1 as alphanumeric, char as ch, one_of},
    combinator::{cut, map, opt, value},
    error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    multi::many0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    Err as NomErr, IResult,
};

#[derive(Clone, Debug)]
struct Markdown<T> {
    source: T,
    nodes: Vec<MarkdownNode>,
}

#[derive(Clone, Debug)]
enum MarkdownNode {
    Text(Range<usize>),
    Bold(Range<usize>),
}

impl<T: Borrow<str>> From<T> for Markdown<T> {
    fn from(source: T) -> Self {
        let nodes = {
            let string = source.borrow();
            let range = |substr: &str| {
                // string:  ABC123abc
                // substr:    B123a
                let ptr_diff = substr.as_ptr() as usize - string.as_ptr() as usize;
                ptr_diff..ptr_diff + substr.len()
            };

            let mut parser = many0(alt((
                map(
                    delimited(tag("**"), take_until("**"), tag("**")),
                    |s: &str| MarkdownNode::Bold(range(s)),
                ),
                map(take(1_usize), |s: &str| MarkdownNode::Text(range(s))),
            )));
            let result: IResult<&str, Vec<_>, NomError<_>> = parser(string);
            let (rest, nodes) = result.unwrap();
            assert_eq!(rest, "");
            nodes
        };

        Self { source, nodes }
    }
}

#[test]
fn test() {
    let md = {
        let s = String::from("abc **qwe** qwe");
        let md = Markdown::from(s);
        md
    };
    dbg!(md);
}

#[test]
fn test1() {
    let ok1 = |ok| -> Result<_, nom::Err<NomError<_>>> { Ok(ok) };
    let ok2 = |ok| -> Result<_, nom::Err<NomError<_>>> { Ok(ok) };
    assert_eq!(tag("**")("**a"), ok1(("a", "**")));
    assert_eq!(many0(tag("**"))("****a"), ok2(("a", vec!["**", "**"])));
}

/*


nom node
    description
    example
    documentation
        sub section
        ....

node
    anom is a parser combinators library written in Rust.
    Its goal is to provide [[tools]] to build safe parsers without compromising
    the speed or memory consumption.
    To that end, it uses extensively Rust's strong
    typing and memory safety to produce fast and correct parsers, and provides functions,
    macros and ((id.12312){}) to abstract most of the error prone plumbing.
    /today
    /map

    /today
    [[tools]]

refer to ((sub section id))


person
name:: amir




/command -> call the command

[[nom]] replaces text with a hyperlink to the nom node

((sub section id)) replaces text with link to subsection

{{[[embed]]: ((node id to embed))}}

:: creates attributes/properties on the node

*/

/*
/ add edge

/add node abc
/edge add
/add edge arg1, arg2
id:
*/

// add edge

// add edge id=qwe from=asd to=zxc
// add edge qwe asd zxc
// add edge asd zxc

// add node id=asd props.a=123 props.b=123 props.c=123 props.d=123
// add node id=asd props: a=123 b=123 c=123 d=123
// add node *asd props.a=123
// add type=node asd props.a=123

// set node id=asd props.a="qwdasd\" qwaweqwe"
// set node asd props.a="qwdasd\" qwaweqwe"

// set node props a="123" b="123" c= d=

// print node asd props.a
// print node asd

// remove node id=asd
// remove node asd
// remove edge asd

// Other considerations
// create a command generator
// return last command
// return last created node/edge

// command enum{
//     success //command worked
//     cancel //user canceled command
//     nothing /command did nothing and cancel was not pressed
//     failure //command failed, bad input, bad computation
//     unknown_command // not found or typo
//     exit_app //app exited
// }
