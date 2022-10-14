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

    // let outcome = contract
    //     .call("init")
    //     .args_json(json!({
    //         "beneficiary": alice.id(),
    //     }))
    //     // .deposit(deposit)
    //     .transact()
    //     .await?;


    // println!("status: {:?}", outcome);
    // let outcome = contract
    //     .call("add_new_raffle")
    //     .args_json(json!({
    //         "end_time": 42,
    //         "ticket_price": 42,
    //         "prizes": [
    //             {
    //                 "token_id": "1111",
    //                 "owner_id": alice.id(),
    //             },
    //         ],
    //     }))
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

    // let outcome = contract
    //     .call("get_counter")
    //     .transact()
    //     .await?;

    // println!("new_default_meta outcome: {:#?}", outcome);

    // let deposit = 10000000000000000000000;
    // let outcome = contract
    //     .call("nft_mint")
    //     .args_json(json!({
    //         "token_id": "0",
    //         "token_owner_id": contract.id(),
    //         "token_metadata": {
    //             "title": "Olympus Mons",
    //             "dscription": "Tallest mountain in charted solar system",
    //             "copies": 1,
    //         },
    //     }))
    //     .deposit(deposit)
    //     .transact()
    //     .await?;

    // println!("nft_mint outcome: {:#?}", outcome);

    // let result: serde_json::Value = worker
    //     .view(contract.id(), "nft_metadata", Vec::new())
    //     .await?
    //     .json()?;

    // println!("--------------\n{}", result);

    // println!("Dev Account ID: {}", contract.id());

    Ok(())
}
