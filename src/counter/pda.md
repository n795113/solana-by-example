# Program Derive Address (PDA)

There 2 properties of an account:
- owner: who can modify the account
- authority: who can sign the account

PDAs are just specific accounts that have no private keys their authorities are belongs to programs.

Here is a good image from [Solana Cookbook](https://solanacookbook.com/core-concepts/pdas.html#facts) to show the difference between **ownership** and **authority**:
![](https://solanacookbook.com/assets/account-matrix.c3a79f80.png)

In this example. We will use a PDA to store the counter value!

### Why PDA

PDAs serve as the foundation for [Cross-Program Invocation](https://docs.solana.com/developing/programming-model/calling-between-programs#cross-program-invocations), which allows Solana apps to be composable with one another.

### Rent

Accounts need to be PAID to live on the chain. 
The fee is depends on how much data an account holds, and how long it will live.
We will see how to count the fee in this example.

[More detail about rent from official doc](https://docs.solana.com/implemented-proposals/rent)