# Compatibility Notes

The compatibility helpers are designed for dataset metadata workflows.

The current rules are:

- Public-domain dedications (`CC0`, `PDDL`) are compatible with every supported license.
- Licenses that allow commercial use are incompatible with non-commercial licenses.
- Share-alike licenses are treated as compatible with matching share-alike restrictiveness bands.
- `most_restrictive(a, b)` returns the stricter compatible license, or `None` for incompatible pairs.

These checks do not replace legal review. Systems should also surface `spdx_id()` and `rights_uri()` so users can inspect the actual license terms.
