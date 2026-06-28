# surgeist-style

Typed style model, resolution, and visual/layout property contracts for Surgeist.

## API Artifact

The committed API coordination artifact lives at `api/public-api.txt`, but the
generator is owned by the root `surgeist` repo.

Refresh this crate's artifact from the root repo with:

```sh
cargo run --manifest-path api/generator/Cargo.toml -- --crate surgeist-style
```

API refresh tooling is command-only and must not run as part of normal `cargo test`.
