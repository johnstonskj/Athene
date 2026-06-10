---
description: 'Rust additional tools'
applyTo: '**/*.rs'
---

# Rust Additional Tools


Tool specification **may** be externalized to [mise](https://mise.jdx.dev) by providing a `mise.toml` file. A default file example is provided below, but the specific tools and versions should be determined by the team based on project needs and preferences.

```toml
[tools]
rust = "latest"
```

### Build

- Use `cargo` for project management, building, and dependency management.
- Use `cargo test` for running tests
- Use `cargo doc` for generating documentation
- **Must** use `rustfmt` for code formatting
- **Must** use `clippy` for linting and follow its suggestions
- **Must** ensure code compiles with no warnings (use `-D warnings` flag in CI, not `#![deny(warnings)]` in source)

### Logging/Tracing

- Should use the Tokio `tracing` module.
- For top-level, or API, functions use the tracing `#[instrument]` attribute, requires the tracing feature `attributes`.
- For complex, or coordination functions either use `#[instrument]` or manual spans.
- When reporting errors to the console, use `tracing::error!` or `log::error!` instead of `println!`.
- For async code, always use `tracing-futures`.
- For file-based logs usually use `tracing-appender`.
- For test cases depending on trace/log output usually use the `test-log` crate and attribute to initialize tracing in tests.

**Never** log sensitive information; e.g. passwords, tokens, PII.

More complex telemetry should:

- Usually use `opentelemetry` (and `opentelemetry_sdk`).
  - Use `tracing-opentelemetry` to emit trace data through Open Telemetry. Also, use `opentelemetry-stdout` to emit a local copy of the traces.
  - Use `opentelemetry_otlp` to connect to a sink.
- Usually encapsulate initialization in a `telemetry.rs` private module.
  - provide a common `init(...)` function, which calls `init_logging`, `init_metrics`, and `init_tracing`.


### Code Coverage

- Use `cargo-tarpaulin` to run coverage tests.
- Include a `codecov.yml` file with target set to **at least** 75%, and ideally 90%.

```yaml
coverage:
  status:
    project:
      default:
        target: 85%
        threshold: 5%

component_management:
  individual_components:
    - component_id: error
      name: Error Management
      paths:
        - src/error.rs
```

### Additional Common Crates

- **Always** ensure a safe crate supply-chain, using `cargo audit` and using crates with a higher usage among alternatives.


| Use Case                       | Preferred                        | Notes                                                               |
| ------------------------------ | -------------------------------- | ------------------------------------------------------------------- |
| Abstract Number Types          | `num-traits`                     | Provides traits for abstract numeric types.                         |
| Async HTTP client              | `reqwest`                        | High-level HTTP client built on `hyper`.                            |
| Async HTTP server              | `axum`                           | High-level web framework built on `hyper` and `tower`.              |
| Async runtime                  | `tokio`                          | Most widely used async runtime with rich ecosystem.                 |
| Bitflags                       | `bitflags`                       | Provides a convenient way to define bitflags.                       |
| Cli `--verbose` flag handling  | `clap-verbosity-flag`            | Provides easy handling of verbosity flags in CLI applications.      |
| Cli completions command        | `clap_complete`                  | Provides easy generation of shell completions for CLI applications. |
| Cli interactive input          | `inquire`                        | Simple input functions for CLI applications.                        |
| Cli progress bars              | `indicatif`                      | Provides easy-to-use progress bars for CLI applications.            |
| Command-line parsing           | `clap`                           | Feature-rich command-line argument parser.                          |
| Data frame manipulation        | `polars`                         | Fast and feature-rich data frame library.                           |
| Data parallelism               | `rayon`                          | Easy-to-use data parallelism for CPU-bound tasks.                   |
| Date handling/formatting       | `chrono`                         | Most complete and widely used date/time crate.                      |
| Enum utilities                 | `strum`                          | Provides utilities for working with enums, such as iteration.       |
| Error handling in applications | `anyhow`                         | Provides easy error handling for applications with context support. |
| Error handling in libraries    | `thiserror`                      | Provides easy error type definitions for libraries.                 |
| High-level terminal UI         | `ratatui`                        | Provides tools for building rich terminal user interfaces.          |
| Human-readable date formatting | `humantime`                      | Provides easy-to-read date formatting.                              |
| Localization                   | `i18n-embed` + `i18n-embed-fl`   | Provides tools for embedding localized text.                        |
| Low-level terminal control     | `crossterm`                      | Cross-platform terminal manipulation library.                       |
| Readline-style input           | `rustyline` + `rustyline-derive` | Provides readline-style input handling for CLI applications.        |
| Sensitive data types           | `secrecy`                        | Provides secure handling of sensitive data.                         |
| Serialization                  | `serde`                          | Widely used serialization framework, use with `derive` support.     |
| Tracing API                    | `tracing`                        | Provides a powerful and flexible tracing API.                       |
| Tracing file appender          | `tracing-appender`               | Provides file-based logging for `tracing`.                          |
| Tracing Subscriber             | `tracing_subscriber`             | Provides utilities for configuring tracing subscribers.             |
| Tracing test support           | `test-log`                       | Provides utilities for testing code that emits logs/traces.         |
| Tracing to OpenTelemetry       | `tracing-opentelemetry`          | Provides integration between `tracing` and OpenTelemetry.           |
| Units of Measurement           | `uom`                            | Provides type-safe handling of units of measurement.                |
