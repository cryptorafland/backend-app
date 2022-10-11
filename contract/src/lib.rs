extern crate core;

use std::borrow::BorrowMut;
use borsh::{BorshDeserialize, BorshSerialize};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::collections::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::{env, log, near_bindgen, AccountId};
use near_sdk::env::block_timestamp_ms;
use rand::Rng;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RafflesMap {
    raffles: UnorderedMap<u128, Raffle>,
    counter: Counter,
}

impl Default for RafflesMap {
    fn default() -> Self {
        RafflesMap {
            raffles: UnorderedMap::new(b"m"),
            counter: Default::default(),
        }
    }
}

#[near_bindgen]
impl RafflesMap {
    fn get_counter(&self) -> &u128 {
        &self.counter.value
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

    // return true (if we add new participant) or false
    fn add_participant(&mut self, key: u128, id: &AccountId) -> bool {
        let mut current_raffle = self.raffles.get(&key).unwrap();
        let mut participants = current_raffle.participants;
        let participant_exist = participants.insert(id);
        if participant_exist {
            current_raffle.participants = participants;
            self.raffles.insert(&key, &current_raffle);
            true
        } else {
            false
        }
    }

    fn set_counter(&mut self, counter: u128) {
        log!("Saving counter {}", counter);
        self.counter.value = counter;
    }

    fn increment_counter(&mut self) {
        self.counter.value += 1;
    }

    fn add_new_raffle(&mut self, end_time: u64, ticket_price: u32, prizes: Vector<JsonToken>) {
        self.increment_counter();

        let winners: Vector<Winner> = Vector::new(b"t");
        let participants: UnorderedSet<AccountId> = UnorderedSet::new(b"s");

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

#[near_bindgen]
#[derive(Eq, Default, Hash, PartialEq, PartialOrd, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    value: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Raffle {
    //TODO: end time or continues
    end_time: u64,
    prizes: Vector<JsonToken>,
    ticket_price: u32,
    creator_wallet_account_id: AccountId,
    game_continues: bool,
    winners: Vector<Winner>,
    participants: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Raffle {
    fn get_creator(&self) -> &AccountId {
        &self.creator_wallet_account_id
    }

    fn get_participants(&self) -> &UnorderedSet<AccountId> {
        &self.participants
    }

    fn get_winners(&self) -> &Vector<Winner> {
        &self.winners
    }

    fn get_prizes(&self) -> &Vector<JsonToken> {
        &self.prizes
    }

    fn get_random_participant(&self) -> Option<AccountId> {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..self.participants.len());
        self.participants.as_vector().get(random_number)
    }

    // TODO: get_random_prize
    // fn get_random_prize(&mut self) -> Option<JsonToken> {
    //     let mut rng = rand::thread_rng();
    //     let random_number = rng.gen_range(0..self.prizes.len());
    //     self.prizes.get(random_number)
    // }

    fn get_prize(&mut self, counter: u64) -> Option<JsonToken> {
        self.prizes.get(counter)
    }

    fn game_continues(&self) -> &bool {
        &self.game_continues
    }


    fn get_end_time(&self) -> &u64 {
        &self.end_time
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq)]
pub struct JsonToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
}

#[near_bindgen]
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

    #[test]
    fn get_default_counter() {
        let contract = RafflesMap::default();
        assert_eq!(contract.get_counter().clone(), 0);
    }

    #[test]
    fn set_then_get_counter() {
        let mut contract = RafflesMap::default();
        assert_eq!(contract.get_counter().clone(), 0);

        contract.set_counter(1);
        assert_eq!(contract.get_counter().clone(), 1);
    }

    #[test]
    fn test_new_created_raffle() {
        let mut contract = RafflesMap::default();
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
        let mut contract = RafflesMap::default();
        let mut vec: Vector<JsonToken> = Vector::new(b"s");
        vec.push(&JsonToken {
            token_id: "1111".to_string(),
            owner_id: env::predecessor_account_id(),
        });
        contract.add_new_raffle(1, 1, vec);
        contract.add_participant(1, &env::predecessor_account_id());

        assert_eq!(contract.get_raffle(1u128).unwrap().participants.is_empty(), false);
        assert_eq!(contract.raffles.get(&1u128).unwrap().participants.as_vector().get(0).unwrap().to_string(), "bob.near");
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

    #[test]
    fn test_random() {
        //FOR TESTING RANDOM UNCOMMENT AND SEE RESULT SEVERAL TIME

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
        contract.add_participant(1, &AccountId::new_unchecked("kot.near".to_string()));
        contract.add_participant(1, &AccountId::new_unchecked("vlad.near".to_string()));
        contract.add_participant(1, &AccountId::new_unchecked("alice.near".to_string()));

        let nft = JsonToken {
            token_id: "1".to_string(),
            owner_id: env::predecessor_account_id(),
        };
        let nft1 = JsonToken {
            token_id: "2".to_string(),
            owner_id: env::predecessor_account_id(),
        };

        contract.cancel_raffle(1u128);

        assert_eq!(contract.raffles.get(&1u128).unwrap().participants.len(), 4);
        assert_eq!(contract.raffles.get(&1u128).unwrap().prizes.len(), 4);
        assert_eq!(contract.raffles.get(&1u128).unwrap().winners.len(), 4);

        assert_eq!(contract.raffles.get(&1u128).unwrap().winners.get(0).unwrap().prize.token_id,
            contract.raffles.get(&1u128).unwrap().prizes.get(0).unwrap().token_id);
        // assert_eq!(contract.raffles.get(&1u128).unwrap().winners.get(0).unwrap().winner_wallet_account_id, nft.owner_id);

        assert_eq!(contract.raffles.get(&1u128).unwrap().winners.get(1).unwrap().prize.token_id,
            contract.raffles.get(&1u128).unwrap().prizes.get(1).unwrap().token_id);
        // assert_eq!(contract.raffles.get(&1u128).unwrap().winners.get(1).unwrap().winner_wallet_account_id.to_string(), "alice.near".to_string());
    }

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
