# Release Maintenance

Run the strict release gate before publishing:

```sh
nix flake check --keep-going --print-build-logs
nix develop -c cargo package --list
nix develop -c cargo publish --dry-run
```

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
