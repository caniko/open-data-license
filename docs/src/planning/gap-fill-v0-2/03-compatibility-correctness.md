# Phase 03 — Fix compatibility model + add property-based tests

> **Recommended Codex model: GPT 5.5 high**
>
> This phase carries the most legal/design risk in the plan. The current
> compatibility check has a documented-as-wrong case (`CC-BY-SA` ↔ `ODbL`
> returns `true`), and the fix requires reasoning about share-alike family
> membership, one-way relicensing semantics, and reflexivity/symmetry
> invariants. A `medium` model would likely "fix" the cross-family bug while
> introducing a new asymmetry; `high` is warranted to keep the invariants
> intact. Not `max` — the search space is small and the legal references are
> clear-cut once you anchor on "share-alike is family-local."

## Working tree
Branch `phase/03-compatibility-correctness`. Depends on Phase 02 being merged (new variants must already exist so the matrix tests cover them).

## Goal
Tighten `is_compatible_with` so that share-alike licenses from different families are correctly reported as incompatible, and add proptest coverage that pins down reflexivity, symmetry, and the public-domain absorbing-element property.

## Why
The current code has an explicit bug acknowledged in `cross_family_sa_incompatible` (which the test name promises but the assertion contradicts): `CC-BY-SA` and `ODbL` both sit at restrictiveness 2, so the "same band" rule lets them through. Legally they are not compatible — a CC-BY-SA work cannot be relicensed under ODbL or vice versa. Similarly, `CC-BY` → `ODbL` is currently allowed because only one side is share-alike, but ODbL's share-alike provision would still attach to any combined database. Shipping 0.2 with a known-wrong compatibility check is worse than not shipping the helper at all.

## Out of scope
- A full SPDX-style "license inbound/outbound compatibility graph". We're keeping the metadata-helper framing — just making the existing helpers correct.
- Adding new public API surface (that's Phase 04).

## Plan
1. Introduce a private `family()` method on `DataLicense` returning an enum `LicenseFamily { CreativeCommons, OpenDataCommons, Cdla, PublicDomain }`.
2. Rewrite `is_compatible_with` with these rules, in order:
   - If either side is `is_public_domain()` → `true`.
   - If `allows_commercial_use()` differs → `false`.
   - If either side `requires_share_alike()` and the families differ → `false` (this is the bug fix; covers CC-BY-SA ↔ ODbL, CC-BY ↔ ODbL, CDLA-Sharing ↔ CC-BY-SA, etc).
   - If both sides `requires_share_alike()` and the families match → `true` only when the variant is the same. (Cross-version SA within a family is conservatively rejected; document this.)
   - Otherwise → `true`.
3. Update or delete the misleading `cross_family_sa_incompatible` test — rename it to `cross_family_sa_now_incompatible` and assert `!result`.
4. Add `proptest` as a `dev-dependency` (caret range; do not pin).
5. Add a new module `src/property_tests.rs` (or extend `tests.rs`) with proptest cases:
   - **Reflexivity**: for any `DataLicense`, `l.is_compatible_with(&l) == true`.
   - **Symmetry**: for any `(a, b)`, `a.is_compatible_with(&b) == b.is_compatible_with(&a)`.
   - **Public-domain absorption**: for any `l`, `Cc0.is_compatible_with(&l)` and `Pddl.is_compatible_with(&l)` are both `true`.
   - **`most_restrictive` agreement**: `most_restrictive(a, b).is_some() == a.is_compatible_with(&b)`.
   Use a `prop_oneof!` over a hand-listed array of all variants — do NOT use `Arbitrary` derive (overkill for a small enum and brings extra deps).
6. Update `docs/src/compatibility.md` with the new rules and a worked example of a cross-family rejection.

## Acceptance criteria
- [ ] `DataLicense::CcBySa.is_compatible_with(&DataLicense::OdcOdbl) == false`.
- [ ] `DataLicense::CcBy.is_compatible_with(&DataLicense::OdcOdbl) == false`.
- [ ] `DataLicense::CcBySa.is_compatible_with(&DataLicense::CcBySa) == true`.
- [ ] All existing tests in `src/tests.rs` either pass unchanged or have been updated with a code-review-worthy commit message explaining the semantic shift.
- [ ] `proptest`-based reflexivity, symmetry, and absorption properties hold across at least 256 cases each.
- [ ] `nix flake check --keep-going --print-build-logs` passes.
- [ ] `docs/src/compatibility.md` documents the family-aware rule and gives the CC-BY-SA ↔ ODbL example.

## Files likely touched
- `src/lib.rs` (new `family()` method, rewritten `is_compatible_with`)
- `src/tests.rs` (rename and invert the cross-family test; keep symmetry test)
- `src/property_tests.rs` (new)
- `Cargo.toml` (add `proptest` as dev-dep)
- `docs/src/compatibility.md`

## Pitfalls
- The previous logic returned `true` for `CC-BY` ↔ `ODbL` because only one side had share-alike. Resist the temptation to keep this for "backwards compatibility" — it was wrong. Note the behavior change clearly in the commit message; the version bump to 0.2 already signals it's a minor with potentially-breaking semantic changes for users who depended on the broken behavior.
- proptest can balloon CI time. Cap with `#![proptest_config(ProptestConfig { cases: 256, .. })]` at module level.
- Keep `family()` private. Exposing it is a Phase 04 conversation, not this one.

## Reference
- ODbL share-alike clause 4.4: <https://opendatacommons.org/licenses/odbl/1-0/>
- Creative Commons compatibility wiki: <https://wiki.creativecommons.org/wiki/ShareAlike_compatibility>
- proptest book: <https://proptest-rs.github.io/proptest/>
