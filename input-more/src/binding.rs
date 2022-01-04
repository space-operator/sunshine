use input_core::Modifiers;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Binding<Sw, Mo, Td, Pd, Ev> {
    pub switch: Sw,
    pub modifiers: Modifiers<Mo>,
    pub timed_data: Td,
    pub pointer_data: Pd,
    pub event: Ev,
}
