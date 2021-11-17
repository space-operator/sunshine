use crate::BlockId;

type Spans<'a> = Vec<Span<'a>>;

#[derive(Clone, Debug)]
pub enum Span<'a> {
    Link(BlockId<'a>),
    Url(Vec<Span<'a>>, &'a str),
    Style(Vec<Span<'a>>, &'a str),
    Text(&'a str),
}

impl<'a> Span<'a> {
    fn parse(text: &'a str) -> Spans<'a> {
        let bytes = text.bytes().enumerate();
        let spans = Vec::new();
        SpanParser {
            text,
            bytes,
            spans,
            maybe_href: false,
        }
        .parse()
    }
}

#[derive(Clone, Debug)]
struct SpanParser<'a> {
    text: &'a str,
    bytes: std::iter::Enumerate<std::str::Bytes<'a>>,
    spans: Spans<'a>,
    maybe_href: bool,
}

// [1324-1234](qwerasdf[1234-1234](qwerasdf[1234-1234](qwerasdf[1234-1234](qwerasdf[1234-1234])
// [1234-2345]
// https://github.com/pest-parser/pest

// [^[]
// \[eqerqwer\]( map

// https://pest.rs/book/grammars/syntax.html

impl<'a> SpanParser<'a> {
    fn parse(mut self) -> Spans<'a> {
        let mut text_offset = 0;
        let mut open_offset = None;
        while let Some((offset, ch)) = self.bytes.next() {
            match ch {
                b'[' => open_offset = Some(offset),
                b']' => match open_offset {
                    Some(start) => {
                        let next = self.bytes.next();
                        if next == Some(b'(') {
                        } else {
                        }
                    }
                    None => {}
                },
                _ => {}
            }
        }
        spans
    }

    fn parse_href(mut self, link_offset: usize, url_offset: usize) -> Spans<'a> {
        let mut open_offset = None;
        while let Some((offset, ch)) = self.bytes.next() {
            match ch {
                b')' => self.spans,
                b'[' => open_offset = Some(offset),
                b']' => match open_offset {
                    Some(start) => {
                        let next = self.bytes.next();
                        if next == Some(b'(') {
                        } else {
                        }
                    }
                    None => {}
                },
                _ => {}
            }
        }
        spans
    }
}

#[test]
fn test() {
    for value in [
        "**abc*",
        "**abc*abcasda**",
        "**abc* abc",
        "**abc* abc**asd",
        "[LINK] _abc*abc_abc*\n\n[LINK]: /blah",
        "_abc*abc_ abc*",
    ] {
        use pulldown_cmark::{Options, Parser};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(value, options);

        let events: Vec<_> = parser.collect();
        println!("{:?}", events);
    }

    todo!();
}

/*
    [link]
    [style A, text]
    [title](url)



    edge cases not handled yet
    [1234-2345-3456-4567](http://wow)




    library of style for the user

    style 1
        alias: book cover


    [style1,style2|text]

    !italics@
    #bold$
    %strike^



        italic
        bold
        16pts


https://github.com/raphlinus/pulldown-cmark/blob/master/src/linklabel.rs
https://github.com/raphlinus/pulldown-cmark/blob/master/tests/suite/regression.rs


    Url
        StyleA


    // **abc*           *[i]abc[/]
    // **abc*abc        **abc*abc
    // **abc* abc       *[i]abc[/] abc
    // **abc* abc**asd  [i]abc abc[/]*abc
    // _abc*abc_abc*    _asd[i]asd_asd[/]
    // _abc*abc_ abc*   [i]abc*abc[/] abc*

    text_start: usize
    underscore_bold_start: usize,
    asterisk_bold_start: usize,
    underscore_italics_start: usize,
    asterisk_italics_start: usize,

    c char
    - space
    * asterisk
    _ underscore
    ~ tilde

    c*a     *italics end or start
    c*-     *italics end
    c**
    c*_c    *italics end or start
    c*_-    *italics end or start or _italics end or start
    c*_*

    -*a     italics start
    -*-

    [qweqwe]

    asd**asdasdas...**asdasd
    ^    ***********
    ---  ***********  ^

    asd**asdasdas*asdda*aass
         ^^^^^^^^
    -------------+++++++++++

    0    1   2    3   4
    abc**abc*abc**abc*abc

    bold(0, 1)
        italics(1, 2) +b

    ParseChunkResult
*/

/*impl<'a> Span<'a> {
    pub fn parse(text: &'a str) -> Spans<'a> {
        let offset = 0;
        let bytes = text.bytes().enumerate();
        let spans = Vec::new();

        SpanParser {
            text,
            offset,
            bytes,
            spans,
        }
        .parse()
    }
}

trait Is {
    fn is() -> bool;
}

struct Yes;
impl Is for Yes {
    fn is() -> bool {
        true
    }
}

struct No;
impl Is for No {
    fn is() -> bool {
        false
    }
}*/

/*
#[derive(Clone, Debug)]
struct SpanParser<'a, Bold, Italics> {
    text: &'a str,
    offset: usize,
    bytes: std::iter::Enumerate<std::str::Bytes<'a>>,
    spans: Spans<'a>,
    _: Bold,
    _: Italics,
}*/

/*
    a ** b * c ~~ d ~~ e ~~ f * g ~~ h * j **
         BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
             IIIIIIIIIIIIIIII
                  S
                         ~~       ~~   *


**asdasda **asda** asda
asdasd**aa*asd~~asd**
asdasd**


**sasdasda*

asdasd **asdasd
asdasd** asdasda

asdasda**asdasdasd sda*a*sd
**ad **asd**
asdasd**as*dasd*asda
    -> [Bold(spans..)]
    -> [spans..]


*/

/*
impl<'a, Bold: Is, Italics: Is> SpanParser<'a, Bold, Italics> {
    pub fn parse(mut self) -> Spans<'a> {
        self = self.parse_after_char();
        if self.offset == self.text.len() {
            self.spans
        } else {
            self.spans.with(Span::Text(&self.text[self.offset..]))
        }
    }

    pub fn parse_after_char(mut self) -> Self {
        while let Some((start, ch)) = self.bytes.next() {
            match ch {
                b'*' => parse_after_char_star(self),
                b' ' => parse_after_space(self),
                _ => parse_after_char(self),
            }
        }
    }

    pub fn parse_after_space(mut self) -> Self {
        while let Some((start, ch)) = self.bytes.next() {
            match ch {
                b'*' => parse_after_space_star(self),
                b' ' => parse_after_space(self),
                _ => parse_after_char(self),
            }
        }
    }

    pub fn parse_after_char_star(mut self) -> Self {
        while let Some((start, ch)) = self.bytes.next() {
            match ch {
                b'*' => parse_after_char(self), // wrap bold
                b' ' => parse_after_space(self),
                _ => if Italics::is() {

                } else {
                    let offset = self.offset;
                    let spans = self.spans;
                    let parser = SpanParser {
                        text: self.text,
                        offset,
                        bytes: self.bytes,
                        spans: vec![],
                    }
                    .parse_after_char();
                    Self {
                        text: parser.text,
                        offset,
                        bytes,
                        spans.with()
                    }
                }
            }
        }
    }

    pub fn parse_after_space_star(mut self) -> Self {
        while let Some((start, ch)) = self.bytes.next() {
            match ch {
                b'*' => parse_after_char(self), // wrap bold
                b' ' => parse_after_space(self),
                _ => parse_after_char(self),
            }
        }
    }


    /*pub fn parse_maybe_italics(mut self) -> Self {
        match self.bytes.next() {
            Some((offset, b'*')) => self.parse_maybe_bold(),
            Some((offset, b' ')) => self,
            (offset, _) => {
                // a*
                SpanParser {
                    text: self.text,
                    offset,
                    bytes: self.bytes,
                    spans: vec![],
                }
                .parse()
            }
        }
    }

    pub fn parse_maybe_bold(mut self) -> Self {
        match self.bytes.next() {
            Some((offset, b' ')) => todo!(),
            _ => todo!(),
        }
    }*/
}*/

/*
        let maybe_italics = || match iter.next() {
            Some(b'*') => match iter.next() {
                Some(b'*') => {}
            },
        };

        while let Some(start, ch) = iter.next() {
            match ch {
                b'*' => maybe_italics(),
            }
        }
*/
/*enum Found {
    LinkStart,
    LinkEnd,
    BoldStart,
    BoldEnd,
    ItalicsStart,
    ItalicsEnd,
    StrikethroughStart,
    StrikethroughEnd,
}

let pred = |ch| {
    if ch == b'*' {
        if state.bold {

        }
    }
    if ch == "*" {

    }
    let link_ch == if state.link { b']' } else { b'[' };
    ch == b'*' || ch == b'~' || ch == link_ch
};
match text.iter().find_map(pred) {
    Some(start) => match text.as_bytes()[start] {
        b'[' =>
        b'*',
        b'~',
        _ => unreachable!(),
    }
    None => Span::Text(&text),
}*/

/*
#[derive(Clone, Debug)]
pub struct SpanParser2<'a> {
    spans: Spans<'a>,
    state: State,
}

#[derive(Clone, Copy, Debug)]
enum State {
    Text(ParentState),
    MaybeItalics(ParentState),
    MaybeBold(ParentState),
    MaybeStrikethrough(ParentState),
}

#[derive(Clone, Copy, Debug, Default)]
struct ParentState {
    maybe_link: bool,
    maybe_bold: bool,
    maybe_italics: bool,
    maybe_strikethrough: bool,
}

#[derive(Clone, Copy, Debug)]
struct TextState {
    offset: usize,
}

#[derive(Clone, Copy, Debug)]
struct LinkOrUrlState {
    offset: usize,
}

#[derive(Clone, Copy, Debug)]
struct TextOrUrlState {
    offset: usize,
}

#[derive(Clone, Copy, Debug)]
struct UrlState {
    title_offset: usize,
    url_offset: usize,
}

#[derive(Clone, Debug)]
enum ParseChunkResult<'a> {
    TextAndParser((&'a str, Spans<'a>, State)),
    Spans(Spans<'a>),
}

impl<'a> SpanParser2<'a> {
    pub fn parse(mut text: &'a str) -> Spans<'a> {
        let mut parser = Self::new();
        loop {
            let (rest, spans, state) = match parser.parse_chunk(text) {
                ParseChunkResult::TextAndParser(result) => result,
                ParseChunkResult::Spans(spans) => return spans,
            };
            text = rest;
            parser = Self { spans, state };
        }
    }

    fn new() -> Self {
        Self {
            spans: vec![],
            state: State::Text(TextState { offset: 0 }),
        }
    }

    fn parse_chunk(self, text: &'a str) -> ParseChunkResult {
        let spans = self.spans;
        match self.state {
            State::Text(state) => ParseChunkResult::TextAndParser(state.parse(text, spans)),
            State::LinkOrUrl(state) => ParseChunkResult::TextAndParser(state.parse(text, spans)),
            State::TextOrUrl(state) => ParseChunkResult::TextAndParser(state.parse(text, spans)),
            State::Url(state) => ParseChunkResult::TextAndParser(state.parse(text, spans)),
            State::End => ParseChunkResult::Spans(spans),
        }
    }
}

impl TextState {
    fn parse<'a>(self, text: &'a str, spans: Spans<'a>) -> (&'a str, Spans<'a>, State) {
        match text.find('[') {
            Some(offset) => (
                &text[offset..],
                spans,
                State::LinkOrUrl(LinkOrUrlState { offset }),
            ),
            None => (text.empty_end(), spans.with(Span::Text(&text)), State::End),
        }
    }
}

impl LinkOrUrlState {
    fn parse<'a>(self, text: &'a str, spans: Spans<'a>) -> (&'a str, Spans<'a>, State) {
        let rest = text;
        let state = todo!();
        (rest, spans, state)
    }
}

impl TextOrUrlState {
    fn parse<'a>(self, text: &'a str, spans: Spans<'a>) -> (&'a str, Spans<'a>, State) {
        let rest = text;
        let state = todo!();
        (rest, spans, state)
    }
}

impl UrlState {
    fn parse<'a>(self, text: &'a str, spans: Spans<'a>) -> (&'a str, Spans<'a>, State) {
        let rest = text;
        let state = todo!();
        (rest, spans, state)
    }
}

trait StrEmptyEnd {
    fn empty_end(self) -> Self;
}

impl<'a> StrEmptyEnd for &str {
    fn empty_end(self) -> Self {
        &self[self.len()..self.len()]
    }
}

trait VecWith<T> {
    fn with(self, value: T) -> Self;
}

impl<T> VecWith<T> for Vec<T> {
    fn with(mut self, value: T) -> Self {
        self.push(value);
        self
    }
}

#[test]
fn test() {
    let text = "abc [link] def [href](http://website.com) qwe [123";
    let result = SpanParser2::parse(text);
    dbg!(result);
}
*/
/*
/*
    a ** b * c ~~ d ~~ e ~~ f * g ~~ h * j **
         BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
             IIIIIIIIIIIIIIII
                  S
                         ~~       ~~   *
*/

#[derive(Clone, Debug, Default)]
struct State2 {
    link: bool,
    bold: bool,
    italics: bool,
    strikethrough: bool,
}

impl Span<'a> {
    pub fn parse<'a>(text: &'a str) -> Spans<'a> {
        Self::parse_chunk(text, State2::default())
    }

    fn parse_chunk<'a>(text: &'a str, state: State2) -> Spans<'a> {
        for j in 0..text.len() {
            if text[j] == b'*' {

            }
        }

        enum Found {
            LinkStart,
            LinkEnd,
            BoldStart,
            BoldEnd,
            ItalicsStart,
            ItalicsEnd,
            StrikethroughStart,
            StrikethroughEnd,
        }

        [[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[

        ** aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa aaaaaaaaaaaaa **
        **

        let pred = |ch| {
            if ch == b'*' {
                if state.bold {

                }
            }
            if ch == "*" {

            }
            let link_ch == if state.link { b']' } else { b'[' };
            ch == b'*' || ch == b'~' || ch == link_ch
        };
        match text.iter().find_map(pred) {
            Some(start) => match text.as_bytes()[start] {
                b'[' =>
                b'*',
                b'~',
                _ => unreachable!(),
            }
            None => Span::Text(&text),
        }
    }
}
*/

/*
trait Spans {
    fn parse_text(self, text: &'a str) -> (&'a str, SpanParser<'a>);
    fn parse_link_or_url(self, text: &'a str) -> (&'a str, SpanParser<'a>);
    fn parse_text_or_url(self, text: &'a str) -> (&'a str, SpanParser<'a>);
    fn parse_url(self, text: &'a str) -> (&'a str, SpanParser<'a>);
}

impl<'a> Spans for Spans<'a> {
    fn parse_text(self, text: &'a str) -> (&'a str, SpanParser<'a>) {
        let rest = text;
        let state = todo!();
        (rest, SpanParser { spans: self, state })
    }

    fn parse_link_or_url(self, text: &'a str) -> (&'a str, SpanParser<'a>) {
        let rest = text;
        let state = todo!();
        (rest, SpanParser { spans: self, state })
    }

    fn parse_text_or_url(self, text: &'a str) -> (&'a str, SpanParser<'a>) {
        let rest = text;
        let state = todo!();
        (rest, SpanParser { spans: self, state })
    }

    fn parse_url(self, text: &'a str) -> (&'a str, SpanParser<'a>) {
        let rest = text;
        let state = todo!();
        (rest, SpanParser { spans: self, state })
    }
}*/

/*for (offset, ch) in text.bytes().enumerate() {
    parser = parser.with(text, offset, ch);
}
let len = text.len();
match parser.state {
    State::Text(offset) => {
        if offset == text.len() {
            parser.spans
        } else {
            parser.spans.with(Span::Text(&text[offset..len]))
        }
    }
    State::LinkOrUrl(offset) => parser.spans.with(Span::Text(&text[offset - 1..len])),
    State::TextOrUrl(offset) => parser.spans,
    State::Url(title_start, url_start) => {
        parser.spans.with(Span::Text(&text[title_start - 1..len]))
    }
}*/
/*fn with(self, text: &'a str, offset: usize, ch: u8) -> Self {
    use self::Url as SpanUrl;
    use State::*;

    let spans = self.spans;
    let state = self.state;
    let (spans, state) = match state {
        Text(start) => match ch {
            b'[' => (
                spans.with(Span::Text(&text[start..offset])),
                LinkOrUrl(offset + 1),
            ),
            _ => (spans, Text(start)),
        },
        LinkOrUrl(start) => match ch {
            b'[' => panic!(),
            b']' => (spans, TextOrUrl(start)),
            _ => (spans, LinkOrUrl(start)),
        },
        TextOrUrl(start) => match ch {
            b'(' => (spans, Url(start, offset + 1)),
            _ => (
                spans.with(Span::Link(BlockId(&text[start..offset - 1]))),
                Text(offset),
            ),
        },
        Url(title_start, url_start) => match ch {
            b')' => (
                spans.with(Span::Url(SpanUrl {
                    name: &text[title_start..url_start - 2],
                    url: &text[url_start..offset],
                })),
                Text(offset + 1),
            ),
            _ => (spans, Url(title_start, url_start)),
        },
    };
    Self { spans, state }
}*/

/*
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
*/
/*
syntax
*/

/*

Good:
    [internal block]
    [title](https://www.example.com)
    **bold**
    *italics*


Root
    //Schools
    //    [1829-1234] asdasdasdasdqwdasdqwdasdq]
    Schools
        [Schools] asdasdasdasdqwdas qweasdqwe qweqweqqweasd
    Hello
        [1232-2341] asdasdasdasdqqqq

Root
    //Schools
    //    [Root] asdasdasdasdqwdasdqwdasdq]
    Schools
        [Schools(!)] asdasdasddqwdas qweasdqwe
    Hello
        [Schools] asdasdasdasdqwdas qweasdqwe

Added "a" on 15
Removed "123" on 1234


[\[\[qdasdqwe\]\]] -> [[[qdasdqwe]]]


[abc[dfg[hj]]]
[abc[dfg[hj]
[abc[dfg[hj]]]


[link_id]
[link_id][link_id][link_id][link_id][link_    id]
[link name] long long very



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

/*pub enum Span {


    Text(Markdown, Style:Heading),
    Link { title: Text, address: String },
    Image { title: String, address: String },
    Attribute { name: String, value: String },
    Widget,
}*/

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
