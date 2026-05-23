# Contributing

Contributions are welcome on the primary Codeberg repository. Open pull requests against the `trunk` branch at:

<https://codeberg.org/caniko/open-data-license>

Mirrors on other forges are read-only. Please do not open issues or pull requests on mirrors unless the mirror explicitly points back to Codeberg for project coordination.

## Running Checks

The full release gate is:

```sh
nix flake check --keep-going --print-build-logs
```

For a faster local test pass inside the development shell:

```sh
nix develop -c cargo test --all-features
```

Before proposing a release-facing change, also check the package contents:

```sh
nix develop -c cargo package --list
nix develop -c cargo publish --dry-run
```

## License And Legal Questions

This crate models published license metadata and conservative compatibility helpers. It does not define the legal meaning of SPDX identifiers, Creative Commons terms, Open Data Commons terms, or CDLA terms.

Questions about license text, legal interpretation, identifier registration, or canonical license metadata should be raised with the upstream license steward, such as SPDX or Creative Commons, rather than as issues in this repository.
