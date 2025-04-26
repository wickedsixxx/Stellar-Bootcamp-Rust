# Atomic Swap Contract

This project implements an **Atomic Swap** smart contract for the Stellar Bootcamp Rust program.

The contract allows two different tokens to be swapped directly, securely, and atomically (i.e., in a single indivisible operation) without the need for a centralized exchange.

---

## 📜 Features

- Allow a seller to offer a specific token for trade.
- Allow a buyer to trade tokens according to the seller's offer.
- Allow the seller to withdraw unsold tokens.
- Allow the seller to update the offer price dynamically.

---

## 🛠️ Main Functions

### `create_offer(env, seller, sell_token, buy_token, sell_price, buy_price)`
- Creates a trade offer.
- `seller`: Address that wants to sell tokens.
- `sell_token`: The token that will be sold.
- `buy_token`: The token that will be received.
- `sell_price` and `buy_price`: Define the exchange rate between the two tokens.

### `trade(env, buyer, buy_token_amount, min_sell_token_amount)`
- Allows a buyer to trade tokens according to the offer.
- Buyer receives the appropriate amount of sell_token.
- Trade fails if the minimum acceptable sell_token amount is not satisfied.

### `withdraw(env, token_address, amount)`
- Allows the seller to withdraw unsold tokens from the offer.

### `update_price(env, sell_price, buy_price)`
- Allows the seller to update the offer price after creation.

---

## 📦 How to Use

1. **Deploy** the contract on the Stellar Soroban environment.
2. **Create** an offer by calling `create_offer` with seller, tokens, and initial prices.
3. **Deposit** tokens into the contract address.
4. **Buyers** can call `trade` to perform an atomic swap based on the current offer.
5. **Seller** can `withdraw` unsold tokens or `update_price` as needed.

---

## 🚀 Example Workflow

```rust
let env = Env::default();
let seller = Address::generate(&env);
let buyer = Address::generate(&env);

let sell_token = Address::generate(&env);
let buy_token = Address::generate(&env);

let offer = AtomicSwapContractClient::new(&env, &env.register_contract(None, AtomicSwapContract));
offer.create_offer(&seller, &sell_token, &buy_token, &sell_price, &buy_price);

// Buyer trades tokens
offer.trade(&buyer, &buy_token_amount, &min_sell_token_amount);

// Seller withdraws tokens or updates price
offer.withdraw(&sell_token, &amount);
offer.update_price(&new_sell_price, &new_buy_price);
