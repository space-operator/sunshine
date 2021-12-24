use std::collections::HashSet;

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
        KeyboardMultiClick(KeyboardMultiClickBinding),
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
        BindingData<KeyboardSwitch, TimedReleaseEventData, BasicAppEventBuilder>;
    pub type KeyboardLongPressBinding =
        BindingData<MouseSwitch, TimedLongPressEventData, BasicAppEventBuilder>;
    pub type KeyboardMultiClickBinding =
        BindingData<MouseSwitch, TimedMultiClickEventData, BasicAppEventBuilder>;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct BindingData<Sw, Td, Ev> {
        switch: Sw,
        timed_data: Td,
        event_builder: Ev,
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
        keyboard_multi_click_scheduler: OurSchedulerState<KeyboardSwitch, MultiClickHandleRequest>,
        mouse_long_press_scheduler: OurSchedulerState<MouseSwitch, LongPressHandleRequest>,
        mouse_multi_click_scheduler: OurSchedulerState<MouseSwitch, MultiClickHandleRequest>,
        bindings: Bindings,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Bindings {
        keyboard_press: HashSet<KeyboardPressBinding>,
        keyboard_release: HashSet<KeyboardPressBinding>,
        keyboard_long_press: HashSet<KeyboardPressBinding>,
        keyboard_multi_click: HashSet<KeyboardPressBinding>,
    }

    #[derive(Clone, Debug, Default)]
    pub struct EventState<Sw, Rq> {
        modifiers: Modifiers<Switch>,
        timed_state: TimedState<Sw>,
        scheduler: OurSchedulerState<Sw, Rq>,
    }

    impl State {
        pub fn with_keyboard_press_state<F, T>(self, func: F) -> (Self, T)
        where
            F: FnOnce(
                EventState<KeyboardSwitch, LongPressHandleRequest>,
            ) -> (EventState<KeyboardSwitch, LongPressHandleRequest>, T),
        {
            let state = EventState {
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
                keyboard_multi_click_scheduler: self.keyboard_multi_click_scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_multi_click_scheduler: self.mouse_multi_click_scheduler,
                bindings: self.bindings,
            };
            (state, timestamp)
        }

        pub fn with_keyboard_release_state<F, T>(self, func: F) -> (Self, T)
        where
            F: FnOnce(
                EventState<KeyboardSwitch, MultiClickHandleRequest>,
            ) -> (EventState<KeyboardSwitch, MultiClickHandleRequest>, T),
        {
            let state = EventState {
                modifiers: self.modifiers,
                timed_state: self.keyboard_timed_state,
                scheduler: self.keyboard_multi_click_scheduler,
            };
            let (state, timestamp) = func(state);
            let state = Self {
                modifiers: state.modifiers,
                keyboard_timed_state: state.timed_state,
                mouse_timed_state: self.mouse_timed_state,
                keyboard_long_press_scheduler: self.keyboard_long_press_scheduler,
                keyboard_multi_click_scheduler: state.scheduler,
                mouse_long_press_scheduler: self.mouse_long_press_scheduler,
                mouse_multi_click_scheduler: self.mouse_multi_click_scheduler,
                bindings: self.bindings,
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

    impl<Sw, Rq> EventState<Sw, Rq> {
        pub fn new(
            modifiers: Modifiers<Switch>,
            timed_state: TimedState<Sw>,
            scheduler: OurSchedulerState<Sw, Rq>,
        ) -> Self {
            Self {
                modifiers,
                timed_state,
                scheduler,
            }
        }

        pub fn split(self) -> (Modifiers<Switch>, TimedState<Sw>, OurSchedulerState<Sw, Rq>) {
            (self.modifiers, self.timed_state, self.scheduler)
        }
    }

    impl EventState<KeyboardSwitch, LongPressHandleRequest> {
        pub fn with_event(self, event: KeyboardSwitchEvent) -> (Self, Option<TimestampMs>) {
            dbg!();
            let (modifiers, timed_state, scheduler) = self.split();
            let (modifiers, result) = modifiers.with_press_event(Switch::Keyboard(event.switch));
            result.unwrap();
            let event = event.with_modifiers(modifiers.clone());
            let (timed_state, result) = timed_state.with_press_event(event.switch);
            let request = result.unwrap();
            let scheduler = scheduler.schedule(event.time, event.clone(), request);

            println!("Ev: {:?}", event);

            let next_scheduled = scheduler.next_scheduled().copied();
            (Self::new(modifiers, timed_state, scheduler), next_scheduled)
        }

        pub fn with_timeout(self, time: TimestampMs) -> Self {
            dbg!();
            const KEYBOARD_LONG_PRESS_DURATION: DurationMs = 1000;

            let (modifiers, timed_state, scheduler) = self.split();
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

                    println!("Ev: {:?}", event);
                }
            }
            Self::new(modifiers, timed_state, scheduler)
        }
    }

    impl EventState<KeyboardSwitch, MultiClickHandleRequest> {
        pub fn with_event(self, event: KeyboardSwitchEvent) -> (Self, Option<TimestampMs>) {
            dbg!();
            let (modifiers, timed_state, scheduler) = self.split();
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

            println!("Ev: {:?}", event);

            let next_scheduled = scheduler.next_scheduled().copied();
            (Self::new(modifiers, timed_state, scheduler), next_scheduled)
        }

        pub fn with_timeout(self, time: TimestampMs) -> Self {
            dbg!();
            const KEYBOARD_MULTICLICK_DURATION: DurationMs = 300;

            let (modifiers, timed_state, scheduler) = self.split();
            let mut timed_state = timed_state;
            let (scheduler, (_, requests)) =
                scheduler.take_scheduled(time - KEYBOARD_MULTICLICK_DURATION);
            for (time, requests) in requests {
                for (event, request) in requests {
                    let (new_timed_state, result) =
                        timed_state.with_multi_click_event(event.switch, request);
                    timed_state = new_timed_state;

                    let timed_data = result.unwrap();
                    let event = event.with_timed_data(timed_data);

                    println!("Ev: {:?}", event);
                }
            }
            Self::new(modifiers, timed_state, scheduler)
        }
    }

    let state = State::default();
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
        3000,
        KeyboardSwitch("LeftCtrl"),
        (),
    )));
    println!("St: {:?}", state.keyboard_timed_state);
    println!();

    //

    panic!();
}
