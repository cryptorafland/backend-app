extern crate core;

use std::borrow::BorrowMut;
use borsh::{self, BorshDeserialize, BorshSerialize};
use near_contract_standards::non_fungible_token::TokenId;
use near_rng::Rng;
use near_sdk::collections::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::{AccountId, env, log, near_bindgen, Balance, Promise};
use near_sdk::env::block_timestamp_ms;
use near_sdk::{
    serde::{Deserialize, Serialize}
};

/**
  * now only 1 winner
  * now only 1 prize
  * now start only now
  */

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
pub const ADD_PART_CALL_COST: u128 = 1_000_000_000_000_000_000_000;
const DEFAULT_COUNTER: u128 = 0;
const DEFAULT_MESSAGE: &str = "Hello";


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RafflesMap {
    raffles: UnorderedMap<u128, Raffle>,
    counter: Counter,
    pub beneficiary: AccountId,
    greeting: String,
}

impl Default for RafflesMap {

    fn default() -> Self {
        RafflesMap {
            raffles: UnorderedMap::new(b"m"),
            counter: Counter{ value: DEFAULT_COUNTER },
            //counter: Default::default(),
            beneficiary: "v1.faucet.nonofficial.testnet".parse().unwrap(),
            greeting: DEFAULT_MESSAGE.to_string(),
        }
    }
}

#[near_bindgen]
impl RafflesMap {

    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init(beneficiary: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            beneficiary: beneficiary,
            counter: Counter{ value: DEFAULT_COUNTER },
            raffles: UnorderedMap::new(b"m"),
            greeting: DEFAULT_MESSAGE.to_string(),
        }
    }

    pub fn get_greeting(&self) -> String {
        return self.greeting.clone();
    }

    pub fn get_counter(&self) -> &u128 {
        &self.counter.value
    }

    pub fn get_counter1(&self) -> u128 {
        return self.counter.value.clone();
    }

    fn get_raffle(&self, key: u128) -> Option<Raffle> {
        self.raffles.get(&key)
    }

    fn get_random_participant(&self, key: &u128) -> Option<AccountId> {
        self.raffles.get(key).unwrap().get_random_participant()
    }

    // TODO: get_random_prize
    // fn get_random_prize(&mut self, key: &u128) -> Option<JsonToken> {
    //     self.raffles.get(key).unwrap().get_random_prize()
    // }

    fn get_prize(&self, key: &u128, counter: u64) -> Option<JsonToken> {
        self.raffles.get(key).unwrap().get_prize(counter)
    }

    #[payable]
    fn add_participant(&mut self, key: u128, sender: &AccountId) -> bool {

        // let sender: AccountId = env::predecessor_account_id();
        let pays: Balance = env::attached_deposit();

        let ticket_price: u128 = self.raffles.get(&key).unwrap().get_ticket_price();

        let to_transfer: Balance = if pays >= ticket_price {
            // Subtract the storage cost to the amount to transfer
            ticket_price - STORAGE_COST 
        } else {
            0
        };

        if to_transfer > 0 {
            // Transfer the amount to the beneficiary
            Promise::new(self.beneficiary.clone()).transfer(to_transfer);
            let return_back: Balance = pays - ticket_price;
            Promise::new(sender.clone()).transfer(return_back);

            let mut current_raffle = self.raffles.get(&key).unwrap();
            let mut participants = current_raffle.participants;
            let participant_exist = participants.insert(sender);
            if participant_exist {
                current_raffle.participants = participants;
                self.raffles.insert(&key, &current_raffle);

                Promise::new(self.beneficiary.clone()).transfer(to_transfer);
                let return_back: Balance = pays - ticket_price;
                Promise::new(sender.clone()).transfer(return_back);

                true
            } else {
                Promise::new(sender.clone()).transfer(pays - ADD_PART_CALL_COST);
                false
            }
        } else {
            false
        }


    }

    fn set_counter(&mut self, counter: u128) {
        log!("Saving counter {}", counter);
        self.counter.value = counter;
    }

    pub fn increment_counter(&mut self) {
        self.counter.value += 1;
    }

    fn add_new_raffle(
        &mut self, 
        // args: Base64VecU8
        end_time: u64, 
        ticket_price: u128, 
        prizes: Vector<JsonToken>
    ) {
        self.increment_counter();

        // let raffle_args: NewRaffleArgs = serde_json::from_slice(&args.0.as_slice()).unwrap();

        // let end_time = u64::from(raffle_args.end_time);
        // let ticket_price = u128::from(raffle_args.ticket_price);
        // let prizes = Vector<JsonToken>::from(raffle_args.prizes);

        let winners: Vector<Winner> = Vector::new(b"t");
        let participants: UnorderedSet<AccountId> = UnorderedSet::new(b"s");

        // // increment counter
        // let mut counter = self.get_counter();
        // counter = counter + 1;
        // self.set_counter(counter);

        // take creator id
        let creator: AccountId = env::predecessor_account_id();

        // TODO: calculate end time

        let new_raffle: Raffle = Raffle {
            end_time: end_time,
            prizes: prizes,
            ticket_price: ticket_price,
            creator_wallet_account_id: creator,
            game_continues: true,
            winners: winners,
            participants: participants,
        };

        let counter = *self.get_counter();

        self.raffles.insert(&counter, &new_raffle);
    }

    fn cancel_raffle(&mut self, key: u128) -> bool {
        // TODO: UNCOMMENT!!!
        // if self.raffles.get(&key).unwrap().get_end_time() <= &block_timestamp_ms() {
            if self.raffles.get(&key).unwrap().get_participants().is_empty() {
                // TODO: send all prizes to creator
                true
            } else {
                for _x in 0..self.raffles.get(&key).unwrap().prizes.len() {
                    //TODO: что делать если остались лишние призы

                    // take random winners and delete from collection participants
                    let winner_account: AccountId = self.get_random_participant(&key).unwrap();

                    // take prize
                    let prize: JsonToken = self.get_prize(&key, _x).unwrap();

                    // TODO в случае выбора приза на рандоме
                    // если уже был приз то ставить ему фоллс для этого призы на мап
                    // take random prize and sale false or delete fromm collection
                    // let prize: JsonToken = self.get_random_prize(&key).unwrap();

                    // add winners to winners
                    let winner: Winner = Winner {
                        winner_wallet_account_id: winner_account,
                        prize: prize,
                    };
                    self.add_winner(key, winner);

                    // TODO: send price
                }
                self.set_game_continues(false, key);
                true
            }
        // } else {
        //     TODO: вывести когда конец
        //     false
        // }
    }

    fn add_winner(&mut self, key: u128, winner: Winner) {
        let mut current_raffle = self.raffles.get(&key).unwrap();
        let mut winners = current_raffle.winners;
        winners.push(&winner);
        current_raffle.winners = winners;
        self.raffles.insert(&key, &current_raffle);
    }

    fn set_game_continues(&mut self, continues: bool, key: u128) {
        let mut current_raffle = self.raffles.get(&key).unwrap();
        current_raffle.game_continues = continues;
        self.raffles.insert(&key, &current_raffle);
    }
}

// #[near_bindgen]
#[derive(Eq, Default, Hash, PartialEq, PartialOrd, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    value: u128,
}

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct NewRaffleArgs {
    end_time: u64,
    ticket_price: u128,
    prizes: Vector<JsonToken>
}

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Raffle {
    //TODO: end time or continues
    end_time: u64,
    prizes: Vector<JsonToken>,
    ticket_price: u128,
    creator_wallet_account_id: AccountId,
    game_continues: bool,
    winners: Vector<Winner>,
    participants: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Raffle {
    // fn add_participant(&mut self, id: AccountId) {
    //     self.participants.push(id);
    // }

    // fn shuffle_participant(&mut self) {
    //     self.participants.shuffle(&mut thread_rng());
    // }

    fn get_ticket_price(&self) -> u128 {
        return self.ticket_price;
    }

    fn get_creator(&self) -> &AccountId {
        return &self.creator_wallet_account_id
    }

    fn get_participants(&self) -> &UnorderedSet<AccountId> {
        &self.participants
    }

    fn get_winners(&self) -> &Vector<Winner> {
        &self.winners
    }

    fn get_prizes(&self) -> &Vector<JsonToken> {
        return &self.prizes
    }

    fn get_prize(&mut self, counter: u64) -> Option<JsonToken> {
        self.prizes.get(counter)
    }

    fn get_random_participant(&self) -> Option<AccountId> {
        let mut rng = Rng::new(&env::random_seed());
        let random_number = rng.rand_range_u64(0, self.participants.len());
        self.participants.as_vector().get(random_number)
    }

    // fn get_random_winner(&self) -> &AccountId {
    //     self.get_participants().choose(&mut thread_rng()).unwrap()
    // }

    fn game_continues(&self) -> &bool {
        &self.game_continues
    }


    fn get_end_time(&self) -> &u64 {
        &self.end_time
    }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
}

// #[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Winner {
    winner_wallet_account_id: AccountId,
    prize: JsonToken,
}

// TODO: перед главным запуском
// удалить все фулл аксес код
// оставить ключи для вызова функций администативных

// TODO MAYBE
// use near collection in state!!!
// метод что позволит тем кто у нас хранит токен за обновление голосовать

#[near_bindgen]
impl Winner {

    //TODO: send prize to winner
    fn send_prize_to_winner(&mut self) {
        // записать как праметр и отправлять через селф.
        // между контрактный вызов нфт
        // cross contract call nft
        // method take my price (backend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::Balance;

    const BENEFICIARY: &str = "beneficiary";
    const NEAR: u128 = 1000000000000000000000000;

    #[test]
    fn get_default_counter() {
        // let contract = RafflesMap::default();
        let contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        assert_eq!(contract.get_counter().clone(), 0);
        // assert_eq!(contract.get_counter(), 0);
    }

    #[test]
    fn random() {
        let mut rng = Rng::new(&env::random_seed());
        let random_number1 = rng.rand_range_u64(0, 10);
        let random_number2 = rng.rand_range_u64(0, 10);
        // only sometime true
        assert_eq!(random_number, random_number1);
    }

    #[test]
    fn set_then_get_counter() {
        // let mut contract = RafflesMap::default();
        let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        assert_eq!(contract.get_counter().clone(), 0);
        contract.set_counter(1);
        assert_eq!(contract.get_counter().clone(), 1);
    }

    #[test]
    fn test_new_created_raffle() {
        // let mut contract = RafflesMap::default();
        let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        let mut vec: Vector<JsonToken> = Vector::new(b"m");
        vec.push(&JsonToken {
            token_id: "1111".to_string(),
            owner_id: env::predecessor_account_id(),
        });
        contract.add_new_raffle(1, 1, vec);

        assert_eq!(contract.raffles.get(&1u128).unwrap().participants.is_empty(), true);
        assert_eq!(contract.raffles.get(&1u128).unwrap().winners.is_empty(), true);
        assert_eq!(contract.raffles.get(&1u128).unwrap().ticket_price, 1);
        assert_eq!(contract.raffles.get(&1u128).unwrap().end_time, 1);
        assert_eq!(contract.raffles.get(&1u128).unwrap().game_continues, true);
        assert_eq!(contract.get_counter().clone(), 1);

        let mut vec1: Vector<JsonToken> = Vector::new(b"m");
        vec1.push(&JsonToken {
            token_id: "1111".to_string(),
            owner_id: env::predecessor_account_id(),
        });

        assert_eq!(contract.raffles.get(&1u128).unwrap().prizes.get(0).unwrap().owner_id.to_string(), "bob.near");
        assert_eq!(contract.raffles.get(&1u128).unwrap().prizes.get(0).unwrap().token_id.to_string(), "1111");
    }

    #[test]
    fn add_participant() {
        let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        let mut vec: Vector<JsonToken> = Vector::new(b"s");
        vec.push(&JsonToken {
            token_id: "1111".to_string(),
            owner_id: env::predecessor_account_id(),
        });
        contract.add_new_raffle(1, 1, vec);
        contract.add_participant(1, &env::predecessor_account_id());
        

        assert_eq!(contract.get_raffle(1u128).unwrap().participants.is_empty(), false);
        assert_eq!(contract.raffles.get(&1u128).unwrap().participants.as_vector().get(0).unwrap().to_string(), env::predecessor_account_id().to_string());

        //Vanias code!
        // let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        // contract.start_new_raffle(1, 10*NEAR, JsonToken { token_id: "1111".to_string(), owner_id: env::predecessor_account_id() });

        // set_context("donor_a", 1000*NEAR);

        // contract.add_participant(1, "donor_a".parse().unwrap());
        // assert_eq!(contract.get_raffle(1).cloned().unwrap().get_participants().get(0).unwrap().to_string(), "donor_a");
    }

    #[test]
    fn test_winner_and_game_continues() {
        let mut contract = RafflesMap::default();
        let mut vec: Vector<JsonToken> = Vector::new(b"m");
        vec.push(&JsonToken {
            token_id: "1111".to_string(),
            owner_id: env::predecessor_account_id(),
        });
        contract.add_new_raffle(1, 1, vec);
        contract.add_participant(1, &env::predecessor_account_id());

        let nft = JsonToken {
            token_id: "1111".to_string(),
            owner_id: env::predecessor_account_id(),
        };

        assert_eq!(contract.get_raffle(1u128).unwrap().participants.is_empty(), false);
        assert_eq!(contract.raffles.get(&1u128).unwrap().prizes.get(0).unwrap().owner_id.to_string(), "bob.near");
        assert_eq!(contract.raffles.get(&1u128).unwrap().prizes.get(0).unwrap().token_id.to_string(), "1111");
        assert_eq!(contract.raffles.get(&1u128).unwrap().participants.as_vector().get(0).unwrap().to_string(), "bob.near");

        contract.cancel_raffle(1u128);

        assert_eq!(contract.get_raffle(1u128).unwrap().winners.is_empty(), false);
        assert_eq!(contract.raffles.get(&1u128).unwrap().winners.get(0).unwrap().winner_wallet_account_id.to_string(), "bob.near");
        assert_eq!(contract.raffles.get(&1u128).unwrap().winners.get(0).unwrap().prize, nft);
        assert_eq!(contract.raffles.get(&1u128).unwrap().game_continues, false);
    }

    // #[test]
    // fn add_participant() {
    //     let mut contract = RafflesMap::default();
    //     contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: env::predecessor_account_id() });
    //     contract.add_participant(1, env::predecessor_account_id());
    //     assert_eq!(contract.get_raffle(1).cloned().unwrap().get_participants().get(0).unwrap().to_string(), "bob.near");
    // }

    #[test]
    fn test_one_acc_one_time() {
        let mut contract = RafflesMap::default();
        let mut vec: Vector<JsonToken> = Vector::new(b"m");

        vec.push(&JsonToken {
            token_id: "1".to_string(),
            owner_id: env::predecessor_account_id(),
        });
        vec.push(&JsonToken {
            token_id: "2".to_string(),
            owner_id: env::predecessor_account_id(),
        });
        vec.push(&JsonToken {
            token_id: "3".to_string(),
            owner_id: env::predecessor_account_id(),
        });
        vec.push(&JsonToken {
            token_id: "4".to_string(),
            owner_id: env::predecessor_account_id(),
        });

        contract.add_new_raffle(1, 1, vec);

        contract.add_participant(1, &env::predecessor_account_id());
        contract.add_participant(1, &env::predecessor_account_id());
        contract.add_participant(1, &env::predecessor_account_id());
        contract.add_participant(1, &env::predecessor_account_id());

        assert_eq!(contract.raffles.get(&1u128).unwrap().participants.len(), 1);
        assert_eq!(contract.add_participant(1, &env::predecessor_account_id()), false);
        assert_eq!(contract.add_participant(1, &AccountId::new_unchecked("alice.near".to_string())), true);
        assert_eq!(contract.raffles.get(&1u128).unwrap().participants.len(), 2);
    }

    // TODO: test for time cancel
    // #[test]
    // fn test_cancel_time() {
    //
    // }
}