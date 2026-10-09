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

CI and crate publication use GitHub Actions workflows under `.github/workflows/`
on GitHub-hosted `ubuntu-24.04` runners. Inspect the repository's Actions tab for
CI and crate-release logs. Documentation publication remains separate:
`.forgejo/workflows/pages.yaml` runs on `atlas` after pushes to `trunk` and
publishes the site to Codeberg Pages. Inspect Forgejo Actions for Pages failures.

Actual crates.io publication is initiated by a maintainer pushing a signed
unprefixed `<version>` tag, such as `0.2.1`. The generated `publish-crate.yaml`
workflow requires the `CRATES_IO_API_TOKEN` repository secret and imports the
pinned public keys from the default branch's `keys/maintainers.gpg` to verify
the tag; no release-key secrets are consumed. The verified tag's peeled commit
must match the immutable event checkout before package metadata is evaluated.
Retry the original tag-push run rather than dispatching a branch. Before tagging,
complete the release gate above and ensure
the tag version matches `Cargo.toml`. Never use the CI workflow as evidence that
a publication succeeded: confirm the release job and the crates.io version.

Regenerate or check the GitHub workflows with the qualified Simit revision,
preserving the repository's existing public trust root:

```sh
SIMIT_MAINTAINERS_GPG="$PWD/keys/maintainers.gpg" nix run github:caniko/simit/0cab0eb028305a2bea3573abe3d61c85489e8a68 -- init ci --check --diff
```

Omit `--check --diff` to regenerate. Review the resulting changes; do not edit
generated workflows by hand or replace maintainer keys from local Git settings.

## Documentation Maintenance

When the public surface changes, update the stable docs in the same change:

- Add or remove license variants in [Introduction](introduction.md) and [API Overview](api-overview.md).
- Revisit [Compatibility Notes](compatibility.md) whenever `is_compatible_with` or `most_restrictive` semantics change.
- Keep [Getting Started](getting-started.md) aligned with the current crate version, optional features, and the newest trustworthy Git reference you want users to copy.
- Do not create or revive a planning chapter for work that has already become shipped behavior. Fold enduring guidance into the stable pages above instead.
