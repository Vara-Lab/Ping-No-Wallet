# Ping contract with no wallet functionality

To compile the contract yo need to execute the following command:

cargo build --release

## Architecture

A contract consists of two directories:
-	App: Where lives all business logic.
-	Wasm: Where the contract is built and the IDL is generated 

## Walletless feature

This contract contains the signless and walletless feature, it uses the service "KeyringService" from the [keyring-service repository](https://github.com/Vara-Lab/Contracts-Services/tree/main/keyring-service)

### Service

- KeyringService:

    This service contains all the methods to handle the walletless and signless features, and gives you the queries to obtain the keyring accounts and the addresses of these keyring accounts.

    This service comes from the [KeyringService](https://github.com/Vara-Lab/Contracts-Services/tree/main/keyring-service) repository, to use it, you just have to put it as another service in the program (This contract already implement it):

    ```rust
    // code ...

    #[derive(Default)]
    pub struct Porgram;

    #[program]
    impl Program {
        #[route("KeyringService")]
        pub fn keyring_svc(&self) -> KeyringService {
            KeyringService::new()
        }

        // code ...
    }
    ```

