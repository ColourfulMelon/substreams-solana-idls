# Repository Navigation Guide

This document captures durable navigation and workflow knowledge for `substreams-solana-idls`.

## 1) Workspace Layout

- `Cargo.toml`: single-crate package metadata and dependencies.
- `src/lib.rs`: protocol module exports.
- `src/common/`: shared `ParseError` and account parsing helpers used by protocol modules.
- `src/<protocol>/`: one module per protocol (for example `raydium`, `jupiter`, `pumpfun`).
- `.github/workflows/`: CI/publish pipelines.
- `target/`: Rust build artifacts (generated; not source).

Each protocol module typically contains:

- `mod.rs`: protocol module entrypoint and `PROGRAM_ID` constants.
- `instructions.rs`: instruction enums + `unpack`.
- `events.rs`: event enums/structs + `unpack` (if supported).
- `accounts.rs`: account helper structs/functions (if needed).
- `<variant>/`: versioned or product-specific submodules (`v4`, `v6`, `amm`, `clmm`, etc.).
- `tests/*.rs`: decode fixtures and behavior checks.

## 2) Architecture / Data-Flow Mental Model

- Input: raw Solana instruction/log bytes.
- Decoder entrypoint: `<module>::instructions::unpack(data)` and/or `<module>::events::unpack(data)`.
- Dispatch mechanism: match on discriminators, then Borsh-deserialize into typed structs.
- Shared errors: `common::ParseError` standardizes decode failures across crates.
- Substreams usage: downstream mappings filter by `PROGRAM_ID`, decode, then map to output entities.

Think of each protocol module as a codec library with a stable decode API: `PROGRAM_ID` + `unpack` + typed structs.

## 3) Where To Edit For X

- Add a new protocol module:
  - Add `src/<protocol>/`.
  - Register the module in `src/lib.rs`.

- Add support for a new program/version inside an existing protocol:
  - Add a new module under `src/<protocol>/<variant>/`.
  - Wire it in `src/<protocol>/mod.rs` or parent `mod.rs`.
  - Add/extend tests in `tests/<protocol>/`.

- Update instruction decoding logic:
  - Edit `instructions.rs` in the relevant protocol/variant module.

- Update event decoding logic:
  - Edit `events.rs` (or `anchor_cpi_event.rs`/`logs.rs` where present).

- Update account parsing helpers:
  - Edit `accounts.rs` in the relevant module.
  - Shared account helper macros live in `src/common/accounts.rs`.

- Change shared parse error behavior:
  - Edit `src/common/mod.rs` (`ParseError`).

- Expose protocol from top-level crate path:
  - Add `pub mod <protocol>;` in `src/lib.rs`.

## 4) Key Build/Test Anchors

- Format check: `cargo fmt --all -- --check`
- Lint check: `cargo clippy --workspace -- -D warnings`
- Tests: `cargo test --workspace`
- WASM compatibility: `cargo check --workspace --target wasm32-unknown-unknown`

CI equivalents are defined in `.github/workflows/ci.yml`.

## 5) Source vs Generated Artifacts

Source of truth:

- `src/**/*.rs`
- `tests/**/*.rs`
- `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`
- `.github/workflows/*.yml`

Generated/build artifacts:

- `target/**`

Committed data artifacts (treated as inputs, not build outputs):

- `src/**/idl*.json`

Those JSON IDLs are checked in and used by maintainers as decoder references.

## Current Pump trade variants

The bonding curve supports legacy buy/sell, exact SOL input, and V2/V3
buy/sell/exact quote input. Token quote mints are decoded from the current
TradeEvent prefix, including its variable string and shareholder vector;
fixed offsets after those fields are unsafe. Official IDLs:
https://github.com/pump-fun/pump-public-docs/tree/main/idl.

Finalized transactions on 2026-10-11 show a second boolean after buy_v2 and
legacy PumpSwap buy flags even where the published IDL declares one. The
parser accepts up to two validated boolean bytes; arbitrary trailing bytes
remain errors. PumpSwap V2 uses `TradeTokenAccounts` for the shared first nine
accounts. Legacy fee recipients at indices 9/10 become token programs in V2.
