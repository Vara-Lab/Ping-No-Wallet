# Ping contract with no wallet functionality

To compile the contract yo need yo enter in 'wasm' and then put:

cargo build --release

## Architecture

A contract consists of two directories:
-	App: Where lives all business logic.
-	Wasm: Where the contract is built and the IDL is generated 

## Walletless feature

This contract contains the signless and walletless feature, it uses the service "KeyringService" and extends the "KeyringQueryService" in "QueryService" from the [keyring-service repository](https://github.com/Vara-Lab/Contracts-Services/tree/main/keyring-service)

### Services

- QueryService:

    This service in the contract extends the 'KeyringQueryService', which will give all the queries to obtain the keyring accounts, and the addresses of these keyring accounts.
    
    This service contains an extra field that will contains the extended service:

    ```rust
    // Service used for all queries
    pub struct QueryService {
        // To extend the wallet less service (and others services), yo need to create
        // an atribute to "store" the service
        keyring_query_service: KeyringQueryService
    }
    ```

    And, you need to implement the "AsRef" trait in the extended service for the service that you use to extend it:

    ```rust
    // You need to implement "AsRef" on the services that will extends another
    // service, in this case "KeyringQueryService"
    impl AsRef<KeyringQueryService> for QueryService {
        fn as_ref(&self) -> &KeyringQueryService {
            &self.keyring_query_service
        }
    }
    ```

- KeyringService:

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
