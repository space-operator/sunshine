#![feature(map_first_last)]

use std::collections::{BTreeSet, HashMap};

#[test]
fn raw_input_to_input_test() {
    use input_core::*;
    use input_more::*;
    use std::collections::{BTreeMap, HashSet};

    type TimestampMs = u64;
    type Coords = (u64, u64);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum RawAction {
        Press(RawSwitch),
        Release(RawSwitch),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum RawEvent {
        KeyboardDown(&'static str, TimestampMs),
        KeyboardUp(&'static str, TimestampMs),
        MouseDown(&'static str, Coords, TimestampMs),
        MouseUp(&'static str, Coords, TimestampMs),
        MouseMove(Coords, TimestampMs),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum RawSwitchEvent {
        KeyboardDown(&'static str, TimestampMs),
        KeyboardUp(&'static str, TimestampMs),
        MouseDown(&'static str, Coords, TimestampMs),
        MouseUp(&'static str, Coords, TimestampMs),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum RawSwitch {
        Key(&'static str),
        Button(&'static str),
    }

    impl RawAction {
        fn switch(&self) -> &RawSwitch {
            match self {
                Self::Press(switch) => &switch,
                Self::Release(switch) => &switch,
            }
        }
    }

    impl RawSwitchEvent {
        fn action(&self) -> RawAction {
            match self {
                Self::KeyboardDown(switch, _) => RawAction::Press(RawSwitch::Key(switch)),
                Self::KeyboardUp(switch, _) => RawAction::Release(RawSwitch::Key(switch)),
                Self::MouseDown(switch, _, _) => RawAction::Press(RawSwitch::Button(switch)),
                Self::MouseUp(switch, _, _) => RawAction::Release(RawSwitch::Button(switch)),
            }
        }
    }

    /*impl RawEvent {
        fn action(&self) -> Option<RawAction> {
            RawSwitchEvent::try_from(*self)
                .as_ref()
                .map(RawSwitchEvent::action)
                .ok()
        }
    }*/

    impl TryFrom<RawEvent> for RawSwitchEvent {
        type Error = ();
        fn try_from(event: RawEvent) -> Result<Self, Self::Error> {
            match event {
                RawEvent::KeyboardDown(a, b) => Ok(Self::KeyboardDown(a, b)),
                RawEvent::KeyboardUp(a, b) => Ok(Self::KeyboardUp(a, b)),
                RawEvent::MouseDown(a, b, c) => Ok(Self::MouseDown(a, b, c)),
                RawEvent::MouseUp(a, b, c) => Ok(Self::MouseUp(a, b, c)),
                RawEvent::MouseMove(_, _) => Err(()),
            }
        }
    }

    impl From<RawSwitchEvent> for RawEvent {
        fn from(event: RawSwitchEvent) -> Self {
            match event {
                RawSwitchEvent::KeyboardDown(a, b) => Self::KeyboardDown(a, b),
                RawSwitchEvent::KeyboardUp(a, b) => Self::KeyboardUp(a, b),
                RawSwitchEvent::MouseDown(a, b, c) => Self::MouseDown(a, b, c),
                RawSwitchEvent::MouseUp(a, b, c) => Self::MouseUp(a, b, c),
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

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum BasicEvent {
        KeyboardDown(&'static str, TimestampMs),
        KeyboardUp(&'static str, TimestampMs),
        KeyboardTimed(&'static str, TimedCombinedEventData, TimestampMs),
        MouseDown(&'static str, Coords, TimestampMs),
        MouseUp(&'static str, Coords, TimestampMs),
        MouseTimed(&'static str, TimedCombinedEventData, Coords, TimestampMs),
        MouseMove(Coords, TimestampMs),
    }

    impl BasicEvent {
        fn action(&self) -> Option<RawAction> {
            match self {
                Self::KeyboardDown(switch, _) => Some(RawAction::Press(RawSwitch::Key(switch))),
                Self::KeyboardUp(switch, _) => Some(RawAction::Release(RawSwitch::Key(switch))),
                Self::KeyboardTimed(_, _, _) => None,
                Self::MouseDown(switch, _, _) => Some(RawAction::Press(RawSwitch::Button(switch))),
                Self::MouseUp(switch, _, _) => Some(RawAction::Release(RawSwitch::Button(switch))),
                Self::MouseTimed(_, _, _, _) => None,
                Self::MouseMove(_, _) => None,
            }
        }
    }

    impl From<RawEvent> for BasicEvent {
        fn from(event: RawEvent) -> Self {
            match event {
                RawEvent::KeyboardDown(a, b) => Self::KeyboardDown(a, b),
                RawEvent::KeyboardUp(a, b) => Self::KeyboardUp(a, b),
                RawEvent::MouseDown(a, b, c) => Self::MouseDown(a, b, c),
                RawEvent::MouseUp(a, b, c) => Self::MouseUp(a, b, c),
                RawEvent::MouseMove(a, b) => Self::MouseMove(a, b),
            }
        }
    }

    impl From<(RawSwitchEvent, TimedCombinedEventData)> for BasicEvent {
        fn from((event, timed_event): (RawSwitchEvent, TimedCombinedEventData)) -> Self {
            match (event, timed_event) {
                (RawSwitchEvent::KeyboardDown(a, b), t) => Self::KeyboardTimed(a, t, b),
                (RawSwitchEvent::KeyboardUp(a, b), t) => Self::KeyboardTimed(a, t, b),
                (RawSwitchEvent::MouseDown(a, b, c), t) => Self::MouseTimed(a, t, b, c),
                (RawSwitchEvent::MouseUp(a, b, c), t) => Self::MouseTimed(a, t, b, c),
            }
        }
    }

    #[derive(Clone, Debug)]
    struct State {
        timed_state: TimedState<RawSwitch>,
        modified_state: Modifiers<RawSwitch>,
        timeouts: BTreeMap<TimestampMs, (RawSwitchEvent, TimedHandleRequest)>,
        last_timestamp: TimestampMs,
    }

    impl State {
        fn with_event(
            self,
            event: RawEvent,
        ) -> (Self, Vec<EventWithModifiers<BasicEvent, RawSwitch>>) {
            //println!();
            //println!("{:?}", event);

            let mut timed_state = self.timed_state;
            let mut timeouts = self.timeouts;
            let timestamp = event.timestamp();
            assert!(self.last_timestamp < timestamp);
            let last_timestamp = timestamp;

            let mut events: Vec<BasicEvent> = Vec::new();

            while let Some(entry) = timeouts.first_entry() {
                if *entry.key() > timestamp {
                    break;
                }
                let (_, (event, request)) = entry.remove_entry();
                let (new_state, result) =
                    timed_state.with_delayed_event(*event.action().switch(), request);
                if let Some(result) = result {
                    let timed_event = result.unwrap();
                    events.push((event.into(), timed_event.into()).into());
                }
                timed_state = new_state;
            }

            events.push(event.into());
            let (new_state, request) = match RawSwitchEvent::try_from(event) {
                Ok(event) => match event.action() {
                    RawAction::Press(switch) => {
                        let (state, request) = timed_state.with_press_event(switch);
                        let request = request.unwrap();
                        (state, Some((event, request.into(), 1000)))
                    }
                    RawAction::Release(switch) => {
                        let (state, result) = timed_state.with_release_event(switch);
                        let result = result.unwrap();
                        match result {
                            Some((timed_event, request)) => {
                                events.push((event, timed_event.into()).into());
                                (state, Some((event, request.into(), 300)))
                            }
                            None => (state, None),
                        }
                    }
                },
                Err(()) => (timed_state, None),
            };

            timed_state = new_state;
            if let Some((event, request, timeout)) = request {
                timeouts.insert(timestamp + timeout, (event, request));
            }
            let _ = request;

            let (events, modified_state) = events.into_iter().fold(
                (Vec::new(), self.modified_state),
                |(mut events, state), event| {
                    let state = match event.action() {
                        Some(RawAction::Press(switch)) => state.with_press_event(switch),
                        Some(RawAction::Release(switch)) => state.with_release_event(switch),
                        None => state,
                    };
                    let event = EventWithModifiers::new(event, state.clone());
                    events.push(event);
                    (events, state)
                },
            );

            //println!("{:?}", events);

            (
                Self {
                    timed_state,
                    modified_state,
                    timeouts,
                    last_timestamp,
                },
                events,
            )
        }
    }

    let mut state = State {
        timed_state: TimedState::new(),
        modified_state: Modifiers::new(),
        timeouts: BTreeMap::new(),
        last_timestamp: 0,
    };

    let mut time = {
        let mut iter = (10000..).step_by(200);
        move || iter.next().unwrap()
    };
    let raw_events = vec![
        //
        RawEvent::MouseDown("LeftMouseButton", (0, 0), time()),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), time()),
        //
        RawEvent::KeyboardDown("LeftCtrl", time()),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), time()),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), time()),
        RawEvent::KeyboardUp("LeftCtrl", time()),
        //
        RawEvent::KeyboardDown("LeftShift", time()),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), time()),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), time()),
        RawEvent::KeyboardUp("LeftShift", time()),
        //
        RawEvent::KeyboardDown("LeftAlt", time()),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), time()),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), time()),
        RawEvent::KeyboardUp("LeftAlt", time()),
        //
        RawEvent::KeyboardDown("LeftCtrl", time()),
        RawEvent::KeyboardDown("LeftShift", time()),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), time()),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), time()),
        RawEvent::KeyboardUp("LeftCtrl", time()),
        RawEvent::KeyboardUp("LeftShift", time()),
        //
        RawEvent::KeyboardDown("LeftShift", time()),
        RawEvent::KeyboardDown("LeftAlt", time()),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), time()),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), time()),
        RawEvent::KeyboardUp("LeftShift", time()),
        RawEvent::KeyboardUp("LeftAlt", time()),
        //
        RawEvent::KeyboardDown("LeftAlt", time()),
        RawEvent::KeyboardDown("LeftCtrl", time()),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), time()),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), time()),
        RawEvent::KeyboardUp("LeftAlt", time()),
        RawEvent::KeyboardUp("LeftCtrl", time()),
        //
        RawEvent::KeyboardDown("LeftAlt", time()),
        RawEvent::KeyboardDown("LeftCtrl", time()),
        RawEvent::KeyboardDown("LeftShift", time()),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), time()),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), time()),
        RawEvent::KeyboardUp("LeftAlt", time()),
        RawEvent::KeyboardUp("LeftCtrl", time()),
        RawEvent::KeyboardUp("LeftShift", time()),
        //
        /*RawEvent::KeyboardDown("LeftCtrl", 10000),
        RawEvent::KeyboardUp("LeftCtrl", 10500),
        RawEvent::KeyboardDown("LeftCtrl", 11000),
        RawEvent::KeyboardUp("LeftCtrl", 13000),
        RawEvent::KeyboardDown("LeftCtrl", 15000),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), 15100),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), 15200),
        RawEvent::MouseDown("LeftMouseButton", (0, 0), 15300),
        RawEvent::MouseUp("LeftMouseButton", (0, 0), 15400),
        RawEvent::KeyboardUp("LeftCtrl", 18000),
        RawEvent::MouseMove((0, 1000), 20000),*/
    ];

    let mut imm_events = vec![];
    for raw_event in raw_events {
        let (new_state, new_events) = state.with_event(raw_event);
        state = new_state;
        imm_events.extend(new_events);
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum PointerEvent {
        MouseDown(&'static str, Coords, TimestampMs),
        MouseUp(&'static str, Coords, TimestampMs),
        MouseTimed(&'static str, TimedCombinedEventData, Coords, TimestampMs),
        MouseMove(Coords, TimestampMs),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum BasicEventBinding {
        KeyboardDown(&'static str),
        KeyboardUp(&'static str),
        KeyboardTimed(&'static str, TimedCombinedEventData),
        MouseDown(&'static str),
        MouseUp(&'static str),
        MouseTimed(&'static str, TimedCombinedEventData),
        MouseMove,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum PointerEventBinding {
        MouseDown(&'static str),
        MouseUp(&'static str),
        MouseTimed(&'static str, TimedCombinedEventData),
        MouseMove,
    }

    impl PointerEvent {
        fn coords(self) -> Coords {
            match self {
                Self::MouseDown(_, coords, _) => coords,
                Self::MouseUp(_, coords, _) => coords,
                Self::MouseTimed(_, _, coords, _) => coords,
                Self::MouseMove(coords, _) => coords,
            }
        }
    }

    impl TryFrom<BasicEvent> for PointerEvent {
        type Error = ();
        fn try_from(other: BasicEvent) -> Result<Self, Self::Error> {
            match other {
                BasicEvent::MouseDown(a, b, c) => Ok(Self::MouseDown(a, b, c)),
                BasicEvent::MouseUp(a, b, c) => Ok(Self::MouseUp(a, b, c)),
                BasicEvent::MouseTimed(a, b, c, d) => Ok(Self::MouseTimed(a, b, c, d)),
                BasicEvent::MouseMove(a, b) => Ok(Self::MouseMove(a, b)),
                BasicEvent::KeyboardDown(..)
                | BasicEvent::KeyboardUp(..)
                | BasicEvent::KeyboardTimed(..) => Err(()),
            }
        }
    }

    impl From<BasicEvent> for BasicEventBinding {
        fn from(event: BasicEvent) -> Self {
            match event {
                BasicEvent::KeyboardDown(switch, _) => Self::KeyboardDown(switch),
                BasicEvent::KeyboardUp(switch, _) => Self::KeyboardUp(switch),
                BasicEvent::KeyboardTimed(switch, timed, _) => Self::KeyboardTimed(switch, timed),
                BasicEvent::MouseDown(switch, _, _) => Self::MouseDown(switch),
                BasicEvent::MouseUp(switch, _, _) => Self::MouseUp(switch),
                BasicEvent::MouseTimed(switch, timed, _, _) => Self::MouseTimed(switch, timed),
                BasicEvent::MouseMove(_, _) => Self::MouseMove,
            }
        }
    }

    impl From<PointerEvent> for PointerEventBinding {
        fn from(event: PointerEvent) -> Self {
            match event {
                PointerEvent::MouseDown(switch, _, _) => Self::MouseDown(switch),
                PointerEvent::MouseUp(switch, _, _) => Self::MouseUp(switch),
                PointerEvent::MouseTimed(switch, timed, _, _) => Self::MouseTimed(switch, timed),
                PointerEvent::MouseMove(_, _) => Self::MouseMove,
            }
        }
    }

    type NodeId = u64;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum AppEvent {
        CreateNode(NodeId),
        RemoveNode(NodeId),
        SelectNode(NodeId),
        AddNodeToSelection(NodeId),
        StartRectangleSelection(Coords),
        MoveRectangleSelection(Coords),
        EndRectangleSelection(Coords),
        RemoveSelectedNodes,
        DeselectNodes,
        SelectAllNodes,
        DummyEvent1,
        DummyEvent2,
        DummyEvent3,
        DummyEvent4,
        DummyEvent5,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum PointerAppEventKind {
        CreateNode,
        RemoveNode,
        SelectNode,
        AddNodeToSelection,
        StartRectangleSelection,
        MoveRectangleSelection,
        EndRectangleSelection,
        DummyEvent1,
        DummyEvent2,
        DummyEvent3,
        DummyEvent4,
        DummyEvent5,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum BasicAppEventKind {
        RemoveSelectedNodes,
        DeselectNodes,
        SelectAllNodes,
    }

    let basic_event_mappings: HashMap<
        BasicEventBinding,
        Vec<(BTreeSet<RawSwitch>, BasicAppEventKind)>,
    > = [
        /*(
            BasicEventBinding::KeyboardTimed(
                "Delete",
                TimedCombinedEvent::new(TimedCombinedEventKind::Click, 1),
            ),
            vec![(BTreeSet::new(), BasicAppEventKind::RemoveSelectedNodes)],
        ),
        (
            BasicEventBinding::KeyboardTimed(
                "A",
                TimedCombinedEvent::new(TimedCombinedEventKind::Click, 1),
            ),
            vec![(
                [RawSwitch::Key("LeftCtrl")].into_iter().collect(),
                BasicAppEventKind::SelectAllNodes,
            )],
        ),*/
    ]
    .into_iter()
    .collect();

    let lmb_click = PointerEventBinding::MouseTimed(
        "LeftMouseButton",
        TimedCombinedEventData::new(TimedCombinedEventKind::Click, 1),
    );

    let pointer_event_mappings: HashMap<
        PointerEventBinding,
        Vec<(BTreeSet<RawSwitch>, PointerAppEventKind)>,
    > = [
        /*(
            lmb_click,
            vec![(BTreeSet::new(), PointerAppEventKind::SelectNode)],
        ),
        (
            PointerEventBinding::MouseTimed(
                "LeftMouseButton",
                TimedCombinedEvent::new(TimedCombinedEventKind::Click, 2),
            ),
            vec![(BTreeSet::new(), PointerAppEventKind::CreateNode)],
        ),
        (
            lmb_click,
            vec![(
                [RawSwitch::Key("LeftShift")].into_iter().collect(),
                PointerAppEventKind::AddNodeToSelection,
            )],
        ),
        (
            PointerEventBinding::MouseDown("LeftMouseButton"),
            vec![(
                BTreeSet::new(),
                PointerAppEventKind::StartRectangleSelection,
            )],
        ),
        (
            PointerEventBinding::MouseUp("LeftMouseButton"),
            vec![(BTreeSet::new(), PointerAppEventKind::EndRectangleSelection)],
        ),*/
        (
            lmb_click,
            vec![
                (
                    [RawSwitch::Key("LeftShift"), RawSwitch::Key("LeftAlt")]
                        .into_iter()
                        .collect(),
                    PointerAppEventKind::DummyEvent1,
                ),
                (
                    [RawSwitch::Key("LeftShift"), RawSwitch::Key("LeftCtrl")]
                        .into_iter()
                        .collect(),
                    PointerAppEventKind::DummyEvent2,
                ),
                (
                    [RawSwitch::Key("LeftCtrl"), RawSwitch::Key("LeftAlt")]
                        .into_iter()
                        .collect(),
                    PointerAppEventKind::DummyEvent3,
                ),
                (
                    [RawSwitch::Key("LeftCtrl")].into_iter().collect(),
                    PointerAppEventKind::DummyEvent3,
                ),
                (
                    [RawSwitch::Key("LeftAlt")].into_iter().collect(),
                    PointerAppEventKind::DummyEvent4,
                ),
                (
                    [RawSwitch::Key("LeftAlt")].into_iter().collect(),
                    PointerAppEventKind::DummyEvent5,
                ),
            ],
        ),
    ]
    .into_iter()
    .collect();

    for imm_event in imm_events {
        println!("{:?}", imm_event);

        let mut app_events = Vec::new();
        let modifiers = imm_event.modifiers;

        if let Some(event) = PointerEvent::try_from(imm_event.event).ok() {
            const DEFAULT_MAPPING: &Vec<(BTreeSet<RawSwitch>, PointerAppEventKind)> = &Vec::new();

            let binding: PointerEventBinding = event.into();
            let coords = event.coords();
            let mappings = pointer_event_mappings
                .get(&binding)
                .unwrap_or(DEFAULT_MAPPING);
            for (event_modifiers, app_event_kind) in mappings {
                if !event_modifiers.is_subset(&modifiers.switches()) {
                    continue;
                }
                let app_event = match app_event_kind {
                    PointerAppEventKind::CreateNode => AppEvent::CreateNode(coords.0 / 100),
                    PointerAppEventKind::RemoveNode => AppEvent::RemoveNode(coords.0 / 100),
                    PointerAppEventKind::SelectNode => AppEvent::SelectNode(coords.0 / 100),
                    PointerAppEventKind::AddNodeToSelection => {
                        AppEvent::AddNodeToSelection(coords.0 / 100)
                    }
                    PointerAppEventKind::StartRectangleSelection => {
                        AppEvent::StartRectangleSelection(coords)
                    }
                    PointerAppEventKind::MoveRectangleSelection => {
                        AppEvent::MoveRectangleSelection(coords)
                    }
                    PointerAppEventKind::EndRectangleSelection => {
                        AppEvent::EndRectangleSelection(coords)
                    }
                    PointerAppEventKind::DummyEvent1 => AppEvent::DummyEvent1,
                    PointerAppEventKind::DummyEvent2 => AppEvent::DummyEvent2,
                    PointerAppEventKind::DummyEvent3 => AppEvent::DummyEvent3,
                    PointerAppEventKind::DummyEvent4 => AppEvent::DummyEvent4,
                    PointerAppEventKind::DummyEvent5 => AppEvent::DummyEvent5,
                };
                app_events.push((app_event, event_modifiers));
            }
        }

        {
            const DEFAULT_MAPPING: &Vec<(BTreeSet<RawSwitch>, BasicAppEventKind)> = &Vec::new();

            let event = imm_event.event;
            let binding: BasicEventBinding = event.into();
            let mappings = basic_event_mappings
                .get(&binding)
                .unwrap_or(DEFAULT_MAPPING);
            for (event_modifiers, app_event_kind) in mappings {
                if !event_modifiers.is_subset(&modifiers.switches()) {
                    continue;
                }
                let app_event = match app_event_kind {
                    BasicAppEventKind::RemoveSelectedNodes => AppEvent::RemoveSelectedNodes,
                    BasicAppEventKind::DeselectNodes => AppEvent::DeselectNodes,
                    BasicAppEventKind::SelectAllNodes => AppEvent::SelectAllNodes,
                };
                app_events.push((app_event, event_modifiers));
            }
        }

        let app_events_idxs: Vec<_> = app_events
            .iter()
            .enumerate()
            .filter(|(j, event)| {
                app_events
                    .iter()
                    .all(|other| event.1 == other.1 || !other.1.is_superset(event.1))
            })
            .map(|(j, _)| j)
            .collect();

        let mut modifiers_iter = app_events_idxs.iter().map(|&j| app_events[j].1);
        let is_same_modifiers = modifiers_iter
            .next()
            .map_or(false, |first| modifiers_iter.all(|other| other == first));

        let app_events_filtered: Vec<_> = if is_same_modifiers {
            app_events_idxs.into_iter().map(|j| app_events[j]).collect()
        } else {
            vec![]
        };

        println!("{:?}", app_events);
        println!("{:?}", app_events_filtered);
        println!();

        // app_events
        // find longest app_events
        //   A+B+C, A+B+D => remain both
        //   A+B+C, A+B+C+D => remain second
        // if different modifiers then drop
        //   A+B+C, A+B+D => drop
        //   A+B+C, A+B+C => use them all

        // TouchStart(touch_id)
        // #1 -> 100, 100
        // 200, 200 -> #2
        // #1 -> 100, 100 (#1 -- @1)
        // #2 -> 200, 200 (#2 -- @2)
        // 100, 100 -> #1

        // MouseUp 100, 100     MouseTrigger(1)
        // MouseDown 200, 200   MouseTrigger(2)
        //  drop_state()

        // TouchStart -> #1 100, 100
        // TouchMove -> #1(active) 100, 110
        // TouchStart -> #2 100, 100
        // TouchMove -> #2 (@1)

        // TouchStart   @1 100, 100 #1
        // TouchStart   @2 200, 200 #2
        // TouchEnd     @1 100, 100 #1
        // TouchEnd     @2 200, 200 #2
        // TouchStart   @1 200, 200 #2
    }
    panic!();

    /*
        basic-event = (raw-event | raw-switch-event + timed-combined-event)
        complex-event = (basic-event + BTreeSet)

        try complex-event as (pointer-event-kind + coords)
            filter possible event builders
                get event kinds in mapping by event + BTreeSet
            check available events
                CreateNode(node_id) + BTreeSet

        try complex-event as (basic-event-kind)
            filter possible event builders
                get event kinds in mapping by event + BTreeSet
            check available events
                DeselectNodes(...) + BTreeSet

        check overrides
    */

    //imm_events

    //panic!();

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

        immediate-event | modified-state > event-with-BTreeSet
            switch? + kind? + num? + binding + timestamp + BTreeSet

        event-with-BTreeSet | app > filtered-event-with-BTreeSet
            switch? + kind? + num? + binding + timestamp + BTreeSet + app-event-data

        filtered-event-with-BTreeSet | helpers > filtered2-event-with-BTreeSet
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
        TimedEvent(TimedCombinedEventKind),
    }

        struct MappingWithCoords {
            binding: BindingWithCoords,
            BTreeSet: HashSet<EventSwitch>,
            event: AppEventKindWithCoords,
        }
    */
}
