# GideonTax

Deterministic US federal income tax calculator written in Rust.

## Overview

GideonTax aims to be a fully deterministic, auditable tax engine for US federal individual income tax. The project is built as a Rust workspace with modular crates:

- **gideon-tax-core** - Core tax calculation logic and types (filing status, tax brackets, etc.)
- **gideon-tax-form** - Tax form schemas and type definitions for IRS forms (W-2, 1099s, 1040, schedules, etc.)

## Project Structure

```
gideon-tax/
├── crates/
│   ├── gideon-tax-core/    # Core tax engine
│   └── gideon-tax-form/    # Form schemas and types
├── prompts/                # Research prompts for IRS rule extraction
└── Cargo.toml              # Workspace configuration
```

## Tax Year Coverage

- **2025** - Federal individual income tax forms (24 source forms, 32 return forms)

See [`crates/gideon-tax-form/README.md`](crates/gideon-tax-form/README.md) for the full form catalog.

## Building

```sh
cargo build
```

## License

Licensed under the [Gideon Christian Open Source License (GCOSL) v1.0](LICENSE).
