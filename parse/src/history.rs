// #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// pub struct TextId(usize);

// #[derive(Clone, Debug)]
// pub struct History {
//     undos: Vec<Commit>,
//     redos: Vec<Commit>,
//     texts: HashMap<TextId, String>,
// }

// #[derive(Clone, Debug)]
// pub struct Commit {
//     changes: Vec<Change>,
//     cursor_before: Cursor,
//     cursor_after: Cursor,
// }

// #[derive(Clone, Debug)]
// pub struct Change {
//     text_id: TextId,
//     offset: usize,
//     before: String,
//     after: String,
// }

// #[derive(Clone, Debug)]
// pub struct Cursor {
//     text_id: TextId,
//     start: usize,
//     end: usize,
// }

// impl History {
//     fn get_text(&self, text_id: TextId) -> Option<&str> {
//         self.texts.get(text_id)
//     }

//     fn set_text(&mut self, text_id: TextId, text: String) {
//         use std::collections::HashMap::Entry;
//         match texts.entry(text_id) {
//             Entry::Occupied(entry) => undos
//             Entry::Vacant
//         }
//     }
// }

// undos
// redos

// root
//     a
//         b
//             c
//                 b
//             d
//                 ...
//                 ...
//             e
//                 qw asd qwe

//     String -> String
//     [1231] adfwefawsdfasdf

// 1357        B   12345678
// 1357        S   1-3-5-7-
//     2468    S   -2-4-6-8

// per-span history when typing
// per-block history when focused
// per-workspace history when nothing in focus

// +   a sdgffg
// +       b fsgdsdg asdf
// +           c safgdfg fsgsgf
//             d asdf

// --------------------------
// -       a sdgffg        -
// -  b fsgdsdg

// ====
// qwwefa 45646 4564 1231
// ====

// {
//     block_id,
//     offset: 0,
//     cursor: 0,
//     before: "",
//     after: "a",
// }

// {
//     block_id,
//     offset: 1,
//     cursor: 1,
//     before: "",
//     after: "b",
// }
// {
//     block_id,
//     offset: 1,
//     cursor: 2,
//     before: "b",
//     after: "",
// }
