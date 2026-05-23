# Compatibility Notes

The compatibility helpers are designed for dataset metadata workflows.

The current rules are:

- Public-domain dedications and markers (`CC0`, `PDDL`, `PDM`) are compatible with every supported license.
- Licenses that allow commercial use are incompatible with non-commercial licenses.
- Share-alike obligations are family-local. If either side requires share-alike and the two licenses are from different families, the pair is incompatible.
- If both licenses require share-alike and are from the same family, they are compatible only when they are the same variant. This crate conservatively rejects cross-version and externally declared share-alike compatibility.
- Otherwise, licenses in the same commercial-use class are compatible.
- `most_restrictive(a, b)` returns the stricter compatible license, or `None` for incompatible pairs.

The implementation is intentionally conservative and keeps a few invariants stable:

- Compatibility is reflexive: every supported license is compatible with itself.
- Compatibility is symmetric: if `a` is compatible with `b`, then `b` is compatible with `a`.
- Public-domain licenses and markers behave as absorbing permissive cases for compatibility checks.
- `most_restrictive(a, b).is_some()` agrees with `a.is_compatible_with(&b)`.

For example, `CC-BY-SA-4.0` and `ODbL-1.0` both use share-alike terms, but they come from different license families. `DataLicense::CcBySa.is_compatible_with(&DataLicense::OdcOdbl)` therefore returns `false`. The same family-aware rule rejects combining a non-share-alike Creative Commons license such as `CC-BY-4.0` with `ODbL-1.0`, because ODbL's share-alike terms would still govern the resulting derivative database.

The model intentionally stays conservative. Creative Commons has a formal ShareAlike compatibility process for BY-SA, and ODbL has its own compatible-license mechanism. This crate does not encode those external compatibility determinations; it only provides metadata-level checks for the supported enum variants.

These checks do not replace legal review. Systems should also surface `spdx_id()` and `rights_uri()` so users can inspect the actual license terms.
