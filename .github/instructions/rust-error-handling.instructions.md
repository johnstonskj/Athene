---
description: 'Rust error handling guidelines'
applyTo: '**/*.rs'
---

# Rust Error Handling Guidelines

- **Always** handle errors gracefully using `Result<T, E>` and **always** provide meaningful error messages.
- **Do** use `?`, `match`, `if let`, or `while let` in library code.
  - Use the `if let ... && validity_check` pattern in edition 2024 to reduce nested if statements.
- **Do not** use `unwrap()` or `expect()` unless absolutely necessary.
- **Do not** panic in library code, return `Result` instead.
- **Always** add a top-level `error` module.

## Library Error Type

- Unless there is some need to, don't hand-write error types.
  - Use `thiserror` to define library error type and `anyhow` for application-level errors.
  - Error variants should be descriptive and include relevant context.
  - Provide meaningful error messages with context using `.context()` from `anyhow`.
- Error types should be meaningful and well-behaved (implement standard traits).
- `error.rs` *may* provide helper functions for error variant construction, but should not contain significant logic.

## Library Errors

- Include the `#[must_use]` annotation for API functions.
- Validate function arguments and return appropriate errors for invalid input.
- Use `Result<T, E>` for recoverable errors and `panic!` only for unrecoverable errors.
- Use `std::convert::Infallible` instead of `()` for traits/functions that require an error type.
- Use `Option<T>` for values that may or may not exist.
