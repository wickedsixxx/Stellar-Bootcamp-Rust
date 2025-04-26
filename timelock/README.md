# Timelock Contract

This is a simple **Timelock** smart contract written for the Stellar Bootcamp Rust project.

It allows locking tokens for a specific duration.  
After the lock period expires, the tokens can be withdrawn by the owner.

---

## 📜 Features

- Lock tokens until a specific future time.
- Prevent withdrawal before the unlock time.
- Allow withdrawal once the unlock time has passed.
- Basic authorization and validation checks.

---

## 🛠️ Functions

### `create(env, admin, unlock_time)`
- Initializes the Timelock contract.
- `admin`: Address of the account that will control the lock.
- `unlock_time`: UNIX timestamp when tokens can be unlocked.

### `lock(env, token_address, amount)`
- Locks a given `amount` of tokens in the contract.

### `withdraw(env, token_address, amount)`
- Withdraws the locked tokens **only if the current time >= unlock_time**.
- Fails if attempted before the unlock time.

---

## 📦 How to Deploy & Use

1. Deploy the contract using the Stellar Soroban environment.
2. Initialize it with your address and unlock timestamp using `create`.
3. Lock desired tokens using `lock`.
4. Attempt withdrawal after the unlock time using `withdraw`.

---

## 🚀 Example Usage

```rust
let env = Env::default();
let admin = Address::generate(&env);
let token_address = Address::generate(&env);

let contract = TimelockContractClient::new(&env, &env.register_contract(None, TimelockContract));
contract.create(&admin, &unlock_time);

// Lock tokens
contract.lock(&token_address, &amount);

// Withdraw tokens after unlock time
contract.withdraw(&token_address, &amount);
