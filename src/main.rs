use ethers::{abi::AbiEncode, contract::abigen, prelude::*};
use eyre::Result;
use std::io::prelude::*;
use std::{convert::TryFrom, fs::File, sync::Arc};

abigen!(
    EzuContract,
    r#"[
        function ownerOf(uint256 tokenId) external view returns (address)
        function totalSupply() external view returns (uint256)
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<()> {
    let contract_address = "<insert_contract_address_here>".parse::<Address>()?;

    let client =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/<insert_infura_api_key_here>")
            .unwrap();
    let client = Arc::new(client);

    let contract = EzuContract::new(contract_address, client.clone());

    let total_supply = contract.total_supply().call().await?;
    let total_supply: i32 = total_supply.to_string().parse().unwrap();

    let mut output = File::create("snapshot.json")?;

    writeln!(output, "{{")?;

    for i in 1..total_supply {
        println!("{i}");
        let owner: Address = contract.owner_of(i.into()).call().await?;
        let owner = owner.encode_hex();
        let address_no_prefix = &owner[26..];
        let full_address = format!("'0x{}',", address_no_prefix);
        writeln!(output, "{}", full_address)?;
    }

    writeln!(output, "}}")?;

    Ok(())
}
