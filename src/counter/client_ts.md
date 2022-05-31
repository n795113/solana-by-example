# Client (Typescript)

First, let's generate an address for our state account. This address is special compared to other anther accounts
since it is a hash instead of a public key.
```javascript
const accountSeed = 'solana-by-example'; // can be any string

const accountAddress = await PublicKey.createWithSeed(
    payerKeypair.publicKey,
    accountSeed,
    programId,
); // The result looks like a public key but is just a hash
```

Second, check if this accounts has already created on chain. If not, then send a transaction to System Program to create an account using the address we generated.

```javascript
var account = await connection.getAccountInfo(accountAddress);
if (account === null) {

    // The account only holds an u32 number so the size is 4 bytes
    const constAccountSize = 4

    // Count how much rent should be paid for this account
    const rentFee = await connection.getMinimumBalanceForRentExemption(constAccountSize);

    const transaction = new Transaction().add(
        SystemProgram.createAccountWithSeed({
        fromPubkey: payerKeypair.publicKey,
        basePubkey: payerKeypair.publicKey,
        seed: accountSeed, // "solana-by-example"
        newAccountPubkey: accountAddress,
        lamports: rentFee,
        space: constAccountSize,
        programId,
        }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payerKeypair]);
}
```

Third, send another transaction to our Counter Program by passing the account we get at step 2. The Counter Program
then should modify the data of the account.

```javascript
const stateAccount = {
    pubkey: accountAddress,
    isWritable: true,
    isSigner: false
}

const instruction = new TransactionInstruction({
    keys: [stateAccount],
    programId,
    data: Buffer.alloc(0), // instruction data won't be used by this example
});

await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payerKeypair],
);
```

Finally, query the account again to check if the data is updated.

```javascript
account = await connection.getAccountInfo(accountAddress);

if (account === null) {
    throw 'Error: cannot find the greeted account';
}

const StateAccountSchema = new Map([
    [StateAccount, {kind: 'struct', fields: [['counter', 'u32']]}]
]);

const state = borsh.deserialize(
    StateAccountSchema, 
    StateAccount, // State class
    account.data
);

console.log(
    'Address:'
    accountAddress.toBase58(),
    'counter:',
    state.counter
);
```
add `State` class outside the `main` function. It is similar to the struct we did in `program.rs`:
```javascript
class StateAccount {
    counter = 0;
    constructor(fields: {counter: number} | undefined = undefined) {
        if (fields) {
            this.counter = fields.counter;
        }
    }
}
```

[Check out the full code](https://github.com/n795113/solana-by-example/tree/main/examples/counter/client_ts)

### Usage

```shell
ts-node src/main.ts ../program/target/deploy/counter-keypair.json \
~/.config/solana/id.json
```

The counter should increases 1 every time this client runs.

This is the result that run the client 2 times:
```text
$ ts-node src/main.ts ../program/target/deploy/counter-keypair.json \
~/.config/solana/id.json

Payer:  46hytJBhguswo6S8fCcVtR85HEnb9nd1hwMxFWYnSHXc
Program ID:  GuoMVjGXrxDvJaKuRfuKZsiwSHrfvfXg2YH7DTxGqdQe
Creating account: CzZRqZHR4ZEcoyJs61WRFJZP2iX2siPHwVVMGwu3iFdt
Sending the transaction to Counter Program...
Address: CzZRqZHR4ZEcoyJs61WRFJZP2iX2siPHwVVMGwu3iFdt counter: 1
Success
$ ts-node src/main.ts ../program/target/deploy/counter-keypair.json \
~/.config/solana/id.json

Payer:  46hytJBhguswo6S8fCcVtR85HEnb9nd1hwMxFWYnSHXc
Program ID:  GuoMVjGXrxDvJaKuRfuKZsiwSHrfvfXg2YH7DTxGqdQe
Sending the transaction to Counter Program...
Address: CzZRqZHR4ZEcoyJs61WRFJZP2iX2siPHwVVMGwu3iFdt counter: 2
Success
```

We can see at the first time, the client create a new account, and its counter is 1.
At the second time, since the account is already existed, the client doesn't create an account again. 
It just take the existed account, and add its counter to 2.

### Activity

What will happen if the seed changes every time? (now it is hard coded as "solana-by-example")