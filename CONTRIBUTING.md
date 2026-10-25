# Contributing to gitsum

## Build

```bash
cargo build
cargo run
```

Requires `git` on your PATH.

## CI

`cargo build --release` and a smoke run of `gitsum` in the repo root.

## PRs

- Update `CHANGELOG.md`.
- Keep the binary dependency-free (stdlib + git subprocess only).
- Run `cargo fmt` before submitting.
