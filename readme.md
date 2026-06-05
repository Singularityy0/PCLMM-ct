# PCLMM: PClub Concentrated Liquidity Market Maker


### The Math Engine (`math/`)

The `math` crate is the isolated engine block of the protocol. It has no concept of "Solana," "Accounts," or "Wallets." It strictly consumes execution parameters and returns deterministic state transitions.



## Rules of Engagement (Contribution Guidelines)

Smart contract engineering is adversarial. Every pull request must adhere to the following architectural invariants. **Violations will result in automatic PR rejection.**

### 1. The Float Ban

Floating-point numbers (`f32`, `f64`) are strictly banned across the entire workspace. All pricing and token calculations must be executed using integer-based Q64.96 fixed-point arithmetic to guarantee consensus-level determinism.

### 2. Defend the Memory Boundary

You are manipulating 256-bit integers. Multiplication of two Q96 token amounts will silently overflow a standard `U256`.

- **Do NOT** use standard `*` and `/` operators for token fractions.
- **DO** use `full_math::mul_div` and `full_math::mul_div_rounding_up`. These functions utilize temporary 512-bit memory allocation to protect against intermediate overflows.

### 3. Graceful Failure Only (No Panics)

Panics are an attack vector. A malicious actor can force a panic to consume compute units and stall state.

- **Do NOT** use `.unwrap()`, `.expect()`, or `panic!()` in core logic.
- **DO** return `Result`. Check your bounds before you calculate.


## Pull Request Lifecycle

1. **Fork & Branch:** Create a branch prefixed with your feature (e.g., `math/tick-bitmap` or `prog/initialize-pool`).
2. **Implement:** Write your business logic adhering to the rules above.
3. **Test:** Ensure your code passes all existing tests. If you are adding a new mathematical feature, you must add a corresponding `proptest` or hardcoded vector to validate edge cases.
4. **Submit PR:** Request a review from a Core Maintainer. CI will automatically run `cargo clippy`, `cargo fmt`, and the test suite.

> "In Web2, you write code to get the correct answer. In Web3, you write code to get the correct answer within strict hardware limits, while under constant attack. Manage your bits." — PCLMM Core
