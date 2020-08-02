# Rust Calculator
A simple mathematical expression parser and evaluator.

Use with `noglob` to avoid unwanted shell expansion.

Example:
```bash
noglob cargo run 1+2*3/7
# 1.8571429
noglob cargo run 2pi
# 6.2831855
```