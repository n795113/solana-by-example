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
    keys: [], // Accounts
    programId,
    data: Buffer.alloc(0), // HelloSolana doesn't use instruction data so pass an empty byte array
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