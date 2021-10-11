# rtg Model

This module contains everything about the Model objects:

- `graphql_model`: Models expressed  in GraphQL SDL.
  - Generate the JSON Schema for Models
  - Serialize Model to JSON
  - Deserialization Model from JSON
- `implicit_model`: Models where all fields are optional: Useful to make them easy for humans
  - Generate the JSON Schema for Models
  - Serialize Model to JSON
  - Deserialization Model from JSON
  - `Models` structs and enums are derived automatically from `Explicit Models`, using the `partial-struct` crate
- `explicit_model`: Models where all fields are required: Useful pivotal object in rtg
  - Generate the JSON Schema for Explicit Models
  - Serialize Explicit Model to JSON
  - Deserialize Explicit Model from JSON
- `model_cache`: Models that are indexed by various properties: Useful during execution
  - Generate Model Cache from Explicit Models
  - Serialize Model Cache to JSON
  - Deserialize Model Cache from JSON

## Contribute

To run in TDD mode:

```bash
cargo watch --clear -x test
```

## Example

```rust
fn foo() -> i32 {
    1 + 1
}
assert_eq!(foo(), 2);
```
