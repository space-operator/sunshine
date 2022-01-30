// TODO

use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Modifiers<Sw>(BTreeSet<Sw>);
