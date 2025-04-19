#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("admin"))
            .unwrap();
        admin.require_auth();

        let mut balance: i128 = env.storage().persistent().get(&to).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&to, &balance);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let mut from_balance: i128 =
            env.storage().persistent().get(&from).unwrap_or(0);
        let mut to_balance: i128 =
            env.storage().persistent().get(&to).unwrap_or(0);

        if from_balance < amount {
            panic!("Yetersiz bakiye");
        }

        from_balance -= amount;
        to_balance += amount;

        env.storage().persistent().set(&from, &from_balance);
        env.storage().persistent().set(&to, &to_balance);
    }

    pub fn balance(env: Env, user: Address) -> i128 {
        env.storage().persistent().get(&user).unwrap_or(0)
    }
}
