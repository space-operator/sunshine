use core::borrow::Borrow;
use core::hash::Hash;
use core::marker::PhantomData;

use input_core::Modifiers;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClickOrDragData;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SwitchEventMore<Sw, Mo>(Sw, Modifiers<Mo>, ClickOrDragData);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TriggerEventMore<Tr, Mo>(Tr, Modifiers<Mo>, ClickOrDragData);

pub trait SwitchMapping<'a, Sw, Mo, Co> {
    type Filtered: SwitchMappingSwitchSubset<'a, Sw, Mo, Co>;

    fn filter_by_switch(&'a self, switch: Sw) -> Option<Self::Filtered>;
}

pub trait SwitchMappingSwitchSubset<'a, Sw, Mo, Co> {
    type Filtered: SwitchMappingModifiersSubset<'a, Sw, Mo, Co> + MaybePointerEvent;

    fn filter_by_modifiers(&'a self, modifiers: Modifiers<Mo>) -> Option<Self::Filtered>;
}

// device + switch + coords
// device + coords on move

pub trait SwitchMappingModifiersSubset<'a, Sw, Mo, Co> {
    type Binding: SwitchBinding<'a>;
    type BindingRef: Borrow<Self::Binding>;
    type Bindings: Iterator<Item = Self::BindingRef>;
    type Modifiers: Borrow<Modifiers<Mo>>;
    type Filtered: Iterator<Item = (Self::Modifiers, Self::Bindings)>;
    /*SwitchesSwitchModifiersMapping<'a, Switch = Self::Switch>;*/

    fn filter_by_event(&'a self, event: SwitchEventMore<Sw, Mo>) -> Self::Filtered;
}

pub trait SwitchBinding<'a> {
    type Event;

    fn build(&'a self) -> Self::Event;
}

/*
    mapping
    input_more
        ....
            mapping.filter_by_switch()
            mapping.filter_by_modifiers()
                SwitchMappingModifiersSubset
                OptionalPointerProcessing
                    NoPointerProcessing


            mapping.filter_by_event()

*/

/*
    trait IsPointerEvent {
        const IS_POINTER_EVENT: bool;
    }
*/

// show trick about unused generics for auto generics

pub trait MaybePointerEvent {
    type Wrapper: PointerEventProcessing;
    fn pointer_event(self) -> Self::Wrapper;
}

pub trait PointerEventProcessing {
    // implemented only in library
    type PointerState;
    fn with_change_event();
}

impl PointerEventProcessing for () {
    type PointerState = ();
    fn with_change_event() {}
}

impl<'a, T: PointerEvent<'a>> PointerEventProcessing for (T,) {
    type PointerState = i32;
    fn with_change_event() {
        // do smth
    }
}

pub trait PointerEvent<'a> {
    type Coords;
    fn get_coords(&'a self) -> Self::Coords;
    fn is_drag_start(&'a self, coords: Self::Coords) -> bool;
}

struct MappingSubset<Sw>(Sw);
struct KeyboardSwitch(i32);
struct MouseSwitch(i8);

impl<'a> PointerEvent<'a> for MappingSubset<MouseSwitch> {
    type Coords = (u32, u32);
    fn get_coords(&'a self) -> Self::Coords {
        (1, 2)
    }
    fn is_drag_start(&'a self, coords: Self::Coords) -> bool {
        false
    }
}

impl MaybePointerEvent for MappingSubset<KeyboardSwitch> {
    type Wrapper = ();
    fn pointer_event(self) -> Self::Wrapper {
        ()
    }
}

impl MaybePointerEvent for MappingSubset<MouseSwitch> {
    type Wrapper = (Self,);
    fn pointer_event(self) -> Self::Wrapper {
        (self,)
    }
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

/*
#[test]
fn test() {
    use core::slice;
    use std::collections::{hash_map, HashMap};

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum SimpleEvent {
        KeyDown(&'static str),
        KeyUp(&'static str),
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum PointerEvent {
        ButtonDown(&'static str, i32),
        ButtonUp(&'static str, i32),
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum SimpleSwitch {
        KeyboardKey(&'static str),
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum PointerSwitch {
        MouseButton(&'static str),
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Switch {
        KeyboardKey(&'static str),
        MouseButton(&'static str),
    }

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct SimpleEventData;

    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct PointerEventData(i32);

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum SimpleBinding {
        Save(u32),
        Undo,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum PointerBinding {
        AddNode(u32),
        RemoveNode,
        Save(u32),
        Undo,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum AppEvent {
        AddNode(u32, i32),
        RemoveNode(i32),
        Save(u32),
        Undo,
    }

    #[derive(Clone, Debug, Default)]
    struct Mapping<Sw, Bi>(
        HashMap<Sw, HashMap<Modifiers<Switch>, Vec<Bi>>>,
        PhantomData<Sw>,
    );

    //type SimpleMapping = Mapping<SimpleSwitch, SimpleBinding>;
    //type PointerMapping = Mapping<PointerSwitch, PointerBinding>;

    #[derive(Clone, Debug)]
    struct MappingBySwitch<'a, Sw, Bi>(&'a HashMap<Modifiers<Switch>, Vec<Bi>>, PhantomData<Sw>);

    //type SimpleMappingBySwitch<'a> = MappingBySwitch<'a, SimpleSwitch, SimpleBinding>;
    //type PointerMappingBySwitch<'a> = MappingBySwitch<'a, PointerSwitch, PointerBinding>;

    #[derive(Clone, Debug)]
    struct MappingBySwitchAndModifiers<'a, Sw, Bi>(
        HashMap<&'a Modifiers<Switch>, &'a [Bi]>,
        PhantomData<Sw>,
    );

    //type SimpleMappingBySwitchAndModifiers<'a> =
    //    MappingBySwitchAndModifiers<'a, SimpleSwitch, SimpleBinding>;
    //type PointerMappingBySwitchAndModifiers<'a> =
    //    MappingBySwitchAndModifiers<'a, PointerSwitch, PointerBinding>;

    #[derive(Clone, Debug)]
    struct MappingBySwitchFilter<'a, Sw, Bi> {
        mapping: hash_map::Iter<'a, Modifiers<Sw>, Vec<Bi>>,
        modifiers: Modifiers<Sw>,
        _marker: PhantomData<(Sw, Bi)>,
    }

    //type SimpleMappingBySwitchFilter<'a> = MappingBySwitchFilter<'a, SimpleSwitch, SimpleBinding>;
    //type PointerMappingBySwitchFilter<'a> =
    //    MappingBySwitchFilter<'a, PointerSwitch, PointerBinding>;

    impl<'a, Da, Sw: Eq + Hash, Bi: 'a> SwitchMapping<'a, Da> for Mapping<Sw, Bi>
    where
        MappingBySwitch<'a, Sw, Bi>: SwitchMappingFilteredBySwitch<'a, Da>,
    {
        type Switch = Sw;
        type Filtered = MappingBySwitch<'a, Sw, Bi>;

        fn filter_by_switch(&'a self, switch: Self::Switch) -> Option<Self::Filtered> {
            self.0.get(&switch).map(MappingBySwitch::new)
        }
    }

    impl<'a, Sw, Bi> MappingBySwitch<'a, Sw, Bi> {
        fn new(data: &'a HashMap<Modifiers<Switch>, Vec<Bi>>) -> Self {
            Self(data, PhantomData)
        }
    }

    impl<'a, Da, Sw: Ord, Bi> SwitchMappingFilteredBySwitch<'a, Da> for MappingBySwitch<'a, Sw, Bi>
    where
        Bi: SwitchBinding<'a, Da>,
    {
        type ModifiersSwitch = Switch;
        type Binding = Bi;
        type BindingRef = &'a Bi;
        type Bindings = slice::Iter<'a, Bi>;
        type Modifiers = &'a Modifiers<Self::ModifiersSwitch>;
        type Filtered = MappingBySwitchFilter<'a, Self::ModifiersSwitch, Bi>;

        fn filter_by_modifiers(
            &'a self,
            event_modifiers: Modifiers<Self::ModifiersSwitch>,
        ) -> Self::Filtered {
            MappingBySwitchFilter::new(self.0, event_modifiers)
        }
    }

    impl<'a, Sw, Bi> MappingBySwitchFilter<'a, Sw, Bi> {
        pub fn new(mapping: &'a HashMap<Modifiers<Sw>, Vec<Bi>>, modifiers: Modifiers<Sw>) -> Self {
            Self {
                mapping: mapping.iter(),
                modifiers,
                _marker: PhantomData,
            }
        }
    }

    impl<'a, Sw: Ord, Bi: 'a> Iterator for MappingBySwitchFilter<'a, Sw, Bi> {
        type Item = (&'a Modifiers<Sw>, slice::Iter<'a, Bi>);

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

    impl<'a, Sw, Bi> MappingBySwitchAndModifiers<'a, Sw, Bi> {
        fn new(data: HashMap<&'a Modifiers<Switch>, &'a [Bi]>) -> Self {
            Self(data, PhantomData)
        }
    }

    impl<'a> SwitchBinding<'a, PointerEventData> for PointerBinding {
        type Event = AppEvent;

        fn build(&'a self, data: PointerEventData) -> Self::Event {
            let coords = data.0;

            match self {
                PointerBinding::AddNode(data) => AppEvent::AddNode(*data, coords),
                PointerBinding::RemoveNode => AppEvent::RemoveNode(coords),
                PointerBinding::Save(data) => AppEvent::Save(*data),
                PointerBinding::Undo => AppEvent::Undo,
            }
        }
    }

    impl<'a> SwitchBinding<'a, SimpleEventData> for SimpleBinding {
        type Event = AppEvent;

        fn build(&'a self, data: SimpleEventData) -> Self::Event {
            match self {
                SimpleBinding::Save(data) => AppEvent::Save(*data),
                SimpleBinding::Undo => AppEvent::Undo,
            }
        }
    }

    let simple_mapping = Mapping(
        [(
            SimpleSwitch::KeyboardKey("Space"),
            [
                (
                    Modifiers::new().with_press_event(Switch::KeyboardKey("Ctrl")),
                    vec![SimpleBinding::Save(1)],
                ),
                (
                    Modifiers::new()
                        .with_press_event(Switch::KeyboardKey("Ctrl"))
                        .with_press_event(Switch::KeyboardKey("Alt")),
                    vec![SimpleBinding::Save(2)],
                ),
                (
                    Modifiers::new().with_press_event(Switch::KeyboardKey("Shift")),
                    vec![SimpleBinding::Save(3)],
                ),
            ]
            .into_iter()
            .collect(),
        )]
        .into_iter()
        .collect(),
        PhantomData,
    );

    let pointer_mapping = Mapping(
        [(
            PointerSwitch::MouseButton("Left"),
            [
                (
                    Modifiers::new().with_press_event(Switch::KeyboardKey("Ctrl")),
                    vec![PointerBinding::AddNode(4)],
                ),
                (
                    Modifiers::new()
                        .with_press_event(Switch::KeyboardKey("Ctrl"))
                        .with_press_event(Switch::KeyboardKey("Alt")),
                    vec![PointerBinding::AddNode(5)],
                ),
                (
                    Modifiers::new().with_press_event(Switch::KeyboardKey("Shift")),
                    vec![PointerBinding::AddNode(6)],
                ),
            ]
            .into_iter()
            .collect(),
        )]
        .into_iter()
        .collect(),
        PhantomData,
    );

    {
        let submapping = simple_mapping
            .filter_by_switch(SimpleSwitch::KeyboardKey("Space"))
            .unwrap();

        let result: Vec<_> = submapping
            .filter_by_modifiers(
                Modifiers::new()
                    .with_press_event(Switch::KeyboardKey("Ctrl"))
                    .with_press_event(Switch::KeyboardKey("Alt")),
            )
            .flat_map(|(_, bindings)| bindings.map(|binding| binding.build(SimpleEventData)))
            .collect();

        println!("{:?}", result);
    }

    {
        let submapping = pointer_mapping
            .filter_by_switch(PointerSwitch::MouseButton("Left"))
            .unwrap();

        let result: Vec<_> = submapping
            .filter_by_modifiers(
                Modifiers::new()
                    .with_press_event(Switch::KeyboardKey("Ctrl"))
                    .with_press_event(Switch::KeyboardKey("Alt")),
            )
            .flat_map(|(_, bindings)| bindings.map(|binding| binding.build(PointerEventData(123))))
            .collect();

        println!("{:?}", result);
    }
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
*/
