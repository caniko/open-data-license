# Release Maintenance

Run the strict release gate before publishing:

```sh
nix flake check --keep-going --print-build-logs
nix develop -c cargo build --no-default-features
nix develop -c cargo build --all-features
nix develop -c cargo test --no-default-features
nix develop -c cargo test --all-features
nix develop -c cargo package --list
nix develop -c cargo publish --dry-run
```

These checks enforce the crate's intended packaging and compatibility shape:

- `--no-default-features` proves the base crate does not require `utoipa`.
- `--all-features` proves the optional schema integration still compiles and documents correctly.
- `cargo package --list` confirms release artifacts include the docs-facing files and examples shipped in `Cargo.toml`.

The Codeberg repository is public and uses Forgejo Actions workflows under `.forgejo/workflows/`. CI jobs are written for the self-hosted `atlas` runner.

Before a Codeberg workflow can run, enable Actions in the repository UI:

```text
Settings -> Units -> Enable Actions
```

Actual crates.io publication remains human-in-the-loop:

```sh
cargo login
cargo publish
```

## Documentation Maintenance

When the public surface changes, update the stable docs in the same change:

- Add or remove license variants in [Introduction](introduction.md) and [API Overview](api-overview.md).
- Revisit [Compatibility Notes](compatibility.md) whenever `is_compatible_with` or `most_restrictive` semantics change.
- Keep [Getting Started](getting-started.md) aligned with the current crate version, optional features, and the newest trustworthy Git reference you want users to copy.
- Do not create or revive a planning chapter for work that has already become shipped behavior. Fold enduring guidance into the stable pages above instead.
