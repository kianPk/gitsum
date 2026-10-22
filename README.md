# gitsum

**One glance at your git repo** — branch, dirty state, sync, top authors.

Tiny Rust binary. No config. No fuss.

[![Rust](https://img.shields.io/badge/Rust-1.70+-000000?logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Install

```bash
cargo install --git https://github.com/kianPk/gitsum
# or
git clone https://github.com/kianPk/gitsum.git && cd gitsum && cargo install --path .
```

## Usage

```bash
cd your-repo
gitsum
```

### Sample

```text
📦 forge-desk
   branch   main @ a1b2c3d
   commits  128
   last     2 hours ago · Ship workspace drawer
   sync     ↑0 ↓0
   dirty    staged=0 unstaged=2 untracked=1
   authors
            84  Kian Zarandi
            12  teammate
```

## License

MIT © [kianPk](https://github.com/kianPk)
