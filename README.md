# phpvm (Rust)

A cross-platform PHP version manager, NVM-style, written in Rust.

## Install

POSIX shells (bash/zsh):

```bash
curl -fsSL https://raw.githubusercontent.com/olashina201/php-vm/main/scripts/install.sh | bash
```

Using wget:

```bash
wget -qO- https://raw.githubusercontent.com/olashina201/php-vm/main/scripts/install.sh | bash
```

Pin a specific version (release tag):

```bash
PHPVM_VERSION=v0.1.0 \
bash -c "$(curl -fsSL https://raw.githubusercontent.com/olashina201/php-vm/main/scripts/install.sh)"
```

After install, restart your shell or run:

```bash
export PATH="$HOME/.phpvm/bin:$PATH"
phpvm doctor
```

On Windows, use the prebuilt archive from Releases or a PowerShell installer (coming soon).

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
