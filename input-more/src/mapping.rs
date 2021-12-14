use input_core::Modifiers;

pub trait SwitchesMapping<'a> {
    type Switch;
    type Filtered: SwitchesSwitchMapping<'a, Switch = Self::Switch>;

    fn filter_by_switch(&'a self, switch: Self::Switch) -> Option<Self::Filtered>;
}

pub trait SwitchesSwitchMapping<'a> {
    type Switch;
    type Filtered: SwitchesSwitchModifiersMapping<'a, Switch = Self::Switch>;

    fn filter_by_modifiers(&'a self, modifiers: Modifiers<Self::Switch>) -> Option<Self::Filtered>;
}

pub trait SwitchesSwitchModifiersMapping<'a> {
    type Switch;
    type Event;
    type Binding: SwitchesMappingBinding<'a>;

    fn get_event(&'a self, event: Self::Event) -> Vec<(Modifiers<Self::Switch>, Self::Binding)>;
}

#[allow(single_use_lifetimes)] // false positive
pub trait SwitchesMappingBinding<'a> {
    type Event;

    fn build(&'a self) -> Self::Event;
}

#[test]
fn test() {
    use std::collections::HashMap;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Event {
        KeyDown(&'static str),
        KeyUp(&'static str),
        ButtonDown(&'static str),
        ButtonUp(&'static str),
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Switch {
        KeyboardKey(&'static str),
        MouseButton(&'static str),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum Binding {
        AddNode,
        RemoveNode,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum AppEvent {
        AddNode(u32),
        RemoveNode(u32),
    }

    #[derive(Clone, Debug, Default)]
    struct Mapping(HashMap<Switch, HashMap<Modifiers<Switch>, Vec<Binding>>>);

    #[derive(Clone, Debug)]
    struct MappingBySwitch<'a>(&'a HashMap<Modifiers<Switch>, Vec<Binding>>);

    #[derive(Clone, Debug)]
    struct MappingBySwitchAndModifiers<'a>(HashMap<&'a Modifiers<Switch>, &'a [Binding]>);

    impl<'a> SwitchesMapping<'a> for Mapping {
        type Switch = Switch;
        type Filtered = MappingBySwitch<'a>;

        fn filter_by_switch(&'a self, switch: Self::Switch) -> Option<Self::Filtered> {
            todo!();
        }
    }

    impl<'a> SwitchesSwitchMapping<'a> for MappingBySwitch<'a> {
        type Switch = Switch;
        type Filtered = MappingBySwitchAndModifiers<'a>;

        fn filter_by_modifiers(
            &'a self,
            modifiers: Modifiers<Self::Switch>,
        ) -> Option<Self::Filtered> {
            todo!();
        }
    }

    impl<'a> SwitchesSwitchModifiersMapping<'a> for MappingBySwitchAndModifiers<'a> {
        type Switch = Switch;
        type Event = Event;
        type Binding = Binding;

        fn get_event(
            &'a self,
            event: Self::Event,
        ) -> Vec<(Modifiers<Self::Switch>, Self::Binding)> {
            todo!();
        }
    }

    #[allow(single_use_lifetimes)] // false positive
    impl<'a> SwitchesMappingBinding<'a> for Binding {
        type Event = AppEvent;

        fn build(&'a self) -> Self::Event {
            todo!();
        }
    }

    /*let a: Option<MappingBySwitch<'_>> = Mapping::default()
    .0
    .get(&Switch::KeyboardKey("a"))
    .map(MappingBySwitch);*/

    /*impl SwitchesMapping<'a> for Mapping {
        type Switch = Switch;
        type Filtered: SwitchesSwitchMapping<'a, Switch = Self::Switch>;

        fn filter_by_switch(&'a self, switch: Self::Switch) -> Option<Self::Filtered>;
    }*/

    /*
        optimize processing using filtering:
            filter switch-used-in-event-or-modifiers | trigger-used-in-event
            modifiers-processing
            filter switch-used-in-event | trigger-used-in-event
            if pointer-trigger-used-in-events then pointer-processing
            if timed-trigger-used-in-events then timed-processing
    */
}
