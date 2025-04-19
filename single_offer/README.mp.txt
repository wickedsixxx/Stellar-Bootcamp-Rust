# Single Offer Smart Contract

This smart contract is designed for the Stellar Soroban platform to simulate a simple single-offer trading scenario.

## Features
- `create_offer`: Seller can create an offer with price and token details.
- `trade`: Buyer can execute a trade based on the current offer.
- `update_price`: Seller can update the price of an existing offer.
- `get_offer`: Retrieve current offer details.

## Tech
- Written in Rust
- Built with Soroban SDK v20
- Tested using Soroban's testutils

## Folder Structure
- `src/lib.rs`: Core contract logic
- `src/test.rs`: Unit tests
