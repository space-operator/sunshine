use input_core::{
    Modifiers, PointerChangeEventData, PointerMoveEventData, TimedClickExactEventData,
    TimedLongPressEventData, TimedReleaseEventData,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Binding<Sw, Tr, Mo, Ev> {
    Press(SwitchBinding<Sw, Mo, (), (), Ev>),
    Release(
        SwitchBinding<Sw, Mo, Option<TimedReleaseEventData>, Option<PointerChangeEventData>, Ev>,
    ),
    LongPress(SwitchBinding<Sw, Mo, TimedLongPressEventData, (), Ev>),
    ClickExact(SwitchBinding<Sw, Mo, TimedClickExactEventData, (), Ev>),
    Trigger(TriggerBinding<Tr, Mo, Ev>),
    Move(MoveBinding<Mo, PointerMoveEventData<Sw>, Ev>),
}

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

impl<Sw, Tr, Mo, Ev> Binding<Sw, Tr, Mo, Ev> {
    pub fn modifiers(&self) -> &Modifiers<Mo> {
        match self {
            Self::Press(binding) => &binding.modifiers,
            Self::Release(binding) => &binding.modifiers,
            Self::LongPress(binding) => &binding.modifiers,
            Self::ClickExact(binding) => &binding.modifiers,
            Self::Trigger(binding) => &binding.modifiers,
            Self::Move(binding) => &binding.modifiers,
        }
    }
}
