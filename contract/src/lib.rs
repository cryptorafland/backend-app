use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{AccountId, env, log, near_bindgen};
use rand::seq::SliceRandom;
use rand::thread_rng;
use near_sdk::serde::{Deserialize, Serialize};

/**
  * now only 1 winner
  * now only 1 prize
  * now start only now
  */

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct RafflesMap {
    raffles: HashMap<u128, Raffle>,
    counter: Counter,
}

#[near_bindgen]
impl RafflesMap {
    fn get_counter(&self) -> u128 {
        return self.counter.value
    }

    fn get_raffle(&self, key: u128) -> Option<&Raffle> {
        self.raffles.get(&key)
    }

    fn get_mut_raffle(&mut self, key: u128) -> Option<&mut Raffle> {
        self.raffles.get_mut(&key)
    }

    fn shuffle_participant(&mut self, key: u128) {
        self.get_mut_raffle(key).unwrap().shuffle_participant();
    }

    fn add_participant(&mut self, key: u128, id: String) {
        self.raffles.get_mut(&key).unwrap().add_participant(id);
    }

    fn set_counter(&mut self, counter: u128) {
        log!("Saving counter {}", counter);
        self.counter.value = counter;
    }

    pub fn start_new_raffle(&mut self,end_time: u32, ticket_price: u32, prize: JsonToken) {
        self.add_new_raffle(end_time, ticket_price, prize);

        // wait some time
        // IN THIS TIME WE NEED TO ADD PARTICIPANT
        // thread::spawn(|| {
        //     sleep(Duration::from_secs(5));
        // });

        self.cancel_raffle();
    }

    fn add_new_raffle(&mut self, end_time: u32, ticket_price: u32, prize: JsonToken) {
        // create structure for this game
        let participants_in_this_game: Vec<String> =  Vec::new();
        let winners_in_this_game: Vec<Winner> =  Vec::new();
        let mut prizes_in_this_game: Vec<JsonToken> =  Vec::new();

        prizes_in_this_game.push(prize);

        // increment counter
        let mut counter = self.get_counter();
        counter = counter + 1;
        self.set_counter(counter);

        // take creator id
        let creator: String = env::predecessor_account_id().as_str().to_string();

        // create new raffle
        let new_raffle: Raffle = Raffle {
            end_time: end_time,
            prizes: prizes_in_this_game,
            ticket_price: ticket_price,
            creator_wallet_account_id: creator,
            game_continues: true,
            winners: winners_in_this_game,
            participants: participants_in_this_game,
        };

        // add new raffle to map
        self.raffles.insert(self.get_counter(), new_raffle);
    }

    fn cancel_raffle(&mut self) {
        if self.get_mut_raffle(self.get_counter()).unwrap().get_participants().is_empty() {
            // send price to creator
        } else {
            // shuffle participants
            self.shuffle_participant(self.get_counter());

            // take random winner
            let winner: String = self.get_mut_raffle(self.get_counter()).unwrap().get_random_winner();

            // add winner to winners
            self.get_mut_raffle(self.get_counter()).unwrap().add_winner(winner);

            // send price

            // set that raffle is canceled
            self.get_mut_raffle(self.get_counter()).as_mut().unwrap().set_game_continues(false);
        }
    }
}



#[near_bindgen]
#[derive(Eq, Default, Hash, PartialEq, PartialOrd, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    value: u128,
}



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default, Clone)]
pub struct Raffle {
    end_time: u32,
    prizes: Vec<JsonToken>,
    ticket_price: u32,
    creator_wallet_account_id: String,
    game_continues: bool,
    winners: Vec<Winner>,
    participants: Vec<String>
}

#[near_bindgen]
impl Raffle {
    fn add_participant(&mut self, id: String) {
        self.participants.push(id);
    }

    fn shuffle_participant(&mut self) {
        self.participants.shuffle(&mut thread_rng());
    }

    fn get_creator(&self) -> &String {
        return &self.creator_wallet_account_id
    }

    fn get_participants(&self) -> &Vec<String> {
        return &self.participants
    }

    fn get_winners(&self) -> &Vec<Winner> {
        return &self.winners
    }

    fn get_prize(&self) -> &Vec<JsonToken> {
        return &self.prizes
    }

    fn get_random_winner(&self) -> String {
        self.get_participants().choose(&mut thread_rng()).unwrap().to_string()
    }

    fn add_winner(&mut self, winner: String) {
        let prize: Option<&JsonToken> = self.prizes.get(0);
        let new_winner: Winner = Winner {
            winner_wallet_account_id: winner,
            prize: prize.unwrap().clone(),
        };
        &self.winners.push(new_winner);
    }

    fn game_continues(&self) -> &bool {
        return &self.game_continues
    }

    fn set_game_continues(&mut self, continues: bool) {
        self.game_continues = continues;
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    //token ID
    pub token_id: TokenId,
    //owner of the token
    pub owner_id: AccountId,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Winner {
    winner_wallet_account_id: String,
    prize: JsonToken,
}










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_counter() {
        let contract = RafflesMap::default();
        assert_eq!(contract.get_counter(), 0);
    }

    #[test]
    fn set_then_get_counter() {
        let mut contract = RafflesMap::default();
        assert_eq!(contract.get_counter(), 0);
        contract.set_counter(1);
        assert_eq!(contract.get_counter(), 1);
    }

    #[test]
    fn counter_after_creation_raffle() {
        let mut contract = RafflesMap::default();
        assert_eq!(contract.get_counter(), 0);
        contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: ("qqqq".to_string()).parse().unwrap() });
        assert_eq!(contract.get_counter(), 1);
    }

    #[test]
    fn get_creator() {
        let mut contract = RafflesMap::default();
        contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: ("qqqq".to_string()).parse().unwrap() });
        let raffle = contract.raffles.get(&1u128);
        let d = raffle.cloned();
        assert_eq!(d.unwrap().get_creator(), "bob.near");
    }

    #[test]
    fn get_raffle() {
        let mut contract = RafflesMap::default();
        contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: ("qqqq".to_string()).parse().unwrap() });
        assert_eq!(contract.get_raffle(1).cloned().unwrap().get_creator(), "bob.near");
    }

    #[test]
    fn add_participant() {
        let mut contract = RafflesMap::default();
        contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: ("qqqq".to_string()).parse().unwrap() });
        contract.add_participant(1, "111111".to_string());
        assert_eq!(contract.get_raffle(1).cloned().unwrap().get_participants().get(0).unwrap().to_string(), "111111");
    }

    // #[test]
    // fn get_winner() {
    //     let mut contract = RafflesMap::default();
    //     contract.start_new_raffle(1, 1);
    //     let handle = thread::spawn(move || {
    //         for i in 1..5 {
    //             &mut contract.add_participant(1, "1".to_string());
    //         }
    //     });
    //     handle.join().unwrap();
    //     assert_eq!(contract.get_raffle(1).cloned().unwrap().get_participants().get(0).unwrap().to_string(), "1");
    //     // assert_eq!(contract.get_raffle(1).cloned().unwrap().get_winners().get(0).unwrap().to_string(), "1");
    // }
    //
    #[test]
    fn get_winner_new() {
        let mut contract = RafflesMap::default();
        contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: ("qqqq".to_string()).parse().unwrap() });

        contract.add_participant(1, "1".to_string());

        contract.cancel_raffle();

        assert_eq!(contract.get_raffle(1).cloned().unwrap().get_participants().get(0).unwrap().to_string(), "1");
        assert_eq!(contract.get_raffle(1).cloned().unwrap().get_winners().get(0).unwrap().winner_wallet_account_id.to_string(), "1");
    }
}