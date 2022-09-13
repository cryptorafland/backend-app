use std::collections::{LinkedList, HashMap};
use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, log, near_bindgen};
use near_sdk::collections::{LazyOption, TreeMap, UnorderedMap, Vector};
use serde::{Serialize, Deserialize};

// use rand::seq::SliceRandom;
// use rand::thread_rng;
// use array_linked_list::ArrayLinkedList;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct RafflesMap {
    raffles: HashMap<Counter, Raffle>,
    counter: Counter,
}

impl Default for RafflesMap {
    fn default() -> RafflesMap {
        RafflesMap {
            raffles: Default::default(),
            counter: Counter{
                value: 0,
            },
        }
    }
}

#[near_bindgen]
impl RafflesMap {
    pub fn get_raffles(&self) -> &HashMap<Counter, Raffle> {
        &self.raffles
    }

    pub fn get_counter(&self) -> &Counter {
        &self.counter
    }

    pub fn get_counter_value(&self) -> u128 {
        self.counter.value
    }

    pub fn get_raffle(self, game_id: Counter) -> Option<Raffle> {
        self.get_raffles().get(&game_id).cloned()
    }

    pub fn add_new_raffle(&mut self, start_time: u32, end_time: u32, prizes: Vec<NFT>,
                          ticket_price: u32, amount_of_winners: u32) {
        let participants_in_this_game: Vec<String> =  Vec::new();
        let winners_in_this_game: Vec<Winner> =  Vec::new();

        let new_raffle: Raffle = Raffle {
            start_time,
            end_time,
            prizes,
            ticket_price,
            amount_of_winners,
            creator_wallet_account_id: near_sdk::env::predecessor_account_id().to_string(),
            winners: winners_in_this_game,
            participants: participants_in_this_game
        };

        let counter = self.get_counter().clone();
        self.raffles.insert(counter, new_raffle);
        self.counter.increment();
    }

    // pub fn add_new_participate(&mut self, game_id: Counter) {
    //     self.get_raffle(game_id)..push_back(near_sdk::env::predecessor_account_id().to_string());
    // }

    // fn finish_game(&self, game_id: string) {
    //     let winner: string = get_winner(game_id);
    //     send_prise(winner);
    // }

    // fn get_winner(&self, game_id: u128) -> string{
    //     // here we shuffle a list of participant
    //     self.get_raffle(game_id).shuffle(self.get_raffle(game_id).get_participants())
    //     // here we take random number n (or not only one element)
    //
    //     // here we take a element with position n from shuffled list of participant (or not only one element)
    //
    //     // here we write winner (winners) in raffle.winners
    // }

    // fn send_prise(winner_id: string) {
    //     let mut contract = Contract::new_default_meta(winner_id);
    //     contract.nft_transfer(winner_id, token_id.clone(), None, None);
    // }

    // fn shuffle(&mut participants: LinkedList<NonFungibleToken>)  -> LinkedList<NonFungibleToken>{
    //     let mut x = ArrayLinkedList::new();
    //     x = participants;
    //     let mut rng = thread_rng();
    //     x.shuffle(&mut rng);
    //     participants = x;
    //     participants
    // }
}

#[near_bindgen]
#[derive(PartialOrd, Eq, PartialEq, BorshDeserialize, BorshSerialize, Default, Hash, Serialize, Clone, Deserialize)]
pub struct Counter {
    value: u128,
}

#[near_bindgen]
impl Counter {
    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn get_count(&self) -> u128 {
        self.value
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Default)]
pub struct Raffle {
    start_time: u32,
    end_time: u32,
    prizes: Vec<NFT>,
    ticket_price: u32,
    amount_of_winners: u32,

    creator_wallet_account_id: String,

    winners: Vec<Winner>,
    participants: Vec<String>
}

#[near_bindgen]
impl Raffle {
    pub fn new(&mut self, start_time: u32, end_time:u32, prizes:Vec<NFT>,
               ticket_price:u32, amount_of_winners:u32) -> Self {
        log!("New raffles initialization!");
        Self { start_time, end_time, prizes,
            ticket_price, amount_of_winners, winners: Default::default(),
            creator_wallet_account_id: near_sdk::env::predecessor_account_id().to_string(), participants: Default::default()
        }
    }

    pub fn get_participants(self) -> Vec<String> {
        self.participants
    }

    pub fn add_participant(mut self, id: String) {
        self.participants.insert(0, id);
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize)]
pub struct Winner {
    winner_wallet_account_id: String,
    prize: NFT,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
pub struct NFT {
    account_name: String,
    id: String,
}
