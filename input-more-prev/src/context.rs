#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Context<St, Ev> {
    pub state: St,
    pub event: Ev,
}

impl<St, Ev> Context<St, Ev> {
    pub fn new(state: St, event: Ev) -> Self {
        Self { state, event }
    }

    pub fn split(self) -> (St, Ev) {
        (self.state, self.event)
    }

    pub fn map_state<F, St2>(self, func: F) -> Context<St2, Ev>
    where
        F: FnOnce(St) -> St2,
    {
        Context::new(func(self.state), self.event)
    }

    pub fn map_event<F, Ev2>(self, func: F) -> Context<St, Ev2>
    where
        F: FnOnce(Ev) -> Ev2,
    {
        Context::new(self.state, func(self.event))
    }

    pub fn map<F, St2, Ev2>(self, func: F) -> Context<St2, Ev2>
    where
        F: FnOnce(St, Ev) -> (St2, Ev2),
    {
        let (state, event) = func(self.state, self.event);
        Context::new(state, event)
    }
}
