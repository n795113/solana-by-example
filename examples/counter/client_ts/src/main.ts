import fs from 'mz/fs';
import {
  Keypair,
  Connection,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
  SystemProgram
} from '@solana/web3.js';
import * as borsh from 'borsh';

const RPCURL = 'http://127.0.0.1:8899';

class StateAccount {
  counter = 0;
  constructor(fields: {counter: number} | undefined = undefined) {
    if (fields) {
      this.counter = fields.counter;
    }
  }
}

async function main() {
  const args = process.argv.slice(2);
  if (args.length !== 2) {
    console.log("Uasge: <program keypair path> <payer keypair path>");
    process.exit(1);
  }
  const program_keypair_path = args[0];
  const payer_keypair_path = args[1];

  // Establish connection to the cluster
  const connection = new Connection(RPCURL, 'confirmed');
  const version = await connection.getVersion();

  // Get the payer to pay this transaction
  var secretKeyString = await fs.readFile(payer_keypair_path, {encoding: 'utf8'});
  var secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  const payerKeypair =  Keypair.fromSecretKey(secretKey);
  console.log("Payer: ", payerKeypair.publicKey.toBase58());

  // Get the program's pubkey (ID)
  secretKeyString = await fs.readFile(program_keypair_path, {encoding: 'utf8'});
  secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  const programKeypair = Keypair.fromSecretKey(secretKey);
  const programId = programKeypair.publicKey
  console.log("Program ID: ", programId.toBase58());

  // Create an address
  const accountSeed = 'solana-by-example'; // can be any string

  const accountAddress = await PublicKey.createWithSeed(
      payerKeypair.publicKey,
      accountSeed,
      programId,
  ); // The result looks like a public key but is just a hash

  var account = await connection.getAccountInfo(accountAddress);
  if (account === null) {

    // The account only holds an u32 number so the size is 4 bytes
    const constAccountSize = 4

    // Count how much rent should be paid for this account
    const rentFee = await connection.getMinimumBalanceForRentExemption(constAccountSize);

    console.log("Creating account:", accountAddress.toBase58());
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

  // Invoke our Counter Program
  console.log('Sending the transaction to Counter Program...');

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

  // Query the state account
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
    'Address:',
    accountAddress.toBase58(),
    'counter:',
    state.counter
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