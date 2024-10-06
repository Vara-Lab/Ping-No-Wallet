use sails_rs::prelude::*;

pub struct PingState {
    pub last_who_call: ActorId
}

impl PingState {
    pub fn new(caller: ActorId) -> Self {
        Self {
            last_who_call: caller
        }
    }
}
