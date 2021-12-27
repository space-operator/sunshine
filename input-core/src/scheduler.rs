use std::collections::BTreeMap;

use crate::{ClickExactHandleRequest, LongPressHandleRequest};

#[derive(Clone, Debug)]
pub struct SchedulerState<Ti, Da, Rq> {
    requests: BTreeMap<Ti, Vec<(Da, Rq)>>,
}

pub type LongPressSchedulerState<Ti, Da> = SchedulerState<Ti, Da, LongPressHandleRequest>;
pub type ClickExactSchedulerState<Ti, Da> = SchedulerState<Ti, Da, ClickExactHandleRequest>;

impl<Ti, Da, Rq> SchedulerState<Ti, Da, Rq> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_requests(self) -> BTreeMap<Ti, Vec<(Da, Rq)>> {
        self.requests
    }

    pub fn next_scheduled(&self) -> Option<&Ti> {
        self.requests.keys().next()
    }

    pub fn schedule(self, time: Ti, data: Da, request: Rq) -> Self
    where
        Ti: Ord,
    {
        let mut requests = self.requests;
        let requests_by_time = requests.entry(time).or_default();
        requests_by_time.push((data, request));
        Self::from(requests)
    }

    pub fn take_scheduled<TiRef>(self, time: &Ti) -> (Self, Vec<(Ti, Vec<(Da, Rq)>)>)
    where
        Ti: Ord,
    {
        let mut scheduled = self.requests;
        let mut requests = scheduled.split_off(time);
        if let Some((key, value)) = requests.remove_entry(time) {
            let prev = scheduled.insert(key, value);
            assert!(prev.is_none());
        }
        (Self::from(requests), scheduled.into_iter().collect())
    }
}

impl<Ti, Da, Rq> From<BTreeMap<Ti, Vec<(Da, Rq)>>> for SchedulerState<Ti, Da, Rq> {
    fn from(requests: BTreeMap<Ti, Vec<(Da, Rq)>>) -> Self {
        Self { requests }
    }
}

impl<Ti, Da, Rq> Default for SchedulerState<Ti, Da, Rq> {
    fn default() -> Self {
        Self {
            requests: BTreeMap::default(),
        }
    }
}
