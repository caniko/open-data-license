# Phase 04 — SPDX parsing, safety lints, and ergonomic API additions

> **Recommended Codex model: GPT 5.5 medium**
>
> Mechanical additions (a `forbid(unsafe_code)` line, a `from_spdx_id` lookup
> table, doc improvements) with one design call: should `Display` keep
> returning the strum identifier (`"CC_BY_SA"`) or switch to the SPDX form
> (`"CC-BY-SA-4.0"`)? Medium reasoning handles "if you change Display you
> break round-trip with FromStr; keep Display and add `spdx_id()` as a sibling"
> without needing high-effort deliberation.

## Working tree
Branch `phase/04-ergonomics`. Independent of Phase 03 (no compatibility-rule changes), but rebases cleanly on top of Phase 02's variant additions.

## Goal
Expose the canonical SPDX identifier as a first-class parse target, add a no-unsafe guarantee, and round out small ergonomic gaps (`Default`, doc-cfg gating, README badges deferred to Phase 05).

## Why
The current `FromStr` impl (via strum) only accepts the screaming-snake identifier `"CC_BY_SA"`. But every external dataset metadata system in the wild stores `"CC-BY-SA-4.0"` (SPDX). Without a `from_spdx_id` helper, consumers have to maintain their own lookup table, which defeats the point of this crate. A `#![forbid(unsafe_code)]` line is a free signal for security-conscious consumers.

## Out of scope
- Compatibility-matrix changes (Phase 03).
- New license variants (Phase 02).
- Release-polish files (CHANGELOG, CONTRIBUTING, badges) — Phase 05.

## Plan
1. Add `#![forbid(unsafe_code)]` at the top of `src/lib.rs` (after the existing `#![deny(missing_docs)]`).
2. Add `pub fn from_spdx_id(s: &str) -> Option<DataLicense>` on `DataLicense`:
   - Match against the SPDX strings already stored in `meta()`.
   - Case-sensitive (SPDX is canonical lowercase-suffix); document this in the rustdoc.
   - PDM variant (no real SPDX ID): accept `"PDM-1.0"` for symmetry with `spdx_id()`.
3. Add `impl TryFrom<&str> for DataLicense` that delegates to `from_spdx_id` and returns `Result<Self, UnknownSpdxId>` where `UnknownSpdxId(String)` is a new error type implementing `Display`, `Error`, `Debug`, `Clone`, `PartialEq`, `Eq`.
4. Decision: do **not** change `Display`. Keep the existing strum-driven `"CC_BY"` form so `FromStr` ↔ `Display` round-trip is preserved. Document this in the rustdoc on `DataLicense`.
5. Add tests in `src/tests.rs`:
   - `from_spdx_id_roundtrip`: for every variant `l`, `from_spdx_id(l.spdx_id()) == Some(l)`.
   - `from_spdx_id_rejects_unknown`: returns `None` for `""`, `"GPL-3.0"`, `"cc-by-4.0"` (lowercase).
   - `try_from_str_error_carries_input`: the `UnknownSpdxId` error's `Display` includes the input string.
6. Update `docs/src/api-overview.md` with a "Parsing SPDX identifiers" subsection.
7. Update the lib.rs example to also show `DataLicense::from_spdx_id("CC-BY-SA-4.0")`.

## Acceptance criteria
- [ ] `cargo build --all-features` and `cargo build --no-default-features` both succeed; clippy with `--deny warnings` passes.
- [ ] `DataLicense::from_spdx_id("CC-BY-SA-4.0") == Some(DataLicense::CcBySa)`.
- [ ] `DataLicense::from_spdx_id("cc-by-sa-4.0") == None` (case-sensitive).
- [ ] `<DataLicense as TryFrom<&str>>::try_from("GPL-3.0")` returns `Err(UnknownSpdxId("GPL-3.0".into()))`.
- [ ] For every variant `l`, `DataLicense::from_spdx_id(l.spdx_id()) == Some(l)`.
- [ ] `#![forbid(unsafe_code)]` is present and `nix flake check` still passes.
- [ ] `Display` output for every variant is unchanged from before this phase (existing `strum_roundtrip_all` test still passes).

## Files likely touched
- `src/lib.rs`
- `src/tests.rs`
- `docs/src/api-overview.md`

## Pitfalls
- Do not implement `FromStr` for the SPDX form — strum already provides `FromStr` for the screaming-snake form, and providing two `FromStr` impls is not possible. Use `TryFrom<&str>` for the SPDX path and document the asymmetry clearly.
- If the new error type uses `thiserror`, that's a new dependency. Prefer hand-rolling `impl Display + std::error::Error` to keep the dep tree at three crates (serde, strum, optional utoipa).
- `from_spdx_id` is a hot path for some consumers (parsing dataset manifests). Use a `match` (which the compiler will lower to a jump table) rather than building a `HashMap`.

## Reference
- SPDX license expressions: <https://spdx.dev/specifications/>
- Rust API guideline C-CONV-TRAITS: <https://rust-lang.github.io/api-guidelines/interoperability.html>
