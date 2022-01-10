use input_core::{
    Modifiers, PointerChangeEventData, PointerMoveEventData, TimedClickExactEventData,
    TimedLongPressEventData, TimedReleaseEventData,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Binding<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo> {
    Press(SwitchBinding<Sw, Mo, (), (), EvPr>),
    Release(
        SwitchBinding<Sw, Mo, Option<TimedReleaseEventData>, Option<PointerChangeEventData>, EvRe>,
    ),
    LongPress(SwitchBinding<Sw, Mo, TimedLongPressEventData, (), EvLo>),
    ClickExact(SwitchBinding<Sw, Mo, TimedClickExactEventData, (), EvCl>),
    Trigger(TriggerBinding<Tr, Mo, EvTr>),
    Move(MoveBinding<Mo, PointerMoveEventData<Sw>, EvCo>),
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

impl<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>
    Binding<Sw, Tr, Mo, EvPr, EvRe, EvLo, EvCl, EvTr, EvCo>
{
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
