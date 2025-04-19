#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_token() {
    let env = Env::default();

    let contract_id = env.register_contract(None, TokenContract);
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::random(&env);
    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    env.set_auths(&[(admin.clone(), vec![])]);

    client.initialize(&admin);
    client.mint(&admin, &user1, &1000);
    client.transfer(&user1, &user2, &400);

    let balance1 = client.balance(&user1);
    let balance2 = client.balance(&user2);

    assert_eq!(balance1, 600);
    assert_eq!(balance2, 400);
}
