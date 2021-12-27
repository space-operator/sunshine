use core::hash::Hash;
use core::marker::PhantomData;

use input_core::{EventWithModifiers, Modifiers};

use crate::{Action, ActionOrTrigger, Event, SwitchEvent, ToActionOrTrigger};

#[derive(Clone, Debug, Default)]
pub struct State<De, Sw, Co> {
    _device: PhantomData<De>,
    _switch: PhantomData<Sw>,
    _coords: PhantomData<Co>,
    modifiers: Modifiers<Sw>,
}

/*
    low-level:
        press-switch
        release-switch
        trigger

    app-event
*/

/*
    optimize processing using filtering:
        filter switch-used-in-event-or-modifiers | trigger-used-in-event
        modifiers-processing
        filter switch-used-in-event | trigger-used-in-event
        if pointer-trigger-used-in-events then pointer-processing
        if timed-trigger-used-in-events then timed-processing
*/

/*
trait TriggerMapping {
    type Switch;
    type Trigger;

    pub fn filter_by_trigger(trigger: Self::Trigger) -> bool;
    pub fn obtain_by_modifiers(trigger: Self::Trigger, modifiers: Modifiers<Self::Switch>)
}
*/
impl<De, Sw, Co> State<De, Sw, Co> {
    /*
        bindings
            switch?, trigger?, modifiers?

        event
            device_id?, switch?, trigger?, coords?, is_drag_fn?
        mapping
            filter by switch? and trigger?

        with_pressed_event
            switch
        with_pressed_pointer_event
            device_id, switch, coords, is_drag_fn
        with_release_event
            switch
        with_release_pointer_event
            device_id, switch, coords, is_drag_fn
        with_pointer_move_event
            device_id, coords, is_drag_fn

        mapping


    */

    fn with_action_event<Ev>(self, event: Action<Ev>) -> (Self, EventWithModifiers<Action<Ev>, Sw>)
    where
        Ev: SwitchEvent<Switch = Sw>,
        Sw: Clone + Hash + Ord,
    {
        let modifiers = self.modifiers;
        let modifiers = match &event {
            Action::Press(event) => modifiers.with_press_event(event.switch()),
            Action::Release(event) => modifiers.with_release_event(event.switch()),
        };
        let event = EventWithModifiers::new(event, modifiers.clone());
        let state = State {
            modifiers,
            _device: PhantomData,
            _switch: PhantomData,
            _coords: PhantomData,
        };
        (state, event)
    }
    /*
    fn with_event<Ev>(self, event: Ev) -> (Self, EventWithModifiers<Ev, Sw>)
    where
        Ev: Event<Switch = Sw> + for<'a> ToActionOrTrigger<'a>,
        Sw: Clone + Hash + Ord,
    {
        let modified_state = self.modified_state;
        let event = match event.to_action_or_trigger() {
            v @ ActionOrTrigger::Action(Action::Press(action)) => {
                let switch = action.switch();
                drop(v);
                modified_state.with_press_event(event, switch)
            }
            v @ ActionOrTrigger::Action(Action::Release(action)) => {
                let switch = action.switch();
                drop(v);
                modified_state.with_release_event(event, switch)
            }
            v @ ActionOrTrigger::Trigger(_) => {
                drop(v);
                modified_state.with_trigger_event(event)
            }
        };
        let modified_state = event.to_state();
        let state = State {
            modified_state,
            _device: PhantomData,
            _switch: PhantomData,
            _coords: PhantomData,
        };
        (state, event)
    }*/
}
