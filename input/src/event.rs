use std::sync::Arc;

//use serde::{Deserialize, Serialize};

use crate::Modifiers;

//#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
//pub struct Event<T> {
//    pub input: T,
//    pub timestamp: TimestampMs,
//}

#[derive(Clone, Debug)]
pub struct EventWithModifiers<T> {
    pub input: T,
    pub modifiers: Arc<Modifiers>,
}
