# ⚡️ NFT SNAPSHOT ⚡️

This project uses rust's blazingly fast performance along with the [ethers-rs](https://github.com/gakonst/ethers-rs/) library to read blockchain state.

## Prerequisites
- You must have [rust](https://www.rust-lang.org/tools/install) installed
- You must have access to an ethereum node that has a websocket listener:
    - For infura, you must use the websocket URL that starts with wss://
    - For geth, you can run a light client that listens to the websocket port using `geth --syncmode light --http --ws`, wait for it to sync, and use `ws://localhost:8546` as the URL
 
## Quickstart
Run `cargo build && cp target/debug/nft-snapshot nft-snapshot` to install dependencies and copy the .exe to root
Run `./nft-snapshot -a <token address> -p <ethereum node websocket url> -n <final file name>` to start the snapshot.

This will automatically run through all tokens to find the owner and print it out into both the terminal and a file called `./snapshot.json` in the root directory.
