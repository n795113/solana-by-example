# Init Project

First, let's create a new Anchor project:
```shell
anchor init set_counter_anchor
```

`cd` the project then run the test:
```shell
cd set_counter_anchor
anchor test
```

This command will build, deploy the program, then run the test code!
Pretty handy, right?

You should see something like below which means all works.
```text
  set_counter_anchor
Your transaction signature 38P7FC6rNP95RGwQRmYrGZYpsS22KgA6KLk9tMzhuD6U3z324J2xSjiScLtTrSqkNCpt9D5MaCWKrJEBVdydcz2F
    ✔ Is initialized! (322ms)


  1 passing (330ms)

✨  Done in 7.38s.
```
### Program

Let's take a look to `programs/set_counter_anchor/src/lib.rs`.
Here is where the program logic lives.

```rust, ignore
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod new_anchor_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
```

`Context` is mapped to the accounts we pass to a program.

`initialize()` is an instruction of this program. We can have several instructions in one program, 
just like we do in Set Counter example. Different instruction may require different accounts to be passed to it 
so we can define the schema of the accounts using a struct with `#[derive(Accounts)]`.


### Test

Another reason to use Anchor is it set up the test stuffs for you.
In `tests/set_counter_anchor.ts`, we can see the example test.