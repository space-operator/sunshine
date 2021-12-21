#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Context<St, Ev> {
    pub state: St,
    pub event: Ev,
}

impl<St, Ev> Context<St, Ev> {
    pub fn new(state: St, event: Ev) -> Self {
        Self { state, event }
    }

    pub fn map_event<F, Ev2>(self, func: F) -> Context<St, Ev2>
    where
        F: FnOnce(Ev) -> Ev2,
    {
        Context::new(self.state, func(self.event))
    }
}
