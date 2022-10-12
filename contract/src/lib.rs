use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_contract_standards::non_fungible_token::{TokenId};
use near_sdk::{AccountId, env, log, near_bindgen, Balance, Promise};
use rand::seq::SliceRandom;
use rand::thread_rng;
use near_sdk::serde::{Deserialize, Serialize};


/**
  * now only 1 winner
  * now only 1 prize
  * now start only now
  */

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
const DEFAULT_COUNTER: u128 = 0;
const DEFAULT_MESSAGE: &str = "Hello";


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RafflesMap {
    raffles: HashMap<u128, Raffle>,
    counter: Counter,
    pub beneficiary: AccountId,
    greeting: String,
}

impl Default for RafflesMap{
    fn default() -> Self {
        Self {
            beneficiary: "v1.faucet.nonofficial.testnet".parse().unwrap(),
            counter: Counter{ value: DEFAULT_COUNTER },
            raffles: HashMap::new(),
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
            raffles: HashMap::new(),
            greeting: DEFAULT_MESSAGE.to_string(),
        }
    }

    pub fn get_greeting(&self) -> String {
        return self.greeting.clone();
    }

    pub fn get_counter(&self) -> u128 {
        return self.counter.value.clone()
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

    #[payable]
    fn add_participant(&mut self, key: u128, sender: AccountId) {

        // let sender: AccountId = env::predecessor_account_id();
        let pays: Balance = env::attached_deposit();

        let ticket_price: u128 = self.raffles.get_mut(&key).unwrap().get_ticket_price();

        let to_transfer: Balance = if pays >= ticket_price {
            // Subtract the storage cost to the amount to transfer
            ticket_price - STORAGE_COST 
          }else{
            0
        };

        if to_transfer > 0 {
            // Transfer the amount to the beneficiary
            Promise::new(self.beneficiary.clone()).transfer(to_transfer);
            let return_back: Balance = pays - ticket_price;
            Promise::new(sender.clone()).transfer(return_back);

            self.raffles.get_mut(&key).unwrap().add_participant(sender);
        }

    }

    fn set_counter(&mut self, counter: u128) {
        log!("Saving counter {}", counter);
        self.counter.value = counter;
    }

    pub fn start_new_raffle(&mut self,end_time: u32, ticket_price: u128, prize: JsonToken) {
        self.add_new_raffle(end_time, ticket_price, prize);

        // wait some time
        // IN THIS TIME WE NEED TO ADD PARTICIPANT
        // thread::spawn(|| {
        //     sleep(Duration::from_secs(5));
        // });

        self.cancel_raffle();
    }

    fn add_new_raffle(&mut self, end_time: u32, ticket_price: u128, prize: JsonToken) {
        // create structure for this game
        let participants_in_this_game: Vec<AccountId> =  Vec::new();
        let winners_in_this_game: Vec<Winner> =  Vec::new();
        let mut prizes_in_this_game: Vec<JsonToken> =  Vec::new();

        prizes_in_this_game.push(prize);

        // increment counter
        let mut counter = self.get_counter();
        counter = counter + 1;
        self.set_counter(counter);

        // take creator id
        let creator: AccountId = env::predecessor_account_id();

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
            let winner: AccountId = self.get_mut_raffle(self.get_counter()).unwrap().get_random_winner().clone();

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
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Raffle {
    end_time: u32,
    prizes: Vec<JsonToken>,
    ticket_price: u128,
    creator_wallet_account_id: AccountId,
    game_continues: bool,
    winners: Vec<Winner>,
    participants: Vec<AccountId>,
}

#[near_bindgen]
impl Raffle {
    fn add_participant(&mut self, id: AccountId) {
        self.participants.push(id);
    }

    fn shuffle_participant(&mut self) {
        self.participants.shuffle(&mut thread_rng());
    }

    fn get_ticket_price(&self) -> u128 {
        return self.ticket_price;
    }

    fn get_creator(&self) -> &AccountId {
        return &self.creator_wallet_account_id
    }

    fn get_participants(&self) -> &Vec<AccountId> {
        return &self.participants
    }

    fn get_winners(&self) -> &Vec<Winner> {
        return &self.winners
    }

    fn get_prize(&self) -> &Vec<JsonToken> {
        return &self.prizes
    }

    fn get_random_winner(&self) -> &AccountId {
        self.get_participants().choose(&mut thread_rng()).unwrap()
    }

    fn add_winner(&mut self, winner: AccountId) {
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
    pub token_id: TokenId,
    pub owner_id: AccountId,
}



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Winner {
    winner_wallet_account_id: AccountId,
    prize: JsonToken,
}

#[near_bindgen]
impl Winner {
    // fn send_prize_to_winner(&mut self) {
    //     self.prize.nft_transfer(self.winner_wallet_account_id.clone(), self.prize.token_id.clone());
    // }
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
        let contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        assert_eq!(contract.get_counter(), 0);
    }

    #[test]
    fn set_then_get_counter() {
        let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        assert_eq!(contract.get_counter(), 0);
        contract.set_counter(1);
        assert_eq!(contract.get_counter(), 1);
    }

    #[test]
    fn counter_after_creation_raffle() {
        let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        assert_eq!(contract.get_counter(), 0);
        contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: env::predecessor_account_id() });
        assert_eq!(contract.get_counter(), 1);
    }

    #[test]
    fn get_creator() {
        let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: env::predecessor_account_id() });
        let raffle = contract.raffles.get(&1u128);
        
        let d = raffle.cloned();
        assert_eq!(d.unwrap().get_creator().to_string(), env::predecessor_account_id().to_string());
    }

    // #[test]
    // fn get_raffle() {
    //     let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap()); // RafflesMap::default()
    //     contract.start_new_raffle(1, 1, JsonToken { token_id: "1111".to_string(), owner_id: env::predecessor_account_id() });
    //     assert_eq!(contract.get_raffle(1).cloned().unwrap().get_creator().to_string(), "bob.near");
    // }

    #[test]
    fn add_participant() {
        let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        contract.start_new_raffle(1, 10*NEAR, JsonToken { token_id: "1111".to_string(), owner_id: env::predecessor_account_id() });

        set_context("donor_a", 1000*NEAR);

        contract.add_participant(1, "donor_a".parse().unwrap());
        assert_eq!(contract.get_raffle(1).cloned().unwrap().get_participants().get(0).unwrap().to_string(), "donor_a");
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
        let mut contract = RafflesMap::init(BENEFICIARY.parse().unwrap());
        contract.start_new_raffle(1, 1*NEAR, JsonToken {token_id: "1111".to_string(), owner_id: env::predecessor_account_id()});

        set_context("bob.near", 10*NEAR);

        // assert_eq!(contract.get_raffle(1).cloned().unwrap().get_ticket_price(), 1*NEAR);

        contract.add_participant(1, "bob.near".parse().unwrap());
        contract.cancel_raffle();

        assert_eq!(contract.get_raffle(1).cloned().unwrap().get_participants().get(0).unwrap().to_string(), "bob.near");
        assert_eq!(contract.get_raffle(1).cloned().unwrap().get_winners().get(0).unwrap().winner_wallet_account_id.to_string(), "bob.near");
    }


    // Auxiliar fn: create a mock context
    fn set_context(predecessor: &str, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }


}