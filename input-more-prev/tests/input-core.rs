use std::borrow::Borrow;

//#[cfg(feature = "qwdsadfrgsfg")]
#[test]
fn test_chain() {
    use core::fmt::Debug;

    use input_core::*;

    type DurationMs = u64;
    type TimestampMs = u64;
    type Coords = (u64, u64);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Switch {
        Keyboard(KeyboardSwitch),
        Mouse(MouseSwitch),
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct KeyboardSwitch(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MouseSwitch(&'static str);

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum RawEvent {
        KeyboardPress(KeyboardSwitchEvent),
        KeyboardRelease(KeyboardSwitchEvent),
        MousePress(MouseSwitchEvent),
        MouseRelease(MouseSwitchEvent),
        //MouseMove(Coords, TimestampMs),
    }

    impl RawEvent {
        fn time(&self) -> TimestampMs {
            match self {
                Self::KeyboardPress(event) => event.time,
                Self::KeyboardRelease(event) => event.time,
                Self::MousePress(event) => event.time,
                Self::MouseRelease(event) => event.time,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum TimeoutEvent {
        KeyboardPress(TimestampMs),
        KeyboardRelease(TimestampMs),
        MousePress(TimestampMs),
        MouseRelease(TimestampMs),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Binding {
        KeyboardPress(KeyboardPressBinding),
        KeyboardRelease(KeyboardReleaseBinding),
        KeyboardLongPress(KeyboardLongPressBinding),
        KeyboardClickExact(KeyboardClickExactBinding),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum AppEventBuilder {
        Undo(u32),
        CreateNode(u32, Coords),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum BasicAppEventBuilder {
        Undo(u32),
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PointerAppEventBuilder {
        CreateNode(u32),
        Undo(u32),
    }

    pub type KeyboardPressBinding = BindingData<KeyboardSwitch, (), BasicAppEventBuilder>;
    pub type KeyboardReleaseBinding =
        BindingData<KeyboardSwitch, Option<TimedReleaseEventData>, BasicAppEventBuilder>;
    pub type KeyboardLongPressBinding =
        BindingData<MouseSwitch, TimedLongPressEventData, BasicAppEventBuilder>;
    pub type KeyboardClickExactBinding =
        BindingData<MouseSwitch, TimedClickExactEventData, BasicAppEventBuilder>;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct BindingData<Sw, Td, Ev> {
        switch: Sw,
        modifiers: Modifiers<Switch>,
        timed_data: Td,
        event_builder: Ev,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Mapping<Bi>(Vec<Bi>);

    impl<Sw, Td, Ev> Mapping<BindingData<Sw, Td, Ev>> {
        fn to_ref_mapping(&self) -> Mapping<&BindingData<Sw, Td, Ev>> {
            Mapping(self.0.iter().collect())
        }
    }

    impl<Bi> Default for Mapping<Bi> {
        fn default() -> Self {
            Self(Vec::default())
        }
    }

    impl<Bi> Mapping<Bi> {
        fn filter_by_switch<Sw, Td, Ev>(self, switch: &Sw) -> Self
        where
            Sw: PartialEq<Sw>,
            Bi: Borrow<BindingData<Sw, Td, Ev>>,
        {
            Self(
                self.0
                    .into_iter()
                    .filter(|binding| binding.borrow().switch == *switch)
                    .collect(),
            )
        }

        fn filter_by_modifiers<Sw, Td, Ev>(self, modifiers: &Modifiers<Switch>) -> Self
        where
            Bi: Borrow<BindingData<Sw, Td, Ev>>,
        {
            Self(
                self.0
                    .into_iter()
                    .filter(|binding| binding.borrow().modifiers == *modifiers)
                    .collect(),
            )
        }

        fn filter_by_timed_data<Sw, Td, Ev>(self, timed_data: &Td) -> Self
        where
            Td: PartialEq<Td>,
            Bi: Borrow<BindingData<Sw, Td, Ev>>,
        {
            Self(
                self.0
                    .into_iter()
                    .filter(|binding| binding.borrow().timed_data == *timed_data)
                    .collect(),
            )
        }

        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    pub type KeyboardSwitchEvent = SwitchEvent<TimestampMs, KeyboardSwitch, (), (), ()>;
    pub type MouseSwitchEvent = SwitchEvent<TimestampMs, MouseSwitch, Coords, (), ()>;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    struct SwitchEvent<Ti, Sw, Co, Mo, Td> {
        time: Ti,
        switch: Sw,
        coords: Co,
        modifiers: Mo,
        timed_data: Td,
    }

    impl<Ti, Sw, Co> SwitchEvent<Ti, Sw, Co, (), ()> {
        fn new(time: Ti, switch: Sw, coords: Co) -> Self {
            SwitchEvent {
                time: time,
                switch: switch,
                coords: coords,
                modifiers: (),
                timed_data: (),
            }
        }
    }

    impl<Ti, Sw, Co, Td> SwitchEvent<Ti, Sw, Co, (), Td> {
        fn with_modifiers<Mo>(self, modifiers: Mo) -> SwitchEvent<Ti, Sw, Co, Mo, Td> {
            SwitchEvent {
                time: self.time,
                switch: self.switch,
                coords: self.coords,
                modifiers,
                timed_data: self.timed_data,
            }
        }
    }

    impl<Ti, Sw, Co, Mo> SwitchEvent<Ti, Sw, Co, Mo, ()> {
        fn with_timed_data<Td>(self, timed_data: Td) -> SwitchEvent<Ti, Sw, Co, Mo, Td> {
            SwitchEvent {
                time: self.time,
                switch: self.switch,
                coords: self.coords,
                modifiers: self.modifiers,
                timed_data,
            }
        }
    }

    //

    pub type OurSchedulerState<Sw, Rq> =
        SchedulerState<TimestampMs, SwitchEvent<TimestampMs, Sw, (), Modifiers<Switch>, ()>, Rq>;

    #[derive(Clone, Debug, Default)]
    pub struct State {
        modifiers: Modifiers<Switch>,
        keyboard_timed_state: TimedState<KeyboardSwitch>,
        mouse_timed_state: TimedState<MouseSwitch>,
        keyboard_long_press_scheduler: OurSchedulerState<KeyboardSwitch, LongPressHandleRequest>,
        keyboard_click_exact_scheduler: OurSchedulerState<KeyboardSwitch, ClickExactHandleRequest>,
        mouse_long_press_scheduler: OurSchedulerState<MouseSwitch, LongPressHandleRequest>,
        mouse_click_exact_scheduler: OurSchedulerState<MouseSwitch, ClickExactHandleRequest>,
        mappings: Mappings,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Mappings {
        keyboard_press: Mapping<KeyboardPressBinding>,
        keyboard_release: Mapping<KeyboardReleaseBinding>,
        keyboard_long_press: Mapping<KeyboardLongPressBinding>,
        keyboard_click_exact: Mapping<KeyboardClickExactBinding>,
    }

    #[derive(Clone, Debug, Default)]
    pub struct EventState<Bi, Sw, Rq> {
        mapping: Mapping<Bi>,
        modifiers: Modifiers<Switch>,
        timed_state: TimedState<Sw>,
        scheduler: OurSchedulerState<Sw, Rq>,
    }

    impl State {
        pub fn with_keyboard_press_state<F, T>(self, func: F) -> (Self, T)
        where
            F: FnOnce(
                EventState<KeyboardPressBinding, KeyboardSwitch, LongPressHandleRequest>,
            ) -> (
                EventState<KeyboardPressBinding, KeyboardSwitch, LongPressHandleRequest>,
                T,
            ),
        {
            let state = EventState {
                mapping: self.mappings.keyboard_press,
                modifiers: self.modifiers,
                timed_state: self.keyboard_timed_state,
                scheduler: self.keyboard_long_press_scheduler,
            };
            let (state, timestamp) = func(state);
            let state = Self {
                modifiers: state.modifiers,
                keyboard_timed_state: state.timed_state,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: state.scheduler,
                keyboard_click_exact_scheduler: self.keyboard_click_exact_scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
                mappings: Mappings {
                    keyboard_press: state.mapping,
                    keyboard_release: self.mappings.keyboard_release,
                    keyboard_long_press: self.mappings.keyboard_long_press,
                    keyboard_click_exact: self.mappings.keyboard_click_exact,
                },
            };
            (state, timestamp)
        }

        pub fn with_keyboard_release_state<F, T>(self, func: F) -> (Self, T)
        where
            F: FnOnce(
                EventState<KeyboardReleaseBinding, KeyboardSwitch, ClickExactHandleRequest>,
            ) -> (
                EventState<KeyboardReleaseBinding, KeyboardSwitch, ClickExactHandleRequest>,
                T,
            ),
        {
            let state = EventState {
                mapping: self.mappings.keyboard_release,
                modifiers: self.modifiers,
                timed_state: self.keyboard_timed_state,
                scheduler: self.keyboard_click_exact_scheduler,
            };
            let (state, timestamp) = func(state);
            let state = Self {
                modifiers: state.modifiers,
                keyboard_timed_state: state.timed_state,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                keyboard_click_exact_scheduler: state.scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_click_exact_scheduler: self.mouse_click_exact_scheduler,
                mappings: Mappings {
                    keyboard_press: self.mappings.keyboard_press,
                    keyboard_release: state.mapping,
                    keyboard_long_press: self.mappings.keyboard_long_press,
                    keyboard_click_exact: self.mappings.keyboard_click_exact,
                },
            };
            (state, timestamp)
        }

        pub fn with_event(self, event: RawEvent) -> Self {
            let state = self;
            let (state, ()) =
                state.with_keyboard_press_state(|state| (state.with_timeout(event.time()), ()));
            let (state, ()) =
                state.with_keyboard_release_state(|state| (state.with_timeout(event.time()), ()));

            let (state, next_scheduled) = match event {
                RawEvent::KeyboardPress(event) => {
                    state.with_keyboard_press_state(|state| state.with_event(event))
                }
                RawEvent::KeyboardRelease(event) => {
                    state.with_keyboard_release_state(|state| state.with_event(event))
                }
                RawEvent::MousePress(event) => todo!(),
                RawEvent::MouseRelease(event) => todo!(),
            };
            println!("Ti: {:?}", next_scheduled);
            state
        }

        pub fn with_timeout_event(self, event: TimeoutEvent) -> Self {
            let (state, ()) = match event {
                TimeoutEvent::KeyboardPress(time) => {
                    self.with_keyboard_press_state(|state| (state.with_timeout(time), ()))
                }
                TimeoutEvent::KeyboardRelease(time) => {
                    self.with_keyboard_release_state(|state| (state.with_timeout(time), ()))
                }
                TimeoutEvent::MousePress(time) => todo!(),
                TimeoutEvent::MouseRelease(time) => todo!(),
            };
            state
        }
    }

    impl<Bi, Sw, Rq> EventState<Bi, Sw, Rq> {
        pub fn new(
            mapping: Mapping<Bi>,
            modifiers: Modifiers<Switch>,
            timed_state: TimedState<Sw>,
            scheduler: OurSchedulerState<Sw, Rq>,
        ) -> Self {
            Self {
                mapping,
                modifiers,
                timed_state,
                scheduler,
            }
        }

        pub fn split(
            self,
        ) -> (
            Mapping<Bi>,
            Modifiers<Switch>,
            TimedState<Sw>,
            OurSchedulerState<Sw, Rq>,
        ) {
            (
                self.mapping,
                self.modifiers,
                self.timed_state,
                self.scheduler,
            )
        }
    }

    impl EventState<KeyboardPressBinding, KeyboardSwitch, LongPressHandleRequest> {
        pub fn with_event(self, event: KeyboardSwitchEvent) -> (Self, Option<TimestampMs>) {
            let (mapping, modifiers, timed_state, scheduler) = self.split();
            let filtered_mapping = mapping.to_ref_mapping();

            let (modifiers, result) = modifiers.with_press_event(Switch::Keyboard(event.switch));
            result.unwrap();

            let event = event.with_modifiers(modifiers.clone());
            let (timed_state, result) = timed_state.with_press_event(event.switch);
            let request = result.unwrap();

            let scheduler = scheduler.schedule(event.time, event.clone(), request);
            let next_scheduled = scheduler.next_scheduled().copied();

            //let filtered_mapping = filtered_mapping.filter_by_switch(&event.switch);
            //if filtered_mapping.is_empty() {
            //    return (
            //        Self::new(mapping, modifiers, timed_state, scheduler),
            //        next_scheduled,
            //    );
            //}
            //let filtered_mapping = filtered_mapping.filter_by_modifiers(&modifiers);
            //if filtered_mapping.is_empty() {
            //    return (
            //        Self::new(mapping, modifiers, timed_state, scheduler),
            //        next_scheduled,
            //    );
            //}

            println!("Ev1: {:?}", event);

            (
                Self::new(mapping, modifiers, timed_state, scheduler),
                next_scheduled,
            )
        }

        pub fn with_timeout(self, time: TimestampMs) -> Self {
            // TODO: Add filtering
            const KEYBOARD_LONG_PRESS_DURATION: DurationMs = 1000;

            let (mapping, modifiers, timed_state, scheduler) = self.split();
            let mut timed_state = timed_state;
            let (scheduler, (_, requests)) =
                scheduler.take_scheduled(time - KEYBOARD_LONG_PRESS_DURATION);
            for (time, requests) in requests {
                for (event, request) in requests {
                    let (new_timed_state, result) =
                        timed_state.with_long_press_event(event.switch, request);
                    timed_state = new_timed_state;

                    let timed_data = result.unwrap();
                    let event = event.with_timed_data(timed_data);

                    println!("Ev2: {:?}", event);
                }
            }
            Self::new(mapping, modifiers, timed_state, scheduler)
        }
    }

    impl EventState<KeyboardReleaseBinding, KeyboardSwitch, ClickExactHandleRequest> {
        pub fn with_event(self, event: KeyboardSwitchEvent) -> (Self, Option<TimestampMs>) {
            let (mapping, modifiers, timed_state, scheduler) = self.split();
            let filtered_mapping = mapping.to_ref_mapping();

            let (modifiers, (_, result)) =
                modifiers.with_release_event(Switch::Keyboard(event.switch));
            result.unwrap();

            let event = event.with_modifiers(modifiers.clone());
            let (timed_state, result) = timed_state.with_release_event(event.switch);
            let data = result.unwrap();

            let (scheduler, event) = if let Some((timed_event_data, request)) = data {
                (
                    scheduler.schedule(event.time, event.clone(), request),
                    event.with_timed_data(Some(timed_event_data)),
                )
            } else {
                (scheduler, event.with_timed_data(None))
            };
            let next_scheduled = scheduler.next_scheduled().copied();

            //let filtered_mapping = filtered_mapping.filter_by_switch(&event.switch);
            //if filtered_mapping.is_empty() {
            //    return (Self::new(mapping, modifiers, timed_state, scheduler), None);
            //}
            //let filtered_mapping = filtered_mapping.filter_by_modifiers(&modifiers);
            //if filtered_mapping.is_empty() {
            //    return (Self::new(mapping, modifiers, timed_state, scheduler), None);
            //}
            //let mapping = mapping.filter_by_timed_data(&event.timed_data);
            //if mapping.is_empty() {
            //    return (
            //        Self::new(mapping, modifiers, timed_state, scheduler),
            //        next_scheduled,
            //    );
            //}

            println!("Ev3: {:?}", event);

            (
                Self::new(mapping, modifiers, timed_state, scheduler),
                next_scheduled,
            )
        }

        pub fn with_timeout(self, time: TimestampMs) -> Self {
            // TODO: Add filtering
            const KEYBOARD_CLICK_EXACT_DURATION: DurationMs = 300;

            let (mapping, modifiers, timed_state, scheduler) = self.split();
            let mut timed_state = timed_state;
            let (scheduler, (_, requests)) =
                scheduler.take_scheduled(time - KEYBOARD_CLICK_EXACT_DURATION);
            for (time, requests) in requests {
                for (event, request) in requests {
                    let (new_timed_state, result) =
                        timed_state.with_click_exact_event(event.switch, request);
                    timed_state = new_timed_state;

                    let timed_data = result.unwrap();
                    let event = event.with_timed_data(timed_data);

                    println!("Ev4: {:?}", event);
                }
            }
            Self::new(mapping, modifiers, timed_state, scheduler)
        }
    }

    let mut state = State::default();
    state.mappings.keyboard_release.0 = vec![BindingData {
        switch: KeyboardSwitch("LeftCtrl"),
        modifiers: Modifiers::new(),
        timed_data: Some(TimedReleaseEventData {
            kind: TimedReleaseEventKind::Click,
            num_possible_clicks: 2,
        }),
        event_builder: BasicAppEventBuilder::Undo(10),
    }];

    let state = state.with_event(RawEvent::KeyboardPress(SwitchEvent::new(
        1000,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    let state = state.with_event(RawEvent::KeyboardRelease(SwitchEvent::new(
        1100,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    let state = state.with_event(RawEvent::KeyboardPress(SwitchEvent::new(
        1200,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    let state = state.with_event(RawEvent::KeyboardRelease(SwitchEvent::new(
        1300,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    //

    panic!();
}
