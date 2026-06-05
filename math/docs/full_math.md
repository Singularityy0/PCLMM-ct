# fULL MATH.rs

## The Memory Ceiling (The `full_math` Crate)

In smart contract math, the most common operation is calculating a fraction:

$$ \text{Result} = \frac{A \times B}{C} $$

Let us pretend our computer's absolute maximum memory capacity is the number **100**. Any number higher than 100 causes the CPU to panic. We need to calculate:

$$ \text{Result} = \frac{60 \times 80}{120} $$

On paper, the answer is **40**. The number 40 fits perfectly inside our 100-limit container.

But when the CPU runs the code, it executes $60 \times 80$ first. The intermediate result is **4800**. Because $4800 > 100$, the compiler panics, the transaction fails, and the user's trade is rejected even though the final answer ($40$) was completely valid.

### The 256-bit Reality

In the PCLMM protocol, our maximum capacity is a 256-bit integer (`U256`). When we multiply two Q64.96 token amounts, the intermediate fraction expands to 192 bits. If the whole numbers are large, multiplying $A \times B$ will frequently spike to nearly 512 bits.

**The Rule:** You are strictly forbidden from using the standard `*` and `/` operators for fractions in the math crate. You must use the `full_math` module to protect the memory boundary.

### The Protocol Defensive Invariants

Inside `full_math.rs`, we utilize two distinct functions depending on who is receiving money. In DeFi, the protocol must never be generous.

> **`mul_div_rounding_up` (User Pays Us)**
>
> $$ \lceil \frac{A \times B}{C} \rceil $$
>
> When calculating the **Input Amount** the user must pay the pool, we round UP. If the math says the user owes us 10.0001 tokens, we charge them exactly 11 tks. We always charge slightly more.

> **`mul_div` (We Pay the User)**
>
> $$ \lfloor \frac{A \times B}{C} \rfloor $$
>
> When calculating the **Output Amount** we owe the user, we round DOWN. If we owe them 10.9999 tokens, we give them exactly 10 tks.

This microscopic spread ensures the liquidity pool is mathematically solvent against rounding dust and arbitrageurs.

## The 512-bit Architecture (`to_u512` & `from_u512`)

How does `full_math.rs` actually calculate the intermediate overflow without crashing? It uses a "Temporary Warehouse" in memory.

### Understanding Limbs

A standard 64-bit CPU cannot process 256 bits at once. The `ruint` library breaks a `U256` into an array of four 64-bit chunks, called **Limbs**.

$$ \texttt{U256} = [\texttt{u64; 4}] $$

A `U512` is simply a container that is twice as large, possessing eight limbs.

$$ \texttt{U512} = [\texttt{u64; 8}] $$

### Casting Up (`to_u512`)

When we begin our calculation, $A$ and $B$ are 4-limb `U256` numbers. We run them through `to_u512`.

This function takes the four limbs and appends four completely empty limbs (zeros) to the top. It does not change the value; it simply puts the number inside a much larger box. We can now safely multiply $A \times B$ inside the massive 8-limb container. The data expands into the upper limbs without overflowing.

While the massive intermediate result is sitting in the 512-bit container, we perform our division by $C$. Division inherently shrinks the number.

### Casting Down and The Security Boundary (`from_u512`)

Once the math is done, the answer is still in the 8-limb `U512` container. The outer state machine does not know how to read 512 bits. We must run it through `from_u512`.

This is not just a type cast; **it is a cryptographic boundary check.**

The function inspects the top four limbs of the `U512` container:

- **If the top four limbs are exactly 0:** The division successfully shrank the number. The function drops the top four limbs, grabs the bottom four, and safely returns a standard `U256`.
- **If the top four limbs have ANY data in them (not 0):** The final answer is physically too large to fit in 256 bits. The math has fundamentally overflowed the absolute bounds of the protocol.

If an overflow is detected, the function immediately halts execution and returns `ErrorCode::MathOverflow`. In Web3, you never assume your math fits in memory. You calculate it in a massive container, and you explicitly verify the upper bounds are completely empty before casting it back to the active state.
