# Explain

### Client & Program

"Program" is "Smart Contract" in Solana. It just terminology difference.

If you are a web3 newbie, you may take programs (smart contract) as AWS Lambda functions.

Just like client-server model that we are familiar in web2, 
we develop and deploy prgrams online then use clients to interact
with them. it can be any kind of client, such as a typescript client, or a Rust client.

### Account

Accounts are just buffers that store serialized binary on chain. Every data on Solana are stored in accounts.

### Program

Program are not special, they are also data, stored in accounts. These accounts are 
flagged as executable and ownership is transferred to an ebpf loader program.

Programs on Solana are stateless which means we need to pass states from other places
which you may already guess: other accounts.

### States

Oppisite to programs, states are stored in non-executable accounts which can be 2 types:
- writable
- read-only

You may think state accounts as rows in database. We will introduce how to
modify and query them in a later example.

### Resources
[ok so what the fuck is the deal with solana anyway](https://2501babe.github.io/posts/solana101.html)