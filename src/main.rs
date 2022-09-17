// #![feature(async_closure)]

use clap::Parser;
use ethers::{contract::abigen, prelude::*};
use eyre::Result;
use futures;
use std::{fs::File, io::prelude::*, sync::Arc};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    address: Address,
    #[clap(short, long, value_parser)]
    provider: String,
    #[clap(short, long, value_parser)]
    name: String,
    #[clap(short, long, action)]
    object: bool,
}

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
    let arguments = Args::parse();
    let contract_address = arguments.address;
    let provider_url = arguments.provider;

    let client = Arc::new(Provider::<Ws>::connect(provider_url).await?);
    let contract = Arc::new(NFTContract::new(contract_address, client));

    let total_supply = contract.total_supply().call().await?.as_u64();

    let results: Vec<Result<String>> = futures::stream::iter(1..10)
        .map(move |token| {
            let inner_contract = contract.clone();
            async move {
                let owner = inner_contract.owner_of(token.into()).call().await?;
                let owner_string = ethers::utils::to_checksum(&owner, None);

                let result = if arguments.object {
                    format!("   \"{}\": \"{}\"", token.to_string(), owner_string)
                } else {
                    format!("   \"{}\"", owner_string)
                };

                println!("{result}");

                Ok(result)
            }
        })
        .buffered(100)
        .collect()
        .await;

    let results = results
        .into_iter()
        .map(|result| result.unwrap())
        .collect::<Vec<String>>()
        .join(",\n");

    let filename = format!("{}.json", arguments.name);
    let mut output = File::create(filename)?;
    if arguments.object {
        writeln!(output, "{{\n{results}\n}}")?;
    } else {
        writeln!(output, "[\n{results}\n]")?;
    }

    Ok(())
}
