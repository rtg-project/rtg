# rtg Model

This module contains everything about the Model object

- `Models`: Models where all fields are optional: Useful for making it easy for humans
  - Generate the JSON Schema for Models
  - Serialize Model to JSON
  - Deserialization Model from JSON
  - `Models` structs and enums are derived automatically from `Explicit Models`, using the `partial-struct` crate
- `Explicit Models`: Models where all fields are required: Useful pivotal object in rtg
  - Generate the JSON Schema for Explicit Models
  - Serialize Explicit Model to JSON
  - Deserialize Explicit Model from JSON
- `Model Cache`: Models that are indexed by various properties: Useful during execution
  - Generate Model Cache from Explicit Models
  - Serialize Model Cache to JSON
  - Deserialize Model Cache from JSON

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
