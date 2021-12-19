use std::collections::BTreeMap;

use crate::{LongPressHandleRequest, MultiClickHandleRequest};

#[derive(Clone, Debug)]
pub struct TimedSchedulerState<Ti, Re> {
    requests: BTreeMap<Ti, Vec<Re>>,
}

pub type LongPressSchedulerState<Ti> = TimedSchedulerState<Ti, LongPressHandleRequest>;
pub type MultiClickSchedulerState<Ti> = TimedSchedulerState<Ti, MultiClickHandleRequest>;

impl<Ti, Re> TimedSchedulerState<Ti, Re> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_requests(self) -> BTreeMap<Ti, Vec<Re>> {
        self.requests
    }

    pub fn schedule(self, time: Ti, request: Re) -> Self
    where
        Ti: Ord,
    {
        let mut requests = self.requests;
        let requests_by_time = requests.entry(time).or_default();
        requests_by_time.push(request);
        Self::from(requests)
    }

    pub fn scheduled(self, time: &Ti) -> (Self, Vec<Re>)
    where
        Ti: Ord,
    {
        let mut scheduled = self.requests;
        let requests = scheduled.split_off(time);
        (
            Self::from(requests),
            scheduled.into_values().flatten().collect(),
        )
    }
}

impl<Ti, Re> From<BTreeMap<Ti, Vec<Re>>> for TimedSchedulerState<Ti, Re> {
    fn from(requests: BTreeMap<Ti, Vec<Re>>) -> Self {
        Self { requests }
    }
}

impl<Ti, Re> Default for TimedSchedulerState<Ti, Re> {
    fn default() -> Self {
        Self {
            requests: BTreeMap::default(),
        }
    }
}
