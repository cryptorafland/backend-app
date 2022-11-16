use serde_json::json;
use workspaces::{Account, AccountId, Contract, Worker};
// use near_sdk::collections::{Vector};
// use near_contract_standards::non_fungible_token::TokenId;



// const NFT_RAFFLAND: &str = "/home/hello-near-rust/contract/target/wasm32-unknown-unknown/release/contract.wasm";
const NFT_RAFFLAND: &str = "/home/raffland/backend-app/contract/target/wasm32-unknown-unknown/release/contract.wasm";


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(NFT_RAFFLAND)?;
    let contract = worker.dev_deploy(&wasm).await?;
    // let deposit = 10000000000000000000000;
    let alice = worker.dev_create_account().await?;

    //create vector and add token to it

    let outcome = contract
        .call("init")
        .args_json(json!({
            "beneficiary": alice.id(),
        }))
        // .deposit(deposit)
        .transact()
        .await?;


    println!("status: {:?}", outcome);
    let outcome = contract
        .call("add_new_raffle")
        .args_json(json!({
            "end_time": 42,
            "ticket_price": 42,
            "prizes": [
                {
                    "token_id": "1111",
                    "owner_id": alice.id(),
                },
            ],
        }))
        .transact()
        .await?;

    
    println!("status: {:?}", outcome);
    //     // .deposit(deposit)
    //     .transact()
    //     .await?;

    // let outcome = contract
    //     .call("init")
    //     .args_json(json!({
    //         "beneficiary": alice.id(),
    //     }))
    //     .deposit(deposit)
    //     .transact()
    //     .await?;

    // contract.get_greeting()
    let result: u128 = contract
        .call("get_counter1")
        .view()
        .await?
        .json()?;

    println!("status: {:?}", result);

    contract
        .call("increment_counter")
        .transact()
        .await?;

    let result: u128 = contract
        .call("get_counter1")
        .view()
        .await?
        .json()?;

    println!("status: {:?}", result);

//  
//   @@@@@@@ @@@@@@@@  @@@@@@ @@@@@@@  @@@@@@       @@@@@@  @@@@@@@ @@@@@@@@ @@@  @@@  @@@@@@  @@@@@@@  @@@  @@@@@@   @@@@@@     
//     @@!   @@!      !@@       @@!   !@@          !@@     !@@      @@!      @@!@!@@@ @@!  @@@ @@!  @@@ @@! @@!  @@@ !@@         
//     @!!   @!!!:!    !@@!!    @!!    !@@!!        !@@!!  !@!      @!!!:!   @!@@!!@! @!@!@!@! @!@!!@!  !!@ @!@  !@!  !@@!!      
//     !!:   !!:          !:!   !!:       !:!          !:! :!!      !!:      !!:  !!! !!:  !!! !!: :!!  !!: !!:  !!!     !:!     
//      :    : :: ::: ::.: :     :    ::.: :       ::.: :   :: :: : : :: ::: ::    :   :   : :  :   : : :    : :. :  ::.: :      
                    

//0 test return for get_participants & get_winners from outside!

// * 1) create raffle with 1 prizes, add N users, performe drawls, check if winner got their prizes
// * 1.1) raffle can't add participants after certain time close to drawl
// * 1.2) check how big M and N can be
// * 1.3) check if raffle can be created with 0 prizes
// * 1.4) check if raffle can be created with 0 participants
// * 1.5) check case M > N


// * 2) check, that RNG is really random
// * 3) (optionly) check creation of several raffles (from 1)
// * 4) check that get_participants, get_winners, add_participant, am_I_winner/am_I_participant
// * return correct and reasonable values
// * 5) check correctness of gas usage and requirements 
// * 6) check correctness of storage usage 
// * 7) check NFT minting and transfering
// * 8) check that everything above correctly works with NFTs



    Ok(())
}
