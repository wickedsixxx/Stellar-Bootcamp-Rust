#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_create_and_trade() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SingleOfferContract);
    let client = SingleOfferContractClient::new(&env, &contract_id);

    let seller = Address::random(&env);
    let buyer = Address::random(&env);

    env.set_auths(&[(seller.clone(), vec![]), (buyer.clone(), vec![])]);

    client.create_offer(
        &seller,
        &Address::from_contract_id(&[0; 32]),
        &Address::from_contract_id(&[1; 32]),
        &100,
        &50,
    );

    client.trade(&buyer, &200);

    let offer = client.get_offer();
    assert_eq!(offer.sell_price, 100);
    assert_eq!(offer.buy_price, 50);
}
