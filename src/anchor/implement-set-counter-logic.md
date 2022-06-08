# Implement Set Counter Logic

### Define State Account
In the previous example, we create the state account at the client side. 
However, in this implementation, we will move this process to the program 
so we add anothor instrucion called `initialize`. The responsibility of this
instruction is to create a state account and set its counter to default, which is 0.

Let's define the state account schema.
```rust, ignore
// This is how we define an account with Anchor
#[account]
pub struct State {
    counter: u32
}
```
You can find the only difference from the vanilla Rust code is the attribute

```rust, ignore
// Original way in vanilla Rust
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct State {
    counter: u32
}
```

### Define Instructions & Argument Accounts

Use `#[derive(accounts)]` attribute to define agrument accounts. By doing so, we define 
what accounts should we pass to an instruction. Then at each instruction, we define its argument accounts 
by wrapping the struct with `Context`.

There are two types of accounts in this example. For the instructions, setting value, increment, and decrement, 
we only need to pass the state account so they share one struct called `Update`. On the other hand, For 
initialization, we also need to pass the payer and the system program ID to out program so we define another
struct called `Initialize`. Let's see the code.

```rust, ignore
#[program]
pub mod set_counter_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u32) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub state: Account<'info, State>
}
```

### Fill the logic

The struct is done. The rest task is to fill the logic:
```rust, ignore
#[program]
pub mod set_counter_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.counter = 0; // init the counter
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.counter = state.counter.checked_sub(1).unwrap_or(0); // avoid overflow
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.counter = state.counter.checked_add(1).unwrap_or(u32::MAX); // avoid overflow
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u32) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.counter = value;
        Ok(())
    }
}
```