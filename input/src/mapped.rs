use std::collections::HashSet;

use crate::{CombinedEvent, CombinedInput, Event, ModifiersFilter};

pub trait MappedContext: Sized {
    type CustomEvent;
    type MappedEvent: Clone;

    fn events(
        &self,
        input: &CombinedInput<Self::CustomEvent>,
    ) -> Vec<(Self::MappedEvent, ModifiersFilter)>;

    // TODO: emit()

    fn process<F>(&self, ev: CombinedEvent<Self::CustomEvent>, mut emit_fn: F)
    where
        F: FnMut(Event<Self::MappedEvent>),
    {
        let mappings = self.events(&ev.input);
        let mut mappings: Vec<_> = mappings
            .into_iter()
            .filter(|(_, modifiers)| {
                ev.modifiers.buttons.is_superset(&modifiers.buttons)
                    && modifiers.axes_ranges.iter().all(|(kind, range)| {
                        ev.modifiers
                            .axes
                            .get(kind)
                            .map(|axis| range.contains(axis))
                            .unwrap_or(false)
                    })
            })
            .map(Option::Some)
            .collect();

        for j1 in 0..mappings.len() {
            for j2 in 0..mappings.len() {
                if j1 != j2 {
                    match (&mappings[j1], &mappings[j2]) {
                        (Some(binding1), Some(binding2)) => {
                            if binding1.1.buttons.is_superset(&binding2.1.buttons) {
                                mappings[j2] = None;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let buttons: HashSet<_> = mappings
            .iter()
            .filter_map(|binding| binding.clone())
            .map(|binding| -> Vec<_> { binding.1.buttons.iter().cloned().collect() })
            .collect();

        if buttons.len() == 1 {
            for binding in mappings.into_iter().filter_map(|binding| binding) {
                emit_fn(Event {
                    input: binding.0.clone(),
                    timestamp: ev.timestamp,
                })
            }
        }
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
    struct InputMappings(HashSet<InputMapping>);

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

    let ctrl = || ButtonKind::KeyboardKey(KeyboardKey::LeftCtrl);
    let shift = || ButtonKind::KeyboardKey(KeyboardKey::LeftShift);
    let alt = || ButtonKind::KeyboardKey(KeyboardKey::LeftAlt);

    let mappings = InputMappings(
        [
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
    );

    impl MappedContext for InputMappings {
        type CustomEvent = CustomEvent;
        type MappedEvent = AppEvent;

        fn events(
            &self,
            input: &CombinedInput<Self::CustomEvent>,
        ) -> Vec<(Self::MappedEvent, ModifiersFilter)> {
            self.0
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
    }

    let mut events = vec![];

    mappings.process(
        CombinedEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers::default()),
            input: lmb(),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    mappings.process(
        CombinedEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers::default()),
            input: rmb(),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    mappings.process(
        CombinedEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: [ButtonKind::KeyboardKey(KeyboardKey::LeftCtrl)]
                    .into_iter()
                    .collect(),
                axes: HashMap::new(),
            }),
            input: lmb(),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    mappings.process(
        CombinedEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: [
                    ButtonKind::KeyboardKey(KeyboardKey::LeftCtrl),
                    ButtonKind::KeyboardKey(KeyboardKey::LeftAlt),
                ]
                .into_iter()
                .collect(),
                axes: HashMap::new(),
            }),
            input: lmb(),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    mappings.process(
        CombinedEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: [
                    ButtonKind::KeyboardKey(KeyboardKey::LeftCtrl),
                    ButtonKind::KeyboardKey(KeyboardKey::LeftShift),
                ]
                .into_iter()
                .collect(),
                axes: HashMap::new(),
            }),
            input: lmb(),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    mappings.process(
        CombinedEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: [ButtonKind::KeyboardKey(KeyboardKey::LeftShift)]
                    .into_iter()
                    .collect(),
                axes: HashMap::new(),
            }),
            input: lmb(),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    mappings.process(
        CombinedEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: HashSet::new(),
                axes: HashMap::new(),
            }),
            input: cool_gesture(),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    mappings.process(
        CombinedEvent {
            timestamp: 1000,
            modifiers: Arc::new(Modifiers {
                buttons: HashSet::new(),
                axes: HashMap::new(),
            }),
            input: smart_gesture(),
        },
        |ev| events.push(ev),
    );
    dbg!(&events);
    events.clear();

    // TODO: check states
}
