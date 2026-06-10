---
description: 'Rust programming language coding conventions and best practices'
applyTo: '**/*.rs'
---

# Rust Coding Conventions and Best Practices

Follow idiomatic Rust practices and community standards when writing Rust code. 

These instructions are based on [The Rust Book](https://doc.rust-lang.org/book/), [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), [RFC 430 naming conventions](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md), and the broader Rust community at [users.rust-lang.org](https://users.rust-lang.org).

## General Instructions

- **Always** prioritize readability, safety, and maintainability, add comments where specific decisions have been made in data structure, algorithm, or crate choices.
- **Always** use strong typing and leverage Rust's ownership system for memory safety.
- **Always** maximize algorithmic big-O efficiency for memory and runtime, it's OK to ask which is a higher priority.
- **Always** use parallelization and SIMD _where appropriate_ , it's OK to ask if _appropriate_ is not clear.
- **Always** break down complex functions into smaller, more manageable functions.
- For algorithm-related code, **always** include explanations of the approach used.
- Write code with good maintainability practices, **always** including comments on why certain design decisions were made.
- **If** a crate can be imported to significantly reduce the amount of new code required to implement a function at optimal performance, and the crate itself is small and does not have much overhead, **Always** use the crate instead.
- For external dependencies, **always** mention their usage and purpose in documentation.

See also:

1. [Rust Error Handling Guidelines](./rust-error-handling.instructions.md)
2. [Rust Code Style and Formatting Guidelines](./rust-code-style.instructions.md)
3. [Rust Additional Tools](./rust-tools.instructions.md)
4. [Rust File Templates](./rust-file-templates.instructions.md)

## Patterns to Follow

- Use modules (`mod` - see template in [Module Template](#module-template) below) and public interfaces (`pub`) to encapsulate logic.
- Implement traits to abstract services or external dependencies.
- Structure async code using `async/await` and `tokio` or `async-std`.
- Prefer enums over flags and states for type safety.
- Use builders for complex object creation.
  - Use `with_field(mut self field: Type) -> Self` rather than `with_field(&mut self, field: Type) -> &mut Self` for better ergonomics.
- Split binary and library code (`main.rs` vs `lib.rs`) for testability and reuse.
- Prefer borrowing and zero-copy operations to avoid unnecessary allocations.

### Ownership, Borrowing, and Lifetimes

- Prefer borrowing (`&T`) over cloning unless ownership transfer is necessary.
- Use `&mut T` when you need to modify borrowed data.
- Explicitly annotate lifetimes when the compiler cannot infer them.
- Use `Rc<T>` for single-threaded reference counting and `Arc<T>` for thread-safe reference counting.
- Use `RefCell<T>` for interior mutability in single-threaded contexts and `Mutex<T>` or `RwLock<T>` for multi-threaded contexts.

### Common Traits Implementation

Eagerly implement common traits where appropriate:

- `Copy`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`, `Debug`, `Display`, `Default`
  - Consider using the `strum` crate for enums to automatically derive a number of these traits and more.
- Use standard conversion traits: `From`, `AsRef`, `AsMut`
- Collections should implement `FromIterator`, `IntoIterator`, and `Extend`
- Newtypes should implement `From`, `Deref`, and `DerefMut` to the inner type for ergonomic access.
- String-like newtypes implement `FromStr` as a constructor. 
  - Provide a method `is_valid(s: &str) -> bool` which at the very least is implemented as `Self::from_str(s).is_ok()` but where possible provides a more optimized way to check validity.
  - Alternatively, implement the validity check in `is_valid` and implement `FromStr` as `if Self::is_valid(s) { ... } else ...`.
- Note: `Send` and `Sync` are auto-implemented by the compiler when safe; avoid manual implementation unless using `unsafe` code

### Type Safety and Predictability

- Use newtypes to provide static distinctions.
- Arguments should convey meaning through types; prefer specific types over generic `bool` parameters.
- Use `Option<T>` appropriately for truly optional values.
- Functions with a clear receiver should be methods.
- Only smart pointers, and newtypes, should implement `Deref` and `DerefMut`.

## Function Design

- **Always** keep functions focused on a single responsibility.
- **Always** prefer borrowing (`&T`, `&mut T`) over ownership _when possible_.
- Functions that modify their arguments should take `&mut self` or `&mut T` rather than consuming ownership.
- Functions returning mutable data should have a name suffixed with `_mut` to indicate mutability.
- Limit function parameters to 5 or fewer; use a config struct for more.
- Return early to reduce nesting.
- Use iterators and combinators over explicit loops where clearer.
- Use iterators instead of index-based loops as they're often faster and safer.
- Return iterators from functions instead of collections when possible to allow for lazy evaluation and better performance.
- Use `AsRef<str>` over `&str` over `String` for function parameters when you don't need ownership.
  - Similarly use `AsRef<[T]>` over `&[T]` over `Vec<T>` or similar for collections.

## Struct  Design

- **Always** keep types focused on a single responsibility.
- **Always** derive common traits: `Debug`, `Clone`, `PartialEq`, except when necessary.
- Use `#[derive(Default)]` when a sensible/preferred default exists.
- Prefer composition over inheritance-like patterns.
- Make fields private by default; provide accessor methods _when needed_.
- Primary constructor methods should be named `new`.
- Additional constructor method names should start with `new_` or `new_with_`.
- Optional fields should have construction methods of the form `fn with_NAME(mut self, NAME: NAME_TYPE) -> Self` allowing them to be chained after `new`.
- Use builder pattern for complex construction cases.
  - For a type named `MyType` provide a `MyTypeBuilder` struct.
  - Add a method `fn builder() -> MyTypeBuilder` to `MyType`.
  - Implement _either_ `From<MyTypeBuilder>`, or possibly `TryFrom<MyTypeBuilder>` for MyType.
  - Add a method `build(self) -> MyType` to `MyTypeBuilder`, which may be called by the `From` or `TryFrom` implementation.

#### Enums Design

- Use enums to represent distinct states or variants of data.
- Enums should include `is_{variant}()` methods for ergonomic checks (see [strum::EnumIs](https://docs.rs/strum/latest/strum/derive.EnumIs.html)).
- Non-flag enums should include `try_as_{variant}() -> Option<{inner_type}>` methods for ergonomic access to inner data (see [strum::EnumTryAs](https://docs.rs/strum/latest/strum/derive.EnumTryAs.html)).

### Future Proofing

- Use sealed traits to protect against downstream implementations.
- Structs should have private fields.
- Functions should validate their arguments.
- All public types must implement `Debug`.

## Supporting no_std


For libraries, whenever possible support `no_std` by having an explicit `std` feature and falling back to `core` when `cfg(not(feature = "std))`.

```toml
[features]
default = ["std"]
std = ["alloc"]
alloc = []
```

```rust
#![cfg_attr(not(feature = "std"), no_std)]

// `alloc` is needed for `String`, `Box`, and other heap-allocated types, so we need to import it
// when `std` is not available. While `core` is always available, `alloc` is only available when
// the `alloc` crate is included explicitly as an `extern`.
#[cfg(all(not(feature = "std"))]
extern crate alloc;

// This pattern simply brings in some of the commonly used "prelude" types we assume are always
// available but without `std` they aren't.
#[cfg(not(feature = "std"))]w
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
```

## Patterns to Avoid

- **Do not** rely on global mutable state, use dependency injection or thread-safe containers.
- **Do not** deeply nested logic, refactor with functions or combinators.
- **Do not** ignore warnings—treat, them as errors during CI.
- **Do not** use `unsafe` unless required, and fully documented.
- **Do not** overuse `clone()`, use borrowing instead of cloning unless ownership transfer is needed.
- **Do not** prematurely `collect()`, keep iterators lazy until you actually need the collection.
- **Do not** use unnecessary allocations, prefer borrowing and zero-copy operations.

## Testing

- Write comprehensive unit tests using `#[cfg(test)]` modules and `#[test]` annotations.
- Use unit test modules alongside the code they test (`mod tests { ... }`).
- Write integration tests in `tests/` directory with descriptive filenames; by default `test_{feature_name}.rs`.
- Write clear and concise comments for each function, struct, enum, and complex logic.
- Ensure test functions have descriptive names and include comprehensive documentation.
- Document error conditions, panic scenarios, and safety considerations.
- Examples should use `?` operator, not `unwrap()` or deprecated `try!` macro.

### Benchmarks

- **Do not** run benchmarks in parallel, as the benchmarks will compete for resources and the results will be invalid.
- **Do not** game the benchmarks. Do not manipulate the benchmarks themselves to satisfy any required performance constraints.
- **Do not** run benchmarks with `target-cpu=native` or any other `RUSTFLAGS`.
- If benchmarking against another crate or library, ensure the benchmarks are apples-to-apples comparisons.
- Ensure benchmark tests are independent. If the tests are dependent due to a feature (e.g. caching), ensure the feature is disabled.

## Project Organization

- Use semantic versioning in `Cargo.toml`.
- Include comprehensive metadata: `description`, `license`, `repository`, `keywords`, `categories`.
- Use feature flags for optional functionality.
- Organize code into modules using `mod.rs` or named files.
- Keep `main.rs` or `lib.rs` minimal - move logic to modules.

## Quality Checklist

Before publishing or reviewing Rust code, ensure:

### Core Requirements

- [ ] **Naming**: Follows RFC 430 naming conventions.
- [ ] **Traits**: Implements `Debug`, `Clone`, `PartialEq` where appropriate.
- [ ] **Newtypes**: Implement traits that provide ergonomic access to inner types.
- [ ] **Error Handling**: Uses `Result<T, E>` and provides meaningful error types.
- [ ] **Documentation**: All public items have rustdoc comments with examples, also ensure consistency and completeness.
- [ ] **Testing**: Comprehensive test coverage including edge cases.
  - [ ] **Benchmarks**: Critical paths have benchmarks using `criterion` or similar.
  - [ ] **Property Tests**: Use `proptest` for complex logic where applicable.
- [ ] **Coverage**: Meets or exceeds the project's code coverage target using `cargo tarpaulin`.

### Safety and Quality

- [ ] **Safety**: No unnecessary `unsafe` code, proper error handling.
- [ ] **Performance**: Efficient use of iterators, minimal allocations.
- [ ] **API Design**: Functions are predictable, flexible, and type-safe.
- [ ] **Future Proofing**: Private fields in structs, sealed traits where appropriate.
- [ ] **Tooling**: Code passes `cargo fmt`, `cargo check`, `cargo clippy`, and `cargo test`.
- [ ] **Dependencies**: External dependencies are documented and checked with `cargo audit`, and `cargo outdated --depth 1` for vulnerabilities.
