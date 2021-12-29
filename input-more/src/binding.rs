use input_core::Modifiers;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Binding<Sw, Mo, Ti, Ev> {
    pub switch: Sw,
    pub modifiers: Modifiers<Mo>,
    pub timed_data: Ti,
    pub event: Ev,
}
