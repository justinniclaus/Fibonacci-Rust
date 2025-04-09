## Core Understanding of the Fibonacci Function

- **Fibonacci sequence**: Mathematical sequence where each number is the sum of the two preceding ones (0, 1, 1, 2, 3, 5, 8, 13, ...)
- **Base cases**:
    - F(0) = 0
    - F(1) = 1

## Rust Implementation Analysis

```rust
fn fibonacci(value: i32) -> i32
{
    if value <= 0 { return 0; }
    else if value == 1 { return 1; }

    let mut a = 0;
    let mut b = 1;

    for i in 2..=value 
    {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
```

## Step-by-Step Execution for fibonacci(3)

1. **Parameter validation**:
    
    - Check if `value <= 0` → No, continue
    - Check if `value == 1` → No, continue
2. **Variable initialization**:
    
    - `a = 0` (represents F(0))
    - `b = 1` (represents F(1))
3. **Loop execution** (`for i in 2..=3`):
    
    - **Iteration 1** (i = 2):
        - `temp = a + b = 0 + 1 = 1`
        - `a = b` → `a = 1`
        - `b = temp` → `b = 1`
        - State after iteration: a = 1, b = 1
    - **Iteration 2** (i = 3):
        - `temp = a + b = 1 + 1 = 2`
        - `a = b` → `a = 1`
        - `b = temp` → `b = 2`
        - State after iteration: a = 1, b = 2
4. **Return value**: `b` = 2
    

## Visual Representation

```
F(0) = 0 → Initial a
F(1) = 1 → Initial b
F(2) = F(1) + F(0) = 1 + 0 = 1 → First iteration
F(3) = F(2) + F(1) = 1 + 1 = 2 → Second iteration
```

## Key Concepts

- **Iterative approach**: O(n) time complexity, more efficient than recursive implementation
- **Mutable variables**: Use of `mut` keyword for variables that change value
- **Range syntax**: Rust's `2..=value` creates an inclusive range from 2 to value
- **Expression-based return**: Final line has no semicolon, making it the return value

## Why Start at i=2?

- We start the loop at `i=2` because:
    - F(0) and F(1) are already handled by the base cases
    - The variables `a` and `b` are initialized to F(0)=0 and F(1)=1 respectively
    - The first iteration (i=2) calculates F(2) from F(1) and F(0)
    - Each subsequent iteration calculates F(i) from the two preceding values
- Starting at i=2 aligns with the mathematical definition of the Fibonacci sequence:
    - F(n) = F(n-1) + F(n-2) for n ≥ 2
- This approach correctly maps loop iterations to Fibonacci indices
