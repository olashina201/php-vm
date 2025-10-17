# phpvm (Rust)

A cross-platform PHP version manager, NVM-style, written in Rust.

## Quickstart

```bash
# Build
cargo build --release

# Initialize PATH (POSIX)
eval "$(target/release/phpvm init)"

# List remote versions
phpvm list-remote --major 8.2

# Install
phpvm install 8.3.11

# Use for current shell
eval "$(phpvm use 8.3.11)"

# Set global
a phpvm global 8.3.11

# Which binary
phpvm which php
```

On Windows PowerShell:

```powershell
phpvm init powershell
# Follow the Import-Module instruction printed
```

## Notes
- On Unix, `install --from source` builds from official tarballs: `phpvm install 8.3.11 --from source`.
- You may need dependencies for `./configure` (see doctor).
- Windows supports `--ts` for thread-safe builds.
