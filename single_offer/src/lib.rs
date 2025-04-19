#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[derive(Clone)]
pub struct Offer {
    pub seller: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_price: i128,
    pub buy_price: i128,
}

#[contract]
pub struct SingleOfferContract;

#[contractimpl]
impl SingleOfferContract {
    pub fn create_offer(
        env: Env,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: i128,
        buy_price: i128,
    ) {
        seller.require_auth();

        let offer = Offer {
            seller,
            sell_token,
            buy_token,
            sell_price,
            buy_price,
        };

        env.storage().instance().set(&Symbol::short("OFFER"), &offer);
    }

    pub fn trade(env: Env, buyer: Address, sell_amount: i128) {
        let offer: Offer = env.storage().instance().get(&Symbol::short("OFFER")).unwrap();
        buyer.require_auth();

        let buy_amount = sell_amount * offer.buy_price / offer.sell_price;
        // İşlem burada simüle ediliyor. Gerçek token kontratları ile etkileşim burada yapılır.
        env.events().publish(
            (Symbol::short("TRADE"), &buyer),
            (sell_amount, buy_amount),
        );
    }

    pub fn update_price(env: Env, seller: Address, new_sell_price: i128, new_buy_price: i128) {
        let mut offer: Offer = env.storage().instance().get(&Symbol::short("OFFER")).unwrap();
        seller.require_auth();

        if offer.seller != seller {
            panic!("Yetkisiz erişim");
        }

        offer.sell_price = new_sell_price;
        offer.buy_price = new_buy_price;

        env.storage().instance().set(&Symbol::short("OFFER"), &offer);
    }

    pub fn get_offer(env: Env) -> Offer {
        env.storage().instance().get(&Symbol::short("OFFER")).unwrap()
    }
}
