# Client (Typescript)

The only difference from the last client example is that 
we will send a non-empty array of instruction data this time.

Recap our instruction codes:
- 0: increament
- 1: decreament
- 2: set value

If the given code is 2 (set value), a user should also pass an u32 number in the other
4 bytes after the first bytes which means the total length of the array should be 5.

Here is an example how to make the array:
```javascript
var instructionData = Buffer.alloc(5, 0); // [0, 0, 0, 0, 0]

// The first byte is instruction code
instructionData.writeUint8(instructionCode, 0);

// The last 4 bytes reoresents an u32 value we want to set
instructionData.writeUInt32LE(inputValue, 1);
```
Our program will ignore the last 4 bytes if instruction code is 0 or 1 so I just alloc 5 bytes
for every cases. You can alloc only 1 byte on cases `increament`, and `decreament` of course. The 
program should works the same.

A user should pass the instruction and value if the instruction is "set value". 
Let's modify the arguments handling at the begining of the `main`.
```javascript
const args = process.argv.slice(2);
if (args.length < 3) {
    console.log("Uasge: <instruction code> [value] <program keypair path> <payer keypair path>");
    process.exit(1);
}
const instruct = args[0];
var instructionCode = 0;
var inputValue = 0;
switch (instruct) {
    case "increament": 
        instructionCode = 0;
        break;
    case "decreament":
        instructionCode = 1;
        break;
    case "set":
        instructionCode = 2;
        inputValue = parseInt(args[1]);
        break;
}
const program_keypair_path = (instruct === "set") ? args[2] : args[1];
const payer_keypair_path = (instruct === "set") ? args[3] : args[2];
```
Just a simple example without invalid arguement handling. You can do better than this one.

[Check out the full code](https://github.com/n795113/solana-by-example/tree/main/examples/set_counter/client_ts)

### Usage
Let's set the counter to some value, then play with the `increament`, and `descreament` to check if
they work properly. Finally, we set the counter again to check `set value` truely works.

```text

$ ts-node src/main.ts set 100 ../program/target/deploy/setcounter-keypair.json ~/.config/solana/id.json
Payer:  46hytJBhguswo6S8fCcVtR85HEnb9nd1hwMxFWYnSHXc
Program ID:  5RKoJE1ZwEnvGjhMG9mLgz8ARFEqHV6JfectxedmM7Rg
Sending the transaction...
Address: H1JS1dZ4do8t71XUs5g3eVjcNYLdgZhzFqiHASSPC5CE counter: 100
Success
$ ts-node src/main.ts increament ../program/target/deploy/setcounter-keypair.json ~/.config/solana/id.json
Payer:  46hytJBhguswo6S8fCcVtR85HEnb9nd1hwMxFWYnSHXc
Program ID:  5RKoJE1ZwEnvGjhMG9mLgz8ARFEqHV6JfectxedmM7Rg
Sending the transaction...
Address: H1JS1dZ4do8t71XUs5g3eVjcNYLdgZhzFqiHASSPC5CE counter: 101
Success
$ ts-node src/main.ts decreament ../program/target/deploy/setcounter-keypair.json ~/.config/solana/id.json
Payer:  46hytJBhguswo6S8fCcVtR85HEnb9nd1hwMxFWYnSHXc
Program ID:  5RKoJE1ZwEnvGjhMG9mLgz8ARFEqHV6JfectxedmM7Rg
Sending the transaction...
Address: H1JS1dZ4do8t71XUs5g3eVjcNYLdgZhzFqiHASSPC5CE counter: 100
Success
$ ts-node src/main.ts set 1234 ../program/target/deploy/setcounter-keypair.json ~/.config/solana/id.json  
Payer:  46hytJBhguswo6S8fCcVtR85HEnb9nd1hwMxFWYnSHXc
Program ID:  5RKoJE1ZwEnvGjhMG9mLgz8ARFEqHV6JfectxedmM7Rg
Sending the transaction...
Address: H1JS1dZ4do8t71XUs5g3eVjcNYLdgZhzFqiHASSPC5CE counter: 1234
Success
```