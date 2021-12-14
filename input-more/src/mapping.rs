use core::borrow::Borrow;

use input_core::Modifiers;

pub trait SwitchesMapping<'a> {
    type Switch;
    type Filtered: SwitchesSwitchMapping<'a, Switch = Self::Switch>;

    fn filter_by_switch(&'a self, switch: Self::Switch) -> Option<Self::Filtered>;
}

pub trait SwitchesSwitchMapping<'a> {
    type Switch;
    type Binding: SwitchBinding<'a>;
    type BindingRef: Borrow<Self::Binding>;
    type Bindings: Iterator<Item = Self::BindingRef>;
    type Modifiers: Borrow<Modifiers<Self::Switch>>;
    type Filtered: Iterator<Item = (Self::Modifiers, Self::Bindings)>;
    /*SwitchesSwitchModifiersMapping<'a, Switch = Self::Switch>;*/

    fn filter_by_modifiers(&'a self, modifiers: Modifiers<Self::Switch>) -> Self::Filtered;
}

pub trait SwitchBinding<'a> {
    type Data;
    type Event;

    fn build(&'a self, event: Self::Data) -> Self::Event;
}

/*
pub trait SwitchesSwitchModifiersMapping<'a> {
    type Switch;
    type Event;
    type Binding: SwitchesMappingBinding<'a>;

    // (modifiers + bindings)* => modifiers + bidings*

    fn get_event(&'a self, event: Self::Event) -> Vec<(Modifiers<Self::Switch>, Self::Binding)>;
}

#[allow(single_use_lifetimes)] // false positive
pub trait SwitchesMappingBinding<'a> {
    type Event;

    fn build(&'a self) -> Self::Event;
}*/

#[test]
fn test() {
    use core::slice;
    use std::collections::{hash_map, HashMap};

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Event {
        KeyDown(&'static str, u32),
        KeyUp(&'static str, u32),
        ButtonDown(&'static str, u32),
        ButtonUp(&'static str, u32),
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Switch {
        KeyboardKey(&'static str),
        MouseButton(&'static str),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum Binding {
        AddNode(u32),
        RemoveNode,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum AppEvent {
        AddNode(u32, u32),
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
            self.0.get(&switch).map(MappingBySwitch)
        }
    }

    #[derive(Clone, Debug)]
    struct MappingBySwitchFilter<'a> {
        mapping: hash_map::Iter<'a, Modifiers<Switch>, Vec<Binding>>,
        modifiers: Modifiers<Switch>,
    };

    impl<'a> MappingBySwitchFilter<'a> {
        pub fn new(
            mapping: &'a HashMap<Modifiers<Switch>, Vec<Binding>>,
            modifiers: Modifiers<Switch>,
        ) -> Self {
            Self {
                mapping: mapping.iter(),
                modifiers,
            }
        }
    }

    impl<'a> Iterator for MappingBySwitchFilter<'a> {
        type Item = (&'a Modifiers<Switch>, slice::Iter<'a, Binding>);

        fn next(&mut self) -> Option<Self::Item> {
            self.mapping
                .find(|(binding_modifiers, _)| {
                    binding_modifiers
                        .switches()
                        .is_subset(self.modifiers.switches())
                })
                .map(|(binding_modifiers, bindings)| (binding_modifiers, bindings.iter()))
        }
    }

    impl<'a> SwitchesSwitchMapping<'a> for MappingBySwitch<'a> {
        type Switch = Switch;
        type Binding = Binding;
        type BindingRef = &'a Binding;
        type Bindings = slice::Iter<'a, Binding>;
        type Modifiers = &'a Modifiers<Self::Switch>;
        type Filtered = MappingBySwitchFilter<'a>;

        fn filter_by_modifiers(
            &'a self,
            event_modifiers: Modifiers<Self::Switch>,
        ) -> Self::Filtered {
            MappingBySwitchFilter::new(self.0, event_modifiers)
        }
    }

    impl<'a> SwitchBinding<'a> for Binding {
        type Data = Event;
        type Event = AppEvent;

        fn build(&'a self, event: Self::Data) -> Self::Event {
            let id = match event {
                Event::KeyDown(_, id) => id,
                Event::KeyUp(_, id) => id,
                Event::ButtonDown(_, id) => id,
                Event::ButtonUp(_, id) => id,
            };

            match self {
                Binding::AddNode(data) => AppEvent::AddNode(id, *data),
                Binding::RemoveNode => AppEvent::RemoveNode(id),
            }
        }
    }

    let mapping = Mapping(
        [
            (
                Switch::KeyboardKey("Space"),
                [
                    (
                        Modifiers::new().with_press_event(Switch::KeyboardKey("Ctrl")),
                        vec![Binding::AddNode(1)],
                    ),
                    (
                        Modifiers::new()
                            .with_press_event(Switch::KeyboardKey("Ctrl"))
                            .with_press_event(Switch::KeyboardKey("Alt")),
                        vec![Binding::AddNode(2)],
                    ),
                    (
                        Modifiers::new().with_press_event(Switch::KeyboardKey("Shift")),
                        vec![Binding::AddNode(3)],
                    ),
                ]
                .into_iter()
                .collect(),
            ),
            (
                Switch::MouseButton("Left"),
                [
                    (
                        Modifiers::new().with_press_event(Switch::KeyboardKey("Ctrl")),
                        vec![Binding::AddNode(4)],
                    ),
                    (
                        Modifiers::new()
                            .with_press_event(Switch::KeyboardKey("Ctrl"))
                            .with_press_event(Switch::KeyboardKey("Alt")),
                        vec![Binding::AddNode(5)],
                    ),
                    (
                        Modifiers::new().with_press_event(Switch::KeyboardKey("Shift")),
                        vec![Binding::AddNode(6)],
                    ),
                ]
                .into_iter()
                .collect(),
            ),
        ]
        .into_iter()
        .collect(),
    );

    let submapping = mapping
        .filter_by_switch(Switch::KeyboardKey("Space"))
        .unwrap();

    let result: Vec<_> = submapping
        .filter_by_modifiers(
            Modifiers::new()
                .with_press_event(Switch::KeyboardKey("Ctrl"))
                .with_press_event(Switch::KeyboardKey("Alt")),
        )
        .flat_map(|(_, bindings)| {
            bindings.map(|binding| binding.build(Event::KeyDown("Space", 123)))
        })
        .collect();

    println!("{:?}", result);
    panic!();

    /*
        optimize processing using filtering:
            filter switch-used-in-event-or-modifiers | trigger-used-in-event
            modifiers-processing
            filter switch-used-in-event | trigger-used-in-event
            if pointer-trigger-used-in-events then pointer-processing
            if timed-trigger-used-in-events then timed-processing
    */
}
