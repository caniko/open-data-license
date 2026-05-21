# Phase 02 — Mark enums `#[non_exhaustive]` and expand license coverage

> **Recommended Codex model: GPT 5.5 medium**
>
> Adding enum variants is mechanical, but the design call on *which* licenses
> to include (and where they land in the restrictiveness ranking) requires
> knowledge of the dataset-licensing landscape. Medium reasoning is enough to
> reason about CDLA-Permissive vs CDLA-Sharing placement; `low` would just
> dump variants in without thought about ordering and break Phase 03.

## Working tree
Branch `phase/02-license-coverage`, depends on Phase 01 being merged (or rebases cleanly on it).

## Goal
Make the `DataLicense` and `DataUseRestrictionKind` enums forward-compatible (`#[non_exhaustive]`) and add the missing licenses that modern open-data initiatives actually use.

## Why
Today, adding a new license variant is a SemVer-breaking change because downstream `match` expressions are checked for exhaustiveness. `#[non_exhaustive]` makes future additions minor-version safe. Separately, the current coverage misses the Linux Foundation's **CDLA-Permissive-2.0** and **CDLA-Sharing-1.0** licenses — both common in ML/AI dataset releases — and the **Public Domain Mark 1.0** which Europeana, Wikimedia Commons, and museum data dumps use heavily.

## Out of scope
- Compatibility-matrix rules for new variants (deferred to Phase 03; this phase just inserts conservative defaults and lets Phase 03 sharpen them).
- An `Other(String)` escape-hatch variant — separate design discussion; the `#[non_exhaustive]` attribute is the cheaper first move.

## Plan
1. In `src/lib.rs`, add `#[non_exhaustive]` to the `DataLicense` enum.
2. In `src/data_use_restriction.rs`, add `#[non_exhaustive]` to `DataUseRestrictionKind` and to `DataUseRestrictionSpec` (struct).
3. Add new variants to `DataLicense` with appropriate `#[strum(serialize = ...)]` and `#[serde(rename = ...)]`:
   - `CdlaPermissive2_0` → strum/serde `"CDLA_PERMISSIVE_2_0"`, SPDX `"CDLA-Permissive-2.0"`, URI `https://cdla.dev/permissive-2-0/`.
   - `CdlaSharing1_0` → strum/serde `"CDLA_SHARING_1_0"`, SPDX `"CDLA-Sharing-1.0"`, URI `https://cdla.dev/sharing-1-0/`.
   - `Pdm` (Public Domain Mark) → strum/serde `"PDM"`, SPDX `"PDDL-1.0"` is wrong here — PDM has no SPDX ID; expose `""` is also wrong. Decision: use the literal `"PDM-1.0"` (community shorthand) and document in the rustdoc that it is *not* an SPDX-registered identifier.
4. Wire each new variant through `meta()`, `is_public_domain()`, `allows_commercial_use()`, `requires_share_alike()`, `requires_attribution()`, and `restrictiveness()`:
   - **CDLA-Permissive-2.0**: permissive, commercial OK, no share-alike, no attribution requirement on data (data 2.0 dropped attribution). Restrictiveness 1.
   - **CDLA-Sharing-1.0**: share-alike, commercial OK, attribution required. Restrictiveness 2 (same band as CC-BY-SA).
   - **PDM**: public-domain marker (not a license — a marker of pre-existing PD status). Treat like `Cc0`/`Pddl` for all booleans. Restrictiveness 0.
5. Add tests in `src/tests.rs`:
   - Extend `serde_roundtrip_all` expected table with the three new entries.
   - Update `hash_all_distinct` count from `8` to `11`.
   - Add a `non_exhaustive_compiles` doctest in `src/lib.rs` showing that a downstream `match` with a `_ =>` arm continues to compile after additions.
6. Update `docs/src/api-overview.md` enum list.
7. Update `docs/src/introduction.md` to list the new coverage (CDLA + PDM).

## Acceptance criteria
- [ ] `DataLicense` and `DataUseRestrictionKind` carry `#[non_exhaustive]`.
- [ ] `DataLicense::iter().count()` returns `11`.
- [ ] `serde_json::to_string(&DataLicense::CdlaPermissive2_0)` returns `"\"CDLA_PERMISSIVE_2_0\""`; round-trip works for all three new variants.
- [ ] `spdx_id`, `rights_uri`, `display_name` return non-empty strings for every variant (verified by the existing iter-based tests).
- [ ] `restrictiveness()` is total: every variant has a defined value, and the existing `restrictiveness_ordering_is_total` test still passes.
- [ ] `nix flake check --keep-going --print-build-logs` passes.
- [ ] README and docs/src/{introduction,api-overview}.md list the three new licenses.

## Files likely touched
- `src/lib.rs`
- `src/data_use_restriction.rs`
- `src/tests.rs`
- `README.md`
- `docs/src/introduction.md`
- `docs/src/api-overview.md`

## Pitfalls
- **PDM is not legally a license** — it is an assertion that a work is already in the public domain. Document this in the variant's rustdoc so consumers don't conflate it with `Cc0` (which is an active dedication).
- **CDLA-Permissive-2.0 dropped the attribution requirement** that v1.0 had. Don't reflexively mark it `requires_attribution = true`.
- **`#[non_exhaustive]` on the struct `DataUseRestrictionSpec`** means downstream code can't construct it with `Spec { kind, summary, source_url }` literal syntax. If that's undesirable, leave the struct exhaustive and only mark the enum. Pick one and document the rationale in the commit message.
- Variant identifiers with digits (`CdlaPermissive2_0`) trip rustfmt sometimes — leave as-is, it's the conventional naming.

## Reference
- CDLA spec: <https://cdla.dev/>
- Public Domain Mark: <https://creativecommons.org/publicdomain/mark/1.0/>
- SPDX license list: <https://spdx.org/licenses/>
- Rust `#[non_exhaustive]` reference: <https://doc.rust-lang.org/reference/attributes/type_system.html>
