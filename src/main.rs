#![feature(async_closure)]

use ethers::{contract::abigen, prelude::*};
use eyre::Result;
use std::{fs::File, sync::Arc};
use std::io::prelude::*;
use std::env;
use futures;

abigen!(
    NFTContract,
    r#"[
        function ownerOf(uint256 tokenId) external view returns (address)
        function totalSupply() external view returns (uint256)
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<()> {
    let arguments = env::args().collect::<Vec<String>>();
    let contract_address = arguments[1].parse::<Address>()?;
    let provider_url = arguments[2].parse::<String>()?;

    let client = Arc::new(Provider::<Ws>::connect(provider_url).await?);
    let contract = Arc::new(NFTContract::new(contract_address, client));

    let total_supply = contract.total_supply().call().await?.as_u64();

    let results: Vec<Result<String>> = futures::stream::iter(1..total_supply)
        .map(|token| {
            let inner_contract = contract.clone();
            async move {
                let owner = inner_contract.owner_of(token.into()).call().await?;
                let owner_string = ethers::utils::to_checksum(&owner, None);

                let result = format!("  \"{}\": \"{}\"", token.to_string(), owner_string);
                println!("{result}");

                Ok(result)
            }
        })
        .buffered(100)
        .collect()
        .await;

    let results = results.into_iter()
        .map(|result| result.unwrap())
        .collect::<Vec<String>>()
        .join(",\n");

    let mut output = File::create("snapshot.json")?;
    writeln!(output, "{{\n{results}\n}}")?;

    Ok(())
}
