use core::mem::take;
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

    pub fn schedule(&mut self, time: Ti, data: Da, request: Rq)
    where
        Ti: Ord,
    {
        self.requests.entry(time).or_default().push((data, request));
    }

    pub fn take_scheduled(&mut self, time: &Ti) -> impl Iterator<Item = (Ti, Vec<(Da, Rq)>)>
    where
        Ti: Ord,
    {
        let mut scheduled = take(&mut self.requests);
        self.requests = scheduled.split_off(time);
        if let Some((key, value)) = self.requests.remove_entry(time) {
            let prev = scheduled.insert(key, value);
            assert!(prev.is_none());
        }
        scheduled.into_iter()
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
