---
description: 'Rust programming language code style and formatting guidelines'
applyTo: '**/*.rs'
---

# Rust Code Style and Formatting Guidelines

- **Always** use consistent naming conventions following [RFC 430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md).
- **Always** write idiomatic, safe, and efficient Rust code that follows the borrow checker's rules.
- **Always** ensure code compiles without warnings, using the following block of lint attributes in `Cargo.toml:

```toml
[workspace.lints.rust]
exported_private_dependencies = "deny"
anonymous_parameters = "deny"
bare_trait_objects = "deny"
deref_nullptr = "deny"
drop_bounds = "deny"
dyn_drop = "deny"
ellipsis_inclusive_range_patterns = "deny"
unsafe_code = "forbid"
unsafe_op_in_unsafe_fn = "forbid"

absolute_paths_not_starting_with_crate = "warn"
elided_lifetimes_in_paths = "warn"
explicit_outlives_requirements = "warn"
future_incompatible = { level = "warn", priority = -1 }
macro_use_extern_crate = "warn"
missing_debug_implementations = "warn"
nonstandard_style = { level = "warn", priority = -1 }
noop_method_call = "warn"
rust_2018_idioms = { level = "warn", priority = -1 }
rust_2021_compatibility = { level = "warn", priority = -1 }
rust_2021_prelude_collisions = "warn"
semicolon_in_expressions_from_macros = "warn"
single_use_lifetimes = "warn"
trivial_casts = "warn"
trivial_numeric_casts = "warn"
unexpected_cfgs = "warn"
unreachable_pub = "warn"
unused = { level = "warn", priority = -1 }

[workspace.lints.rustdoc]
all = "warn"
missing_crate_level_docs = "warn"
broken_intra_doc_links = "warn"

[workspace.lints.clippy]
all = "warn"
```

## rustfmt

- Follow the Rust Style Guide and use `rustfmt` for automatic formatting.
  - Do not override format defaults without documentation and team consensus.
  - Just In Case: you **must** use 4 spaces for indentation, **never** tabs.
- Keep lines under 100 characters when possible.

The project should include an empty `rustfmt.toml` file such as this:

```toml
# This is an empty `rustfmt.toml` file. This means that the default configuration will
# always be used, as per-project configuration overrides any per-user configuration.
```

## Comments / Documentation

- Document all public APIs with rustdoc following the [API Guidelines](https://rust-lang.github.io/api-guidelines/).
  - Any non-trivial public item should have an example.
- Use `#[doc(hidden)]` to hide implementation details from public documentation.
- Place function and struct documentation immediately before the item using `///`.
- Place module-level documentation at the top of the file using `//!`.
- Avoid including redundant comments which are tautological or self-demonstating; 
  e.g. cases where it is easily parsable what the code does at a glance or its 
  function name giving sufficient information as to what the code does.
- Keep README file examples up-to-date:
  - with the code examples from `lib.rs` for library packages; use ` ```rust no_run` for examples with placeholders.
  - with command-line invocations with the `--help` option; use ` ```bash` for examples with placeholders.


Include the content of [@`Cargo-docs.toml`](https://github.com/johnstonskj/agent-instructions/blob/main/rust/templates/Cargo-docs.toml) in each package's `Cargo.toml` file.

To customize the documentation generation, use the `rustdoc-args` key in the`docs` metadata in `Cargo.toml`, for example:

```toml
[package.metadata.docs.rs]
# This allows only the default target ("x86_64-unknown-linux-gnu") to run, saving time and resources on docs.rs.
targets = []

# Generally you want to document all, but you can specify features if you want to only document a subset of features.
all-features = true

# Uncomment the following to add custom stylesheets/scripts to the generated HTML files.
# rustdoc-args = [ "--html-in-header", "doc-src/header.html" ]
```

## Linting

- Use *both* `cargo check` and`cargo clippy` (just in case) in conjunction with
  `cargo fmt` to catch common mistakes and enforce best practices.
