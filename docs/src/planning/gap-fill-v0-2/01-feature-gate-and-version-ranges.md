# Phase 01 — Feature-gate utoipa and relax version pins

> **Recommended Codex model: GPT 5.5 medium**
>
> Mechanical refactor with one design choice (default-on vs default-off for the
> `utoipa` feature) and a downstream-impact judgement on how loose the version
> ranges should be. A smaller model would likely pick exact-pin defaults out of
> caution and re-introduce the very problem we're fixing; medium reasoning is
> enough to internalize "this is a library, not an app" and apply caret ranges
> with intent.

## Working tree
Fresh `git worktree` off `main`, or a feature branch `phase/01-feature-gate-deps`.

## Goal
Make `open-data-license` a polite library dependency: optional `utoipa` integration behind a feature flag, and semver-friendly version ranges instead of exact `=x.y.z` pins.

## Why
The crate currently forces every downstream user to pull `utoipa = "=5.4.0"` plus its `axum_extras`, `uuid`, `chrono` features. That drags in axum, hyper, tower, tokio transitively — wildly disproportionate for a metadata enum. The `=x.y.z` pins on `serde` and `strum` will fight any consumer that uses a different patch version. For a 0.x library aiming at crates.io publication, both are blockers in practice.

## Out of scope
- New license variants (Phase 02).
- Changes to the compatibility model (Phase 03).
- Display / SPDX-parsing API changes (Phase 04).

## Plan
1. In `Cargo.toml`:
   - Change `serde = { version = "=1.0.228", ... }` to `serde = { version = "1.0", features = ["derive"] }`.
   - Change `strum = { version = "=0.28", ... }` to `strum = { version = "0.28", features = ["derive"] }`.
   - Change `utoipa` to `optional = true` with the same features, version `"5"` (caret).
   - Add `[features]` section: `default = []`, `utoipa = ["dep:utoipa"]`.
   - Bump `[package.metadata.docs.rs]` to keep `all-features = true` so docs.rs shows the utoipa-gated items.
2. In `src/lib.rs` and `src/data_use_restriction.rs`:
   - Gate every `use utoipa::ToSchema;` and every `ToSchema` derive with `#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]`.
   - Remove the bare `use utoipa::ToSchema;` imports — the `cfg_attr` form is self-contained.
   - Add `#![cfg_attr(docsrs, feature(doc_cfg))]` at the top of `src/lib.rs` for future feature-gating docs.
3. In `dev-dependencies`, relax `serde_json = "=1.0.149"` to `serde_json = "1"`.
4. Update `README.md` "Install" snippet to mention the `utoipa` feature:
   ```toml
   open-data-license = { version = "0.2", features = ["utoipa"] }
   ```
5. Add a "Features" section to `docs/src/api-overview.md` describing the `utoipa` flag.
6. Verify: `nix flake check --keep-going --print-build-logs` and `nix develop -c cargo build --all-features` both pass; `cargo build` with no features also passes and does not link utoipa (check `cargo tree --no-default-features`).

## Acceptance criteria
- [ ] `cargo build` (no features) succeeds and `cargo tree --no-default-features` does **not** list utoipa, axum, hyper, tokio, uuid, or chrono.
- [ ] `cargo build --features utoipa` succeeds and `ToSchema` is implemented for `DataLicense`, `DataUseRestrictionKind`, `DataUseRestrictionSpec`.
- [ ] `Cargo.toml` contains no `=x.y.z` pins for `serde`, `strum`, `utoipa`, or `serde_json`.
- [ ] `nix flake check --keep-going --print-build-logs` passes.
- [ ] README install snippet mentions the `utoipa` feature.
- [ ] `docs/src/api-overview.md` documents the `utoipa` feature.
- [ ] Existing tests still pass under both `--no-default-features` and `--all-features`.

## Files likely touched
- `Cargo.toml`
- `Cargo.lock` (regenerated)
- `src/lib.rs`
- `src/data_use_restriction.rs`
- `README.md`
- `docs/src/api-overview.md`

## Pitfalls
- Removing the `use utoipa::ToSchema;` line without converting derives leaves an unused-import warning in the `--features utoipa` build and a hard error in the no-feature build. Use `cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))` directly on each struct/enum so no import is needed.
- `cargo deny` may complain about `multiple-versions = "warn"` once we relax the pins. That's fine for this phase; do not silence it by re-pinning.
- Do not also bump the major version of `utoipa` here — that's a separate concern. Just make it optional at the current major.

## Reference
- crates.io API guidelines on optional features: <https://rust-lang.github.io/api-guidelines/future-proofing.html>
- utoipa optional integration pattern in many ecosystem crates (e.g. `axum-extra`, `validator`).
