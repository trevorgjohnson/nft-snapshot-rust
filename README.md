# ⚡️ NFT SNAPSHOT ⚡️

This project uses rust's blazingly fast performance along with the [ethers-rs](https://github.com/gakonst/ethers-rs/) library to read blockchain state.

## Setup

You will have to replace 2 variables within the contract (assuming the contract is following the ERC721 standard):
 - `"<insert_contract_address_here>"` on line 17 with the desired token contract's address 
 - `"https://mainnet.infura.io/v3/<insert_infura_api_key_here>"` on line 20 to your own provider
 
## Quickstart

(This assumes you've already cloned the repo and have [rust](https://www.rust-lang.org/tools/install) installed)

After replacing the 2 variables stated above, go ahead and run `cargo run`. This will start the snapshot and will automatically run through 
all tokens to find the owner and print it out into a file called `/snapshot.json` in the root directory.
