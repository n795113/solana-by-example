The main logic at client:
- Check the signer has enough SOL to pay the transaction
- Send and confirm the transaction

Here is a simple Typescript client.

Scaffolding:
```
client_ts/
- src/
    - main.ts
- package.json
- tsconfig.json
```

In `package.json`:
```json
{
  "name": "hellosolana",
  "version": "0.0.1",
  "description": "",
  "repository": {
    "type": "git",
    "url": "https://github.com/n795113/solana-by-example"
  },
  "keywords": [],
  "author": "",
  "license": "MIT",
  "dependencies": {
    "@solana/web3.js": "^1.33.0",
    "mz": "^2.7.0"
  },
  "devDependencies": {
    "ts-node": "^10.0.0",
    "@types/mz": "^2.7.2",
    "@tsconfig/recommended": "^1.0.1"
  },
  "engines": {
    "node": ">=14.0.0"
  }
}
```
In `tsconfig.json`:
```json
{
  "extends": "@tsconfig/recommended/tsconfig.json",
  "ts-node": {
    "compilerOptions": {
      "module": "commonjs"
    }
  },
  "compilerOptions": {
    "declaration": true,
    "moduleResolution": "node",
    "module": "es2015"
  },
  "include": ["src/client/**/*"],
  "exclude": ["node_modules"]
}
```
In `src/main.ts`:
```typescript
import fs from 'mz/fs';
import {
  Keypair,
  Connection,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';

const RPCURL = 'http://127.0.0.1:8899';

async function main() {
  const args = process.argv.slice(2);
  if (args.length != 2) {
    console.log("Please pass program and payer's keypair paths!");
    process.exit(1);
  }
  const program_keypair_path = args[0];
  const payer_keypair_path = args[1];

  console.log("Let's say hello to Solana...");

  // Establish connection to the cluster
  const connection = new Connection(RPCURL, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', RPCURL, version);

  // Get the payer to pay this transaction
  var secretKeyString = await fs.readFile(payer_keypair_path, {encoding: 'utf8'});
  var secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  // Keypair is an object that has 2 properties: publicKey, secretKey.
  // Both of them are bytes so if we want to print them as strings,
  // we can encode them into Base58 by calling toBase58() method. 
  const payerKeypair =  Keypair.fromSecretKey(secretKey);
  console.log("Payer: ", payerKeypair.publicKey.toBase58());

  // Get the program's pubkey (ID)
  secretKeyString = await fs.readFile(program_keypair_path, {encoding: 'utf8'});
  secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  const programKeypair = Keypair.fromSecretKey(secretKey);
  const programId = programKeypair.publicKey
  console.log("Program ID: ", programId.toBase58());

  // Invoke our Hello Solana Program
  console.log('Sending the transaction...');
  const instruction = new TransactionInstruction({
    // This field accepts an array of accounts
    // However, our program doesn't use accounts' data in this example
    // so just pass an empty array.
    keys: [],
    programId,
    // Our program doesn't use instruction data in this example
    // so just pass an empty byte array
    data: Buffer.alloc(0),
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payerKeypair],
  );

  console.log('Success');
}

main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
);
```


### Usage
1. Run local test validator.
    ```bash
    solana-test-validator
    ```
2. Open another terminal tab to run Solana console log where
"Hello Solana" will print at
    ```bash
    solana logs
    ```
3. Open the third terminal tab and change directory to `client_ts/` to install dependencies
    ```bash
    npm install
    ```
3. Run this rust client
    ```bash
    ts-node src/main.ts ../program/target/deploy/helloSolana-keypair.json \
    ~/.config/solana/id.json
    ```

You should see this on the second terminal:
```text
Streaming transaction logs. Confirmed commitment
Transaction executed in slot 44967:
  Signature: 8aq8hNW9iTEL5UVfE2Ttwi1v9C382yuvtoLX1h4vsvTtRzYgQxd5psSB7Cnj7qstTWqxHeyTASLA4Kf3x33QNwV
  Status: Ok
  Log Messages:
    Program 4kwL8iV4WuCJv41rxeLqhAVxnuKRrZ9PFUSeBkY78BiW invoke [1]
    Program log: Hello Solana
    Program 4kwL8iV4WuCJv41rxeLqhAVxnuKRrZ9PFUSeBkY78BiW consumed 197 of 1400000 compute units
    Program 4kwL8iV4WuCJv41rxeLqhAVxnuKRrZ9PFUSeBkY78BiW success
```

Congrat! we already finished our first progarm on Solana!