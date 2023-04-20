## Usage/Examples
Checks the value of a boolean. Essential for type safety.

```rust
is_true(true); // true
is_true(false); // false
is_true("false"); // invalid

is_false(false); // true
is_false(true); // false
is_false("true"); // invalid
```
Assure that you only pass in boolean values.
