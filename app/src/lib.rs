#![no_std]

// necesary crates
use sails_rs::{
    prelude::*,
    gstd::msg
};

// import our modules 
pub mod states;
pub mod services;


// Import service to be used for the program
use services::{
    ping_service::PingService,
    query_service::QueryService
};

use keyring_service::services::keyring_service::KeyringService;

// Ping program struct to build the program (there can only be one per contract)
#[derive(Default)]
pub struct PingProgram;

// Ping program, it host one or more services and it expose them to the 
// externar consumer.
// Only one program is allowed per application
#[program]
impl PingProgram {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        KeyringService::seed();
        PingService::seed(msg::source());

        Self
    }

    // Method working with "&self", havind no other parameters are treated as exposed
    // service constructors, and are called each time when an incoming request message 
    // needs be dispatched to a selected service
    // It has "message routing", This will change the way a service will be called 
    // (if omitted, the method name will be used, in this case TrafficLightSvc).
    #[route("Ping")]
    pub fn ping_svc(&self) -> PingService {
        PingService::new()
    }

    // Method working with "&self", havind no other parameters are treated as exposed
    // service constructors, and are called each time when an incoming request message 
    // needs be dispatched to a selected service
    // It has "message routing", This will change the way a service will be called 
    // (if omitted, the method name will be used, in this case KeyringSvc).
    #[route("KeyringService")]
    pub fn keyring_svc(&self) -> KeyringService {
        KeyringService::new()
    }

    // Method working with "&self", havind no other parameters are treated as exposed
    // service constructors, and are called each time when an incoming request message 
    // needs be dispatched to a selected service
    // It has "message routing", This will change the way a service will be called 
    // (if omitted, the method name will be used, in this case QuerySvc).
    #[route("QueryService")]
    pub fn query_svc(&self) -> QueryService {
        QueryService::new()
    }


}