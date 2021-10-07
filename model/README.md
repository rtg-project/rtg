# rtg Model

This module contains everything about the Model object

- Generate the JSON Schema for Models
- Serialization Model to JSON
- Deserialization Model from JSON
- Optional Models = Models where all fields are optional (useful for simplifying config)
  - Serialize Optional Model to JSON
  - Deserialization Optional Model from JSON

## Contribute

To run in TDD mode:

```bash
cargo watch -x test
```

## Example

```rust
fn foo() -> i32 {
    1 + 1
}
assert_eq!(foo(), 2);
```
