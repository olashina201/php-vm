# Codebase Structure

This document explains the main files and modules in the phpvm repository and what each is responsible for.

```
.
├─ Cargo.toml
├─ README.md
├─ docs/
│  ├─ REQUIREMENTS.md
│  └─ CODEBASE.md
├─ src/
│  ├─ main.rs
│  ├─ cli.rs
│  ├─ dirs.rs
│  ├─ config.rs
│  ├─ manifest.rs
│  ├─ downloader.rs
│  ├─ archive.rs
│  ├─ resolver.rs
│  ├─ shims.rs
│  ├─ aliases.rs
│  ├─ build_unix.rs
│  └─ self_update_cmd.rs
├─ tests/
│  └─ basic.rs
└─ .github/workflows/ci.yml
```

## Top-level
- `Cargo.toml`: Rust package manifest; dependencies and build profiles.
- `README.md`: Quickstart usage and platform notes.
- `docs/REQUIREMENTS.md`: Detailed implemented features and behaviors.
- `docs/CODEBASE.md`: This file.
- `.github/workflows/ci.yml`: GitHub Actions CI to build and test on macOS, Linux, and Windows.

## Source modules
- `src/main.rs`: Program entrypoint; wires modules and launches the async CLI runner.
- `src/cli.rs`: Clap-based CLI definition and command handlers. Implements:
  - list-remote, list, install, uninstall, use, global, local, which, exec, alias, cache prune, doctor, init, self-update.
  - Calls into other modules to perform actions.
- `src/dirs.rs`: Cross-platform user directory layout under `~/.phpvm` and helpers to ensure/create directories.
- `src/config.rs`: Load/save user config (`config.toml`), with defaults (CPU jobs, flags placeholder).
- `src/manifest.rs`: Remote versions manifest (stub). Replace with a real fetcher later.
- `src/downloader.rs`: HTTP downloader with resume support and optional SHA256 verification.
- `src/archive.rs`: Archive extraction for `.tar.xz`, `.tar.gz`, and `.zip` into destination directories.
- `src/resolver.rs`: Version resolution logic (env `PHPVM_VERSION`, `.php-version`, global file) and writers for `global` and local `.php-version`.
- `src/shims.rs`: Creates shims for `php`, `pecl`, `pear`, `composer`; resolves the real executable for `which`.
- `src/aliases.rs`: Load/save aliases from `aliases.toml` and alias resolution.
- `src/build_unix.rs`: Unix source build pipeline: `./configure`, `make -j`, `make install` into target version directory.
- `src/self_update_cmd.rs`: Self-update implementation using GitHub Releases via `self_update` crate.

## Tests
- `tests/basic.rs`: Minimal test ensuring layout creation succeeds.

## How modules interact (high-level flow)
- The CLI parses user commands and uses `dirs` to ensure the layout exists.
- For `install`, `downloader` fetches archives, `archive` extracts, or `build_unix` builds from source on Unix.
- `resolver` and `aliases` decide what version to target; `shims` provides executable resolution for `which`/`exec`.
- `init` prints shell snippets and (on Windows) writes a PowerShell module file.
- `self_update_cmd` enables in-place binary updates when releases are configured.
