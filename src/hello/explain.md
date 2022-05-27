# Explain

### Client & Program

"Program" is "Smart Contract" in Solana. It just terminology difference.

If you are a web3 newbie, you may take program (smart contract) as AWS Lambda.

Just like client-server model that we are familiar in web2, 
we develop and deploy prgrams then use clients to interact
with them. it can be a typescript client, Python client, or CLI client.

### Account

Accounts are just buffers that store data on chain. They hold serialized binary data. 

### Program

Program are not special, they are also data, stored in accounts. These accounts are 
flagged as executable and ownership is transferred to an ebpf loader program.

Programs on Solana are stateless which means we need to pass states from other places
which you may already guess: other accounts. So there are executable accounts and read-only
accounts. we run the executable accounts by passing read-only accounts which contain data
we need.

Maybe you can think read-only accounts as rows in database. We will introduce how to
query them in a later example.

### Resources
[ok so what the fuck is the deal with solana anyway](https://2501babe.github.io/posts/solana101.html)