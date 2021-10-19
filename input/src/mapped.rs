use std::collections::HashSet;

use crate::{CombinedEvent, CombinedInput, Event, Modifiers};

pub trait MappedContext: Sized {
    type Event: Clone + core::fmt::Debug;

    fn events(&self, input: &CombinedInput) -> Vec<(Self::Event, Modifiers)>;

    fn process<F: FnMut(Event<Self::Event>)>(&self, ev: CombinedEvent, mut emit_fn: F) {
        let mappings = self.events(&ev.input);
        let mut mappings: Vec<_> = mappings
            .into_iter()
            .filter(|(_, modifiers)| {
                ev.modifiers.buttons.is_superset(&modifiers.buttons)
                /* TODO: for axes */
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
                /* TODO: for axes */
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
    use crate::{ButtonKind, KeyboardKey, MouseButton, TimedInput};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum AppEvent {
        A,
        B,
        C,
        D,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    struct InputMapping {
        event: AppEvent,
        input: CombinedInput,
        buttons: Vec<ButtonKind>,
    }

    impl InputMapping {
        pub fn new(event: AppEvent, input: CombinedInput, buttons: Vec<ButtonKind>) -> Self {
            Self {
                event,
                input,
                buttons,
            }
        }
    }

    #[derive(Clone, Debug)]
    struct InputMappings(HashSet<InputMapping>);

    #[derive(Clone, Debug)]
    struct InputMappings2(HashMap<CombinedInput, Vec<(Modifiers, AppEvent)>>);

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
        ]
        .into_iter()
        .collect(),
    );

    impl MappedContext for InputMappings {
        type Event = AppEvent;

        fn events(&self, input: &CombinedInput) -> Vec<(Self::Event, Modifiers)> {
            self.0
                .iter()
                .filter(|mapping| mapping.input == *input)
                .map(|mapping| {
                    (
                        mapping.event.clone(),
                        Modifiers {
                            buttons: mapping.buttons.iter().cloned().collect(),
                            axes: HashMap::new(),
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

    panic!();
}
