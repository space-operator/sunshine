use std::collections::HashMap;

use input_core::Modifiers;

#[derive(Clone, Debug)]
pub struct Events<'a, Mo, Ev>(HashMap<&'a Modifiers<Mo>, Vec<Ev>>);

impl<'a, Mo, Ev> Events<'a, Mo, Ev> {
    pub fn new(events: HashMap<&'a Modifiers<Mo>, Vec<Ev>>) -> Self {
        Self(events)
    }
}
