use sails_rs::{
    prelude::*,
    gstd::msg
};

use crate::states::ping_state::PingState;

use keyring_service::{
    state::KeyringAccounts,
    service_enums::KeyringError
};

static mut PING_STATE: Option<PingState> = None;

#[derive(Default)]
pub struct PingService;

impl PingService {
    // Service state initializer 
    pub fn seed(caller: ActorId) {
        unsafe {
            PING_STATE = Some(PingState::new(caller))
        }
    } 

    // related functions that returns the state as mut
    pub fn state_mut() -> &'static mut PingState {
        let state = unsafe { PING_STATE.as_mut() };
        debug_assert!(state.is_some(), "State is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // related functions that returns the state as ref
    pub fn state_ref() -> &'static PingState {
        let state = unsafe { PING_STATE.as_ref() };
        debug_assert!(state.is_some(), "State is not initialized");
        unsafe { state.unwrap_unchecked() }
    }
}

#[service]
impl PingService {
    // Service constructor
    pub fn new() -> Self {
        Self
    }

    // Remote call "ping" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for "normal" calls or with vouchers.
    pub fn ping(&mut self) -> PingEvent{
        let caller = msg::source();
        self.handle_ping(caller)
    }

    // Remote call "ping_signless" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for signless calls, receives the address of the user
    // who is owner of the signless account
    pub fn ping_signless(
        &mut self,
        user_address: ActorId
    ) -> PingEvent {
        let caller = msg::source();

        // Need to check if the signless addres is afiliated to the user address
        let keyring_result = KeyringAccounts::state_ref()
            .check_keyring_address_by_user_address(
                caller, 
                user_address
            );
        
        if let Err(keyring_error) = keyring_result {
            return PingEvent::KeyringError(keyring_error);
        }
        
        // Call method with the use address as the caller
        self.handle_ping(user_address)
    }

    // Remote call "ping_no_wallet" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for calls with no wallet, receives the name encoded 
    // string of the user who is owner of the signless account
    pub fn ping_no_wallet(
        &mut self,
        user_coded_name: String
    ) -> PingEvent {
        let caller = msg::source();

        // Need to check if the signless addres is afiliated to the user encoded name
        let keyring_result = KeyringAccounts::state_ref()
            .check_keyring_address_by_user_coded_name(
                caller, 
                user_coded_name
            );
        
        if let Err(keyring_error) = keyring_result {
            return PingEvent::KeyringError(keyring_error);
        }

        self.handle_ping(caller)
    }

    // Remote call "pong" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for "normal" calls or with vouchers.
    pub fn pong(&mut self) -> PingEvent{
        let caller = msg::source();
        self.handle_pong(caller)
    }

    // Remote call "pong_signless" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for signless calls, receives the address of the user
    // who is owner of the signless account
    pub fn pong_signless(
        &mut self,
        user_address: ActorId
    ) -> PingEvent {
        let caller = msg::source();

        // Need to check if the signless addres is afiliated to the user address
        let keyring_result = KeyringAccounts::state_ref()
            .check_keyring_address_by_user_address(
                caller, 
                user_address
            );
        
        if let Err(keyring_error) = keyring_result {
            return PingEvent::KeyringError(keyring_error);
        }
            
        self.handle_pong(user_address)
    }

    // Remote call "pong_no_wallet" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    // It is used for calls with no wallet, receives the name encoded 
    // string of the user who is owner of the signless account
    pub fn pong_no_wallet(
        &mut self,
        user_coded_name: String
    ) -> PingEvent {
        let caller = msg::source();

        // Need to check if the signless addres is afiliated to the user encoded name
        let keyring_result = KeyringAccounts::state_ref()
            .check_keyring_address_by_user_coded_name(
                caller, 
                user_coded_name
            );
        
        if let Err(keyring_error) = keyring_result {
            return PingEvent::KeyringError(keyring_error);
        }

        self.handle_pong(caller)
    }
}


// Methods to help service methods to handle common behaviors
impl PingService {
    fn handle_ping(
        &mut self, 
        caller: ActorId
    ) -> PingEvent {
        Self::state_mut()
            .last_who_call = caller;

        PingEvent::Pong
    }

    fn handle_pong(
        &mut self, 
        caller: ActorId
    ) -> PingEvent {
        Self::state_mut()
            .last_who_call = caller;

        PingEvent::Ping
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum PingEvent {
    Ping,
    Pong,
    KeyringError(KeyringError),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TestEnum {
    First(String),
    Second(u64),
    Third(TestStruct),
    Four(Vec<String>),
    Five(Vec<(u64, String)>)
}

#[derive(Encode, Decode, TypeInfo)]
pub struct TestStruct {
    name: String,
    age: u64
}