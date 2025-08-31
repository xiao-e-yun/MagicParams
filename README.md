# Magic Params
call functions with typed arguments derived from a shared context.

> Reference: [axum-style-magic-function-param](https://github.com/alexpusch/rust-magic-patterns/blob/master/axum-style-magic-function-param/Readme.md)

# Usage
```bash
cargo add magic-params
```

# Examples
```rust
define_context!(MyContext {
    value1: i32,
    value2: u32,
    value3: String,
});
context_as_params!(MyContext, 3);

fn main() {
    let ctx = MyContext {
        value1: 42,
        value2: 100,
        value3: "Hello".to_string(),
    };

    let handler = |v1: i32, v2: u32, v3: String| {
        assert_eq!(v1, 42);
        assert_eq!(v2, 100);
        assert_eq!(v3, "Hello");
    };

    handler.call(&ctx);
}
```
