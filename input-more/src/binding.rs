use input_core::Modifiers;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SwitchBinding<Sw, Mo, Td, Pd, Ev> {
    pub switch: Sw,
    pub modifiers: Modifiers<Mo>,
    pub timed_data: Td,
    pub pointer_data: Pd,
    pub event: Ev,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TriggerBinding<Tr, Mo, Ev> {
    pub trigger: Tr,
    pub modifiers: Modifiers<Mo>,
    pub event: Ev,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoveBinding<Mo, Pd, Ev> {
    pub modifiers: Modifiers<Mo>,
    pub pointer_data: Pd,
    pub event: Ev,
}
