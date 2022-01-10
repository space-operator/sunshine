use std::collections::HashSet;

use crate::Binding;

#[derive(Clone, Debug)]
pub struct DeviceMapping<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo> {
    pub bindings: HashSet<Binding<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>>,
}

impl<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>
    DeviceMapping<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>
{
    pub fn new(bindings: HashSet<Binding<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>>) -> Self {
        Self { bindings }
    }

    pub fn bindings(&self) -> &HashSet<Binding<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>> {
        &self.bindings
    }

    pub fn into_bindings(self) -> HashSet<Binding<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>> {
        self.bindings
    }
}

impl<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo> Default
    for DeviceMapping<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>
{
    fn default() -> Self {
        Self {
            bindings: HashSet::default(),
        }
    }
}
