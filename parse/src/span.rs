use pulldown_cmark::{Options, Parser};

#[derive(Clone, Debug)]
pub struct Span(String);

/*
syntax
*/

/*
PasteOrType convert [link name] into [link_id]
    or maybe create if [link name] not exist
    or do nothing if multiple matches

BlockParser and SpanParser
    do not try to link [linkname] or [link_id]

Good:
    [internal block]
    [title](https://www.example.com)
    **bold**
    *italics*
    
We can do it better:
    +bold text -bold
    +16pts text -16pts
    +style[A] test -styleA
    +styleA

[]


node for styling

StyleA
    asdasd
StyleA
    StyleA
        .style
        .bold
        .italic
        16pt
    StyleA
        .style
        .strike
        .italic


[<span attr-name="Graph">asdq-123s-f24f-w24d<span>]


TextPostProcessor [link] -> [1231]
SpanParser [1231] -> ...visual form...

[internal block link name]
[block_uid]

[Schools][Schools][SchoolsA] 
[Schools][Schools][graphid-asdq-123s-f24f-w24d:] Schools

[Schools                |
[asdq-123s-f24f-w24d]   | ]

[Schools                | Ctrl+Z
][qwdasdasdasda]






styleA
    bold
    16pts
    italic



[123rqwer]

asdq **[id:123qwe123]** qdasd


type line
    -> reparsed [abc]->[1423] -> parsed to visule
finish line
    -> reparsed [abc]->[1423] -> saved

pasting
    handle offsets
    convert paths
typing
    handle tabs
    [Schools/A/B/A/B] asdasdqwdasdasd qweasdqwe
    [1231231] asdasdqwdasdasd qweasdqwe
    [ {link: 1231231}, {text: asdasdqwdasdasd qweasdqwe} ]


((file))

Schools
    qweqwe [[Schools]]


preview
    [[asdas]] sdasdq qwasd qweasd
    



preview
    [Schools/A/B/A/B]
    visually [B]
    on delete visible [B

    
 edit

    [Schools/A/B/A/B]
    visually [123123123]
    on delete visible [123123123

    
    [123123123]

    [Schools/A/B/A/B]
    [12312312C]


/command


    [title](https://www.example.com)


*/


//#[derive(Clone, Debug)]
//pub struct SpanParser {
//    spans: Vec<Span>,
//    state: State,
//}

/*
    [November 10, 2021]
    [title](https://www.example.com)



November 10, 2021
    A
        B
            C
                F [[November 10, 2021]]
            G
        H
    I



  https://github.com/athensresearch/athens
  https://github.com/athensresearch/athens/blob/main/src/cljc/athens/parser/impl.cljc
  

    [title: ttps://www.example.com]

    Schools
        SchoolA
            SchoolA

    Schools
        SchoolB
            SchoolA
                [Schools/SchoolA/SchoolA]
                qweasd
            qweqwe
        asdqwd



    [title](https://www.example.com)


     	[title](https://www.example.com)
    [Schools/.../.../]
        SchoolsA

    Schools
        SchoolsA

    qweasd
        [https://asdqwdqwdawea,title]

    Write   Preview

    +b bold-text -b +b non-bold-text -b Aqw
    bold-text non-bold-text A

    +b bold -b
    +i itatics -i
    +c=red red text -c
    +u underline -u

    [123]qwe[123]
    
    underline text underline -

    test (+styleA)text(-) qwe

    text

   
*/

/*
asdf
asdf
asdf @[]

# asdqweasd

    asdf @[]

# qwdqwdas

    asdfasdf @[]
    asdfasdf

    ## qwdqwe

        qweqwe
        qweqwe
        qwe

*/


Paragrath
    Text
    Image
    Wdiged
Paragrath
    Text
    Image
    Wdiged
Paragrath
    Text
    Image
    Wdiged
Paragrath
    Text
    Image
    Wdiged

/*pub enum Span {
        

    Text(Markdown, Style:Heading),
    Link { title: Text, address: String },
    Image { title: String, address: String },
    Attribute { name: String, value: String },
    Widget,
}*/

//#[derive(Clone, Debug)]
pub struct SpanParser;

impl SpanParser {
    pub fn parse(value: &str) -> Vec<Span> {
        vec![Span(value.to_owned())]
    }
}

#[test]
fn test() {
    let text = "*asd*";
    let parser = Parser::new_ext(text, Options::all());
    let events: Vec<_> = parser.collect();
    for event in events {
        println!("{:?}", event);
    }
    panic!();
}

/*
#[derive(Clone, Copy, Debug)]
enum State {
    Text(usize),
    MaybeItalicsStart(usize, usize),
    MaybeBoldStart(usize, usize),
    MaybeItalics(usize, usize),
    MaybeBold(usize, usize),
}

impl SpanParser {
    pub fn parse(value: &str) -> Vec<Span> {
        let parser = Self::new();
        for (offset, ch) in value.iter().enumerate() {
            parser = parser.with(offset, ch);
        }
        parser
    }

    fn new() -> Self {
        Self::Text(0)
    }

    fn from_raw(spans: Vec<Span>, state: State) -> Self {
        Self { spans, state }
    }

    fn with(self, offset: usize, ch: char) -> Self {
        use State::*;

        let spans = self.spans;
        let state = self.state;
        match state {
            Text(start) => match ch {
                "*" => Self::from_raw(spans, MaybeItalicsStart(start, offset)),
                _ => Self::from_raw(spans, Text(start)),
            },
            MaybeItalicsStart(start, offset) => match ch {
                "*" => Self::from_raw(spans, MaybeBoldStart(start, offset)),
                " " => Self::from_raw(spans, Text(start)),
                _ => Self::from_raw(spans, MaybeItalics(start, offset)),
            },
            MaybeBoldStart(start, offset) => match ch {
                "*" => Self::from_raw(spans, MaybeBoldItalicsStart(start, offset)),
                " " => Self::from_raw(spans, Text(start)),
                _ => Self::from_raw(spans, MaybeItalics(start, offset)),
            },
        }
    }
}*/

/*

    **asda**
    __asda**
    __asda__

    **~~**~~**asd**~~**~~**

    TEXT~sub~

    ***~~sdfwersd~~qweqwe***



    ***~~sdfwersd~~qweqwe***
      b
       b+i
         b+i+s
                  ^
    match ch
        substate


    a       Text
    *       MaybeItalicsStart
    **      MaybeBoldStart
    *a      MaybeItalics
    **a     MaybeBold

    // asdqwdfqdfqd
    // [ "**abc", "**" ]
*/

/*
    \

    abc **def*      -> abc * | *def*
    abc **def**     -> abc | **def**

****asda****
**\*asda***

    abc **qwe**
    ----I
        BI
          +
            +IB

    Text ( start )
    MaybeItalics (start, separator )
    MaybeItalicsOrBold (start, separator )
    MaybeBoldItalics (start, separator )
    Text () Italics ()
    ItalicsOrBold
    Bold ()

****italix\*\***
^^
****italix****

*/
