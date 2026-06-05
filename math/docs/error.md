# error.rs

In standard Web2 software engineering, if a function encounters an unexpected state, a developer might simply log an error string or allow the program to crash (panic). A crash in Web2 is an inconvenience; the server restarts, and the user refreshes the page.

In Web3 systems engineering, **a panic is an attack vector.**

If a smart contract panics unexpectedly, it consumes the user's compute units, reverts the transaction, and locks the state. Arbitrage bots will intentionally hunt for mathematical edge cases that force your contract to panic, weaponizing your own logic to stall the network or trap user funds.

Therefore, our math engine must never panic. It must gracefully identify mathematical boundaries, halt execution safely, and return a deterministic, machine-readable error state. **The use of `.unwrap()` or `.expect()` in the math crate is strictly forbidden.**

## Structuring Errors in `no_std`

Because our `math` crate is strictly `#![no_std]`, we do not have access to the standard heap allocator. This means we cannot dynamically allocate `String` objects to describe our errors (e.g., `Err("The price overflowed".to_string())`).

Instead, I defined a strict, lightweight `enum` that maps perfectly to integer codes. When we integrate with the Solana Virtual Machine in coming weeks , the Anchor framework will seamlessly translate these enum variants into official Program Errors.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    MathOverflow,
    ZeroLiquidity,
    InvalidPriceLimit,
    PriceOutOfBounds,
}
```

Every function in the math crate that performs a dangerous calculation must return a `Result<T, ErrorCode>`.

## Why These Errors Exist

Each variant in the `ErrorCode` enum is specifically designed to neutralize a known AMM attack vector or protocol failure state.

### `ErrorCode::MathOverflow`

A whale executes a trade so large that the intermediate token amounts exceed the physical 256-bit memory boundary, causing the `U256` integer to wrap back around to zero (integer overflow). If unhandled, the protocol would execute a multi-million dollar trade for zero cost.

This error is primarily thrown by `full_math.rs`. When `from_u512` detects that the upper four limbs of the 512-bit temporary warehouse are not empty, it returns `MathOverflow`, gracefully aborting the swap before state is corrupted.

### `ErrorCode::ZeroLiquidity`

A user attempts to swap tokens in a price range where $L = 0$.

Look at the token formula: $\Delta x = L \cdot (\dots)$. If $L$ is zero, the output is zero. Worse, if we attempt to calculate pricing impacts by dividing by $L$, the CPU will trigger a Divide-By-Zero panic, crashing the validator node. The engine must check $L > 0$ and return `ZeroLiquidity` if the shelf is empty.

### `ErrorCode::InvalidPriceLimit`

The Outer State Machine sends conflicting parameters to the isolated math engine.

Imagine the price is currently at Tick 0. The user is buying Token 1, which means the price should be going **up**. If the Outer Loop accidentally passes a `sqrt_price_target` that is **lower** than the current price, the math engine formulas will yield negative numbers. Because `U256` cannot be negative, it will underflow and crash. The engine must verify the directional logic and return `InvalidPriceLimit` if the target is on the wrong side of the current price.

### `ErrorCode::PriceOutOfBounds`

The AMM price exceeds the maximum or minimum mathematical limits of the Q64.96 protocol (Tick 887,272 or Tick -887,272).

If the price moves past these bounds, the binary search logarithm in `tick_math.rs` breaks down. The protocol must throw `PriceOutOfBounds` to prevent the pool from entering undefined mathematical territory.

## Execution Example

Here is how the error architecture changes the way you write code.

### Web2 Way (Banned)

```rust
pub fn get_amount_out(amount_in: U256, l: U256) -> U256 {
    // If l is 0, this panics and crashes the blockchain.
    // Standard multiplication can silently wrap/overflow.
    (amount_in * 997) / l
}
```

### The PCLMM Way

```rust
pub fn get_amount_out(
    amount_in: U256,
    l: U256
) -> Result<U256, ErrorCode> {
    // prevent divide hy zero
    if l.is_zero() {
        return Err(ErrorCode::ZeroLiquidity);
    }

    // Memory Overflow using full_math
    let amount_out = full_math::mul_div(
        amount_in,
        U256::from(997),
        l
    )?; // The '?' operator elegantly passes the error up the chain

    Ok(amount_out)
}
```

When you write the `sqrt_price_math` and `swap_math` modules, your first step is not writing the math. Your first step is writing the `if` statements that validate the inputs. Defend the boundaries first, calculate second.
