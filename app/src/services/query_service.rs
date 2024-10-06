// necesary crates
use sails_rs::prelude::*;

use crate::services::ping_service::PingService;
use keyring_service::services::keyring_query_service::KeyringQueryService;

// Service used for all queries
pub struct QueryService {
    // To extend the wallet less service (and others services), yo need to create
    // an atribute to "store" the service
    keyring_query_service: KeyringQueryService
}

// Service that extends KeyringQueryService
#[service(extends = KeyringQueryService)]
impl QueryService {
    // Service constructor
    pub fn new() -> Self {
        Self {
            keyring_query_service: KeyringQueryService::new()
        }
    }

    pub fn last_caller(&self) -> ActorId {
        PingService::state_ref().last_who_call
    }
}

// You need to implement "AsRef" on the services that will extends another
// service, in this case "KeyringQueryService"
impl AsRef<KeyringQueryService> for QueryService {
    fn as_ref(&self) -> &KeyringQueryService {
        &self.keyring_query_service
    }
}