use std::collections::BTreeMap;

use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockId<'a>(pub &'a str);

#[derive(Clone, Debug, Default)]
pub struct Block {
    spans: String,
    blocks: Vec<Block>,
}

#[derive(Clone, Debug, Default)]
pub struct BlockParser<'a> {
    nestings: BTreeMap<&'a str, ParserNestingLevel>,
}

#[derive(Clone, Debug)]
struct ParserNestingLevel {
    blocks: Vec<Block>,
    spans: String,
}

impl<'a> BlockParser<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_blocks(self) -> Vec<Block> {
        self.nestings
            .into_iter()
            .rev()
            .fold(vec![], |children, (_, nesting)| {
                let mut blocks = nesting.blocks;
                blocks.push(Block {
                    spans: nesting.spans,
                    blocks: children,
                });
                blocks
            })
    }

    pub fn with(self, line: &'a str) -> Self {
        let mut nestings = self.nestings;
        let regex = Regex::new("^[ \t]*").unwrap();
        let prefix_len = regex.find(&line).unwrap().end();
        if prefix_len == line.len() {
            return Self { nestings };
        }
        let prefix = &line[..prefix_len];
        let spans = line[prefix_len..].to_owned();

        let rest = nestings.split_off(prefix);
        if let Some(first) = rest.iter().next() {
            assert_eq!(&prefix, first.0);
        }
        let blocks = BlockParser { nestings: rest }.into_blocks();

        nestings.insert(prefix, ParserNestingLevel { blocks, spans });
        Self { nestings }
    }
}

#[test]
fn test() {
    let lines = r#"
    A
    B
      C
      D
    E
        F
        G
            H
            I
    J
            K
                                    L
                                    M
    N
    "#;
    let lines = lines.split("\n");
    let mut parser = BlockParser::new();
    for line in lines {
        parser = parser.with(&line);
    }
    //println!("{:#?}", parser.into_blocks());
    //panic!();
}

#[test]
#[should_panic]
fn test2() {
    let lines = r#"
        A
                B
            C
    "#;
    let lines = lines.split("\n");
    let mut parser = BlockParser::new();
    for line in lines {
        parser = parser.with(&line);
    }
}

#[test]
#[should_panic]
fn test3() {
    let lines = r#"
            A
        B
    "#;
    let lines = lines.split("\n");
    let mut parser = BlockParser::new();
    for line in lines {
        parser = parser.with(&line);
    }
}
