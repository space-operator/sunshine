/*
    TODO:
        (ev, modifiers)
            app: filter all mapping bindings by ev, modifires and context
        (mapping/bindings)
            input: (modifiers, mapping/bindings) => || => (bindings)

    ?
        Modifiers::match_modifiers(_: Binding)
        Binding::match_modifiers(_: Modifiers)
        trait Binding
            fn match(modifiers)
*/

//use core::iter::FusedIterator;
//use std::collections::HashSet;
//use std::vec::IntoIter;

/*

// Ctrl+Shift+Click
// Shift+Click
// Shift+Click

// Ctrl+Shift+Alt+A
// Win+A
// Ctrl+A
// Ctrl+Alt+A
// Ctrl+Alt+A
// Ctrl+Shift+A
// Ctrl+Alt+A   is sup Ctrl+A
// Ctrl+Shift+A is sup Ctrl+A
// Ctrl+Shift+A



#[derive(Clone, Debug)]
pub struct EventButtonMapping<T> {
    pub input: T,
    pub buttons: ModifiersButtons,
    pub events: Output
}

#[derive(Clone, Debug)]
pub struct EventAxesMapping<T> {
    pub input: T,
    pub buttons: ModifiersButtons,
    pub events: Output
}*/

/*
trait MappedEvent {
    type Output;

    fn event() -> EventWithModifiers<T> {

    }

    fn map()
}

trait Mapping<T> {
    type MappedEvent;



    fn process(mut self, ev: EventWithModifiers<T>) -> MappingIterator<T> {

    }
}

pub fn process(mut self, ev: EventWithModifiers<T>, ) -> MappingIterator<T> {

}

pub MappingIterator<T>(IntoIter<T>);

impl Iterator for MappingIterator<T> {
    type Item = T;
    fn next(&mut self) -> Self::Item {
        self.0.next()
    }
}

impl<T> FusedIterator for MappingIterator<T> {}


// input =>  => output


// Click, DblClick, SpacePressed

// Map MouseX => UiX
// Map TouchX => UiX
// DblClick => filter => map => SelectNode

// SelectNode(node)
// StartNodeTextEdit(node)
// DragStart(node, start)
// DragMove(node, start(x, y), current)
// DragEnd(node, start, last)
// DragCancel(node, start)


trait Mapping<T> {
    type MappedEvent;

    fn map(
        ev: EventWithModifiers<T>,
        mappings: Vec<(Self::MappedEvent, ModifiersFilter)>,
    ) -> IntoIter<MappedEvent>;
}
*/

pub trait MappedContext: Sized {
    type CustomEvent;
    type MappedEvent: Clone;

    fn events(
        &self,
        input: &CombinedInput<Self::CustomEvent>,
    ) -> Vec<(Self::MappedEvent, ModifiersFilter)>;

    fn emit(self, ev: Self::MappedEvent, axes: ModifiersAxes) -> Self;

    fn process(mut self, ev: CombinedEvent<Self::CustomEvent>) -> Self {
        let mappings = self.events(&ev.input);
        let mut mappings: Vec<_> = mappings
            .into_iter()
            .filter(|(_, modifiers)| {
                ev.modifiers.buttons.is_superset(&modifiers.buttons)
                    && modifiers.axes_ranges.iter().all(|(kind, range)| {
                        ev.modifiers
                            .axes
                            .get(kind)
                            .map_or(false, |axis| range.contains(axis))
                    })
            })
            .map(Some)
            .collect();

        for j1 in 0..mappings.len() {
            for j2 in 0..mappings.len() {
                if j1 != j2 {
                    if let (Some(binding1), Some(binding2)) = (&mappings[j1], &mappings[j2]) {
                        if binding1.1.buttons.is_superset(&binding2.1.buttons) {
                            mappings[j2] = None;
                        }
                    }
                }
            }
        }

        let modifier_button_sets: HashSet<_> = mappings
            .iter()
            .filter_map(Clone::clone)
            .map(|binding| -> Vec<_> { binding.1.buttons.iter().cloned().collect() })
            .collect();

        if modifier_button_sets.len() == 1 {
            for binding in mappings.into_iter().flatten() {
                self = self.emit(binding.0.clone(), ev.modifiers.axes.clone());
            }
        }
        self
    }
}

#[test]
fn input_mapping_test() {
    use crate::{
        ButtonKind, KeyboardKey, ModifiedInput, Modifiers, MouseButton, TimedInput, TriggerKind,
    };
    use std::collections::HashMap;
    use std::sync::Arc;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum AppEvent {
        A,
        B,
        C,
        D,
        E,
        F,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum CustomEvent {
        CoolGesture,
        SmartGesture,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    struct InputMapping {
        event: AppEvent,
        input: CombinedInput<CustomEvent>,
        buttons: Vec<ButtonKind>,
    }

    impl InputMapping {
        pub fn new(
            event: AppEvent,
            input: CombinedInput<CustomEvent>,
            buttons: Vec<ButtonKind>,
        ) -> Self {
            Self {
                event,
                input,
                buttons,
            }
        }
    }

    #[derive(Clone, Debug)]
    struct Context {
        mappings: HashSet<InputMapping>,
        events: Vec<(AppEvent, ModifiersAxes)>,
    }

    let lmb = || {
        CombinedInput::Timed(TimedInput::Click {
            button: ButtonKind::MouseButton(MouseButton::Primary),
            num_clicks: 1,
        })
    };
    let rmb = || {
        CombinedInput::Timed(TimedInput::Click {
            button: ButtonKind::MouseButton(MouseButton::Secondary),
            num_clicks: 1,
        })
    };
    let cool_gesture = || {
        CombinedInput::Modified(ModifiedInput::Trigger(TriggerKind::Custom(
            CustomEvent::CoolGesture,
        )))
    };

    let smart_gesture = || {
        CombinedInput::Modified(ModifiedInput::Trigger(TriggerKind::Custom(
            CustomEvent::SmartGesture,
        )))
    };

    let ctrl = || ButtonKind::KeyboardKey(KeyboardKey("LeftCtrl".to_owned()));
    let shift = || ButtonKind::KeyboardKey(KeyboardKey("LeftShift".to_owned()));
    let alt = || ButtonKind::KeyboardKey(KeyboardKey("LeftAlt".to_owned()));

    let context = Context {
        mappings: [
            InputMapping::new(AppEvent::A, lmb(), vec![ctrl()]),
            InputMapping::new(AppEvent::B, lmb(), vec![shift()]),
            InputMapping::new(AppEvent::C, lmb(), vec![ctrl(), alt()]),
            InputMapping::new(AppEvent::A, lmb(), vec![ctrl()]),
            InputMapping::new(AppEvent::D, rmb(), vec![ctrl()]),
            InputMapping::new(AppEvent::E, cool_gesture(), vec![]),
            InputMapping::new(AppEvent::F, smart_gesture(), vec![]),
        ]
        .into_iter()
        .collect(),
        events: Vec::new(),
    };

    impl MappedContext for Context {
        type CustomEvent = CustomEvent;
        type MappedEvent = AppEvent;

        fn events(
            &self,
            input: &CombinedInput<Self::CustomEvent>,
        ) -> Vec<(Self::MappedEvent, ModifiersFilter)> {
            self.mappings
                .iter()
                .filter(|mapping| mapping.input == *input)
                .map(|mapping| {
                    (
                        mapping.event.clone(),
                        ModifiersFilter {
                            buttons: mapping.buttons.iter().cloned().collect(),
                            axes_ranges: HashMap::new(),
                        },
                    )
                })
                .collect()
        }

        fn emit(mut self, ev: Self::MappedEvent, axes: ModifiersAxes) -> Self {
            self.events.push((ev, axes));
            self
        }
    }

    let mut context = context.process(CombinedEvent {
        modifiers: Arc::new(Modifiers::default()),
        input: lmb(),
    });
    dbg!(&context.events);
    context.events.clear();

    let mut context = context.process(CombinedEvent {
        modifiers: Arc::new(Modifiers::default()),
        input: rmb(),
    });
    dbg!(&context.events);
    context.events.clear();

    let mut context = context.process(CombinedEvent {
        modifiers: Arc::new(Modifiers {
            buttons: [ctrl()].into_iter().collect(),
            axes: HashMap::new(),
        }),
        input: lmb(),
    });
    dbg!(&context.events);
    context.events.clear();

    let mut context = context.process(CombinedEvent {
        modifiers: Arc::new(Modifiers {
            buttons: [ctrl(), alt()].into_iter().collect(),
            axes: HashMap::new(),
        }),
        input: lmb(),
    });
    dbg!(&context.events);
    context.events.clear();

    let mut context = context.process(CombinedEvent {
        modifiers: Arc::new(Modifiers {
            buttons: [ctrl(), shift()].into_iter().collect(),
            axes: HashMap::new(),
        }),
        input: lmb(),
    });
    dbg!(&context.events);
    context.events.clear();

    let mut context = context.process(CombinedEvent {
        modifiers: Arc::new(Modifiers {
            buttons: [shift()].into_iter().collect(),
            axes: HashMap::new(),
        }),
        input: lmb(),
    });
    dbg!(&context.events);
    context.events.clear();

    let mut context = context.process(CombinedEvent {
        modifiers: Arc::new(Modifiers {
            buttons: HashSet::new(),
            axes: HashMap::new(),
        }),
        input: cool_gesture(),
    });
    dbg!(&context.events);
    context.events.clear();

    let mut context = context.process(CombinedEvent {
        modifiers: Arc::new(Modifiers {
            buttons: HashSet::new(),
            axes: HashMap::new(),
        }),
        input: smart_gesture(),
    });
    dbg!(&context.events);
    context.events.clear();
}
