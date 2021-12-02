#![feature(map_first_last)]

use std::collections::HashSet;

#[test]
fn raw_input_to_input_test() {
    /*
        use input_processor::*;
        use std::{collections::BTreeMap, sync::Arc, sync::Weak};

        type TimestampMs = u64;
        type Coords = (u64, u64);

        type ImmediateEvent = (Option<RawEvent>, Option<AggregateTimedEvent<EventSwitch>>);

        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        enum RawEvent {
            KeyboardDown(&'static str, TimestampMs),
            KeyboardUp(&'static str, TimestampMs),
            MouseDown(&'static str, Coords, TimestampMs),
            MouseUp(&'static str, Coords, TimestampMs),
            MouseMove(Coords, TimestampMs),
        }

        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        enum EventSwitch {
            Key(&'static str),
            Button(&'static str),
        }

        impl EventWithAction for RawEvent {
            type Switch = EventSwitch;

            fn action(&self) -> Option<Action<Self::Switch>> {
                match self {
                    RawEvent::KeyboardDown(switch, _) => Some(Action::Enable(EventSwitch::Key(switch))),
                    RawEvent::KeyboardUp(switch, _) => Some(Action::Disable(EventSwitch::Key(switch))),
                    RawEvent::MouseDown(switch, _, _) => {
                        Some(Action::Enable(EventSwitch::Button(switch)))
                    }
                    RawEvent::MouseUp(switch, _, _) => {
                        Some(Action::Disable(EventSwitch::Button(switch)))
                    }
                    RawEvent::MouseMove(_, _) => None,
                }
            }
        }

        impl RawEvent {
            fn timestamp(&self) -> TimestampMs {
                match self {
                    Self::KeyboardDown(_, timestamp) => *timestamp,
                    Self::KeyboardUp(_, timestamp) => *timestamp,
                    Self::MouseDown(_, _, timestamp) => *timestamp,
                    Self::MouseUp(_, _, timestamp) => *timestamp,
                    Self::MouseMove(_, timestamp) => *timestamp,
                }
            }
        }

        #[derive(Clone, Debug)]
        struct State {
            state: TimedState<EventSwitch>,
            timeouts: BTreeMap<TimestampMs, Weak<ScheduledTransition<EventSwitch>>>,
            last_timestamp: TimestampMs,
        }

        impl State {
            fn with_event(self, event: RawEvent) -> (Self, Vec<ImmediateEvent>) {
                fn apply_timed_transition(
                    mut events: Vec<ImmediateEvent>,
                    mut timeouts: BTreeMap<TimestampMs, Weak<ScheduledTransition<EventSwitch>>>,
                    transition: AggregateTimedTransition<RawEvent, EventSwitch>,
                    timestamp: TimestampMs,
                ) -> (
                    Vec<ImmediateEvent>,
                    TimedState<EventSwitch>,
                    BTreeMap<TimestampMs, Weak<ScheduledTransition<EventSwitch>>>,
                ) {
                    //println!("E {:?}", transition.event);
                    //println!("T {:?}", transition.timed_event);
                    //println!("S {:?}", transition.state);
                    events.push((transition.event, transition.timed_event));
                    if let Some(scheduled) = transition.scheduled {
                        let delay = match scheduled.duration {
                            Duration::LongClick => 1000,
                            Duration::MultiClick => 300,
                        };
                        let _ =
                            timeouts.insert(timestamp + delay, Arc::downgrade(&scheduled.transition));
                    }
                    (events, transition.state, timeouts)
                }

                println!();
                println!("{:?}", event);

                let timestamp = event.timestamp();
                let mut state = self.state;
                let mut timeouts = self.timeouts;
                assert!(self.last_timestamp < timestamp);
                let last_timestamp = timestamp;

                let mut events = Vec::new();

                while let Some(entry) = timeouts.first_entry() {
                    if *entry.key() > timestamp {
                        break;
                    }
                    let (_, timeout) = entry.remove_entry();
                    if let Some(timeout) = timeout.upgrade() {
                        let transition = state.with_timeout_event(timeout).unwrap().into_aggregate();
                        let result = apply_timed_transition(events, timeouts, transition, timestamp);
                        events = result.0;
                        state = result.1;
                        timeouts = result.2;
                    }
                }

                let transition = state.with_event(event.clone()).unwrap().into_aggregate();
                let result = apply_timed_transition(events, timeouts, transition, timestamp);
                events = result.0;
                state = result.1;
                timeouts = result.2;

                println!("{:?}", events);

                (
                    Self {
                        state,
                        timeouts,
                        last_timestamp,
                    },
                    events,
                )
            }
        }

        let state = State {
            state: TimedState::new(),
            timeouts: BTreeMap::new(),
            last_timestamp: 0,
        };

        let state = state
            .with_event(RawEvent::KeyboardDown("LeftCtrl", 10000))
            .0;
        let state = state.with_event(RawEvent::KeyboardUp("LeftCtrl", 10500)).0;
        let state = state
            .with_event(RawEvent::KeyboardDown("LeftCtrl", 11000))
            .0;
        let state = state.with_event(RawEvent::KeyboardUp("LeftCtrl", 13000)).0;

        let state = state
            .with_event(RawEvent::KeyboardDown("LeftCtrl", 15000))
            .0;
        let state = state
            .with_event(RawEvent::MouseDown("LeftMouseButton", (0, 0), 15100))
            .0;
        let state = state
            .with_event(RawEvent::MouseUp("LeftMouseButton", (0, 0), 15200))
            .0;
        let state = state
            .with_event(RawEvent::MouseDown("LeftMouseButton", (0, 0), 15300))
            .0;
        let state = state
            .with_event(RawEvent::MouseUp("LeftMouseButton", (0, 0), 15400))
            .0;

        let state = state.with_event(RawEvent::KeyboardUp("LeftCtrl", 18000)).0;

        let _ = state;
        panic!();
    */
    /*
        binding
            binding-kind + attached-data(coords?)

        raw-event
            switch? + binding + timestamp

        {
            raw-event | timed-state > immediate-timed-event
                switch + kind + num
            timeout | timed-state > delayed-timed-event
                switch + kind + num

            immediate-timed-event + delayed-timed-event > aggregated-timed-event
            event + aggregated-timed-event > aggregated-timed-event-with-data
                switch + kind + num + binding + timestamp
        }
            =>
        {
            press-switch | timed-state > (/) + delayed
            release-switch | timed-state > release-timed-event + delayed
            delayed + switch | timed-state > delayed-timed-event

            raw-event-part + (release-timed-event | delayed-timed-event) > aggregated-timed-event
            raw-event | aggregated-timed-event > combined-timed-event
                switch? + binding + timestamp + kind? + num?
        }

        raw-event + aggregated-timed-event-with-data > immediate-event
            switch? + kind? + num? + binding + timestamp

        immediate-event | modified-state > event-with-modifiers
            switch? + kind? + num? + binding + timestamp + modifiers

        event-with-modifiers | app > filtered-event-with-modifiers
            switch? + kind? + num? + binding + timestamp + modifiers + app-event-data

        filtered-event-with-modifiers | helpers > filtered2-event-with-modifiers
            app-event-data

        app-event-data | build > app-event
    */

    /*
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        enum BindingWithCoords {
            MouseDown(&'static str),
            MouseUp(&'static str),
            MouseMove,
        }

        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        enum BindingWithoutCoords {
            KeyboardDown(&'static str),
            KeyboardUp(&'static str),
        }

        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        enum Binding {
            KeyboardDown(&'static str),
            KeyboardUp(&'static str),
            MouseDown(&'static str),
            MouseUp(&'static str),
            MouseMove,
        }
    impl RawEvent {
        fn binding(&self) -> Binding {
            match self {
                RawEvent::KeyboardDown(switch, _) => Binding::KeyboardDown(switch),
                RawEvent::KeyboardUp(switch, _) => Binding::KeyboardUp(switch),
                RawEvent::MouseDown(switch, _, _) => Binding::MouseDown(switch),
                RawEvent::MouseUp(switch, _, _) => Binding::MouseUp(switch),
                RawEvent::MouseMove(_, _) => Binding::MouseMove,
            }
        }
    }
    */

    /*
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        enum AppEventKindWithCoords {
            SelectNode,
            CreateNode,
        }

        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        enum AppEventWithCoords {
            SelectNode(Coords),
            CreateNode(Coords),
        }

        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        enum AppEventWithoutCoords {
            UnselectNode,
        }

    */
    /*#[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum AppEventKind {
        SelectNode,
        CreateNode,
        UnselectNode,
    }

    impl AppEventKind {
        fn build(self, coords: Option<Coords>) -> AppEvent {
            match self {
                AppEvent::SelectNode => AppEvent
                AppEvent::CreateNode => AppEvent
                AppEvent::UnselectNode => AppEvent
            }
        }
    }*/

    /*
    enum InputEvent {
        Event(RawEvent),
        TimedEvent(AggregateTimedEventKind),
    }

        struct MappingWithCoords {
            binding: BindingWithCoords,
            modifiers: HashSet<EventSwitch>,
            event: AppEventKindWithCoords,
        }
    */
}
