mod block;
//mod history;
//mod resolver;
mod span;
mod text;

pub use block::*;
//pub use history::*;
//pub use resolver::*;
pub use span::*;
pub use text::*;

/*
paste:
    current_block_id
    text_before
    text_after
    pasted_text

TODO:
    db interaction
    change db using text for input
    change db using test for paste
    change db using text for input, parse and input

*/

/*#[derive(Clone, Debug)]
pub enum Span {
    Text(Text),
    Link { title: Text, address: String },
    Image { title: String, address: String },
    Attribute { name: String, value: String },
    Widget,
}*/

/*

A
    B
      C
         D
    E
*/

/*
A
    B
      C
      D
    E
        F           nesting.blocks
        G           nesting.spans
            H       prev_blocks
            J       ...
    Y


4   Schools         4 spans
8       SchoolA     8 blocks[0]
8       SchoolB     8 blocks[1]
8       SchoolC     8 blocks[2]
8       SchoolD     8 spans
4   Uni...v

{
    0: [] / A
    4: [ (B: (C:), (D:)) ] / E
    8: [ (F:) ] / G
    12: [ (H:) ] / J
}

    12: [ (H:), (J:) ]
    8: [ (F:) (G: (H:), (J:)) ]
    4: [ (B: (C:), (D:)), (E: (F:), (G: (H:), (J:))) ]






"": [], A
"    ": [B(C, D)], E
"        ": [F], G
"            ": H
"        ": [F, G(H)]
"    ": [B(C, D), E([F, G(H)])


Schools
    Looking to find shool less than .cost: 4200
    .total= avg().cost
    London School
        Location: London
        Rating: 5 [link]
    New-York School
        Location: New-York
        Rating: 6
    Tokio School
        Location: Tokio
        Rating: 7
        .cost:3500
    London School
        Location: London
        Rating: 1
        .cost:2200


Schools
    Abc
==== convert tab to spaces
Schools
 Abc
 dfg
    qwe

Schools2
   A
   B


====
Schools
      Abc
====
Schools
\t\t\t\tAbc
====
        Schools
            Abc
==== Invalid case
Root
       Schools
    Abc
        Schools
    Abc
        Schools
*/

/*

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
        let string = source.borrow();

        /*
            ** abc ~~ abc ** abc ~~


            if "**"
                Yes => search "**" as end
                    Yes => AsBold start..end
                        repeat
                    No => AsText
                No => search "**" as end
                    Yes =>
                        repeat
                            Text => start..Text::end
                            Bold => start..end , Bold
                    No => AsText start..

        https://github.com/kivikakk/comrak/blob/main/src/nodes.rs
        */

        let nodes = {
            let string = source.borrow();
            let range = |substr: &str| {
                // string:  ABC123abc
                // substr:    B123a
                let ptr_diff = substr.as_ptr() as usize - string.as_ptr() as usize;
                ptr_diff..ptr_diff + substr.len()
            };
            let range2 = |substr1: &str, substr2: &str| {
                assert_eq!(
                    substr1.as_ptr() as usize + substr1.len(),
                    substr2.as_ptr() as usize
                );
                let ptr_diff = substr1.as_ptr() as usize - string.as_ptr() as usize;
                ptr_diff..ptr_diff + substr1.len() + substr2.len()
            };

            let try_bold = || {
                alt((
                    map(
                        delimited(tag("**"), take_until("**"), tag("**")),
                        |s: &str| Ok(s),
                    ),
                    map(rest, |s: &str| Err(s)),
                ))
            };

            let mut parser = alt((
                map(pair(peek(tag("**")), try_bold()), |(_, maybe_bold)| {
                    maybe_bold.map_or_else(
                        |s: &str| vec![MarkdownNode::Text(range(s))],
                        |s: &str| vec![MarkdownNode::Bold(range(s))],
                    )
                }),
                map(
                    pair(take_until("**"), pair(peek(tag("**")), try_bold())),
                    |(text, (_, maybe_bold))| {
                        maybe_bold.map_or_else(
                            |s: &str| vec![MarkdownNode::Text(range2(text, s))],
                            |s: &str| {
                                vec![
                                    MarkdownNode::Text(range(text)),
                                    MarkdownNode::Bold(range(s)),
                                ]
                            },
                        )
                    },
                ),
                map(rest, |s: &str| vec![MarkdownNode::Text(range(s))]),
            ));

            /*let mut parser = many0(alt((
                map(
                    delimited(tag("**"), take_until("**"), tag("**")),
                    |s: &str| MarkdownNode::Bold(range(s)),
                ),
                map(take(1_usize), |s: &str| MarkdownNode::Text(range(s))),
            )));*/

            let mut nodes = vec![];
            let mut rest = string;
            for j in 0..4 {
                let result: IResult<&str, _, NomError<_>> = parser(rest);
                let (rest, new_nodes) = result.unwrap();
                nodes.extend(new_nodes);
                if rest == "" {
                    break;
                }
            }
            //dbg!(&result);
            //assert_eq!(rest, "");
            nodes
        };

        Self { source, nodes }
    }
}

/*
    ** abc ~~ abc ** abc ~~

    **ab
    # Dillinger
    aa**

    - "**" !"**"* "**"
    - "**" ch*
    - ch* "**"?
    ----- "**" !"**"* "**"
    -----
    {}
    ()

    **bold** rest
    chars**bold**
    chars**notbold
*/

#[test]
fn test2() {
    let ok = |ok| -> Result<_, nom::Err<NomError<_>>> { Ok(ok) };
    assert_eq!(
        delimited(tag("**"), is_not("**"), tag("**"))("**abc-def**"),
        ok(("", "abc-def"))
    );
    /*assert_eq!(
        delimited(tag("**"), is_not("**"), tag("**"))("**abc*def**"),
        ok(("", "abc*def"))
    );*/
}

#[test]
fn test() {
    let md = {
        let s = String::from("**qwe* qw**e");
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

#[test]
fn test_cmark() {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::all();
    let input = r#"

*Italic*

**Bold**

_Italic_

__Bold__

# Heading 1

Heading 1
=========

## Heading 2

Heading 2
---------

[Link](http://a.com)

[Link][1]

![Image](http://url/a.png)

![Image][2]

> Blockquote

* List
* List
* List

- List
- List
- List

1. One
2. Two
3. Three

1) One
2) Two
3) Three

Horizontal rule:

---

Horizontal rule:

***

`Inline code` with backticks

```
# code block
print '3 backticks or'
print 'indent 4 spaces'
```

[1]: http://b.org
[2]: http://url/b.jpg
    "#;

    /*struct Event2<'a> {
        Start(Tag<'a>),
        End(Tag<'a>),
        Text(NomParserResult),
        Code(CowStr<'a>),
        Html(CowStr<'a>),
        FootnoteReference(CowStr<'a>),
        SoftBreak,
        HardBreak,
        Rule,
        TaskListMarker(bool),
    }*/

    let parser = Parser::new_ext(input, options);
    /*let result = parser.map(|node| match {
        Event::Start(t) => Event2::Start(t),
        Event::End(t) => Event2::End(t),
        Event::Text(t) => Event2::Text(parse_with_nom(t)),

    })*/
    dbg!(parser.collect::<Vec<_>>());
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
is on the new node
parent node
    == Section lsdjfl;ksjf
    asdfkja;lsdf


    text input field

parent node
    start typing - node.

    more text - node

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
*/
