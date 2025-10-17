# phpvm

phpvm is a fast, cross-platform PHP version manager written in Rust that lets you install, switch, and manage multiple PHP versions per project or globally. It provides lightweight shims and shell integration for seamless PATH management, supports prebuilt binaries and optional source builds, and includes commands for listing, installing/uninstalling, aliasing, executing with a specific version, and diagnosing your setup (doctor) on macOS, Linux, and Windows.

[![CI](https://github.com/olashina201/php-vm/workflows/CI/badge.svg)](https://github.com/olashina201/php-vm/actions)
[![Release](https://github.com/olashina201/php-vm/workflows/release/badge.svg)](https://github.com/olashina201/php-vm/releases)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 **Fast**: Built in Rust for maximum performance
- 🎯 **Simple**: NVM-style commands that PHP developers know and love
- 🔄 **Cross-platform**: Works on macOS, Linux, and Windows
- 📦 **Prebuilt binaries**: No compilation required
- 🛠️ **Source builds**: Install PHP from official source tarballs
- 🎨 **Shell integration**: Automatic PATH management
- 💾 **Caching**: Smart caching for faster installs
- 🔧 **Doctor**: Diagnose and fix common issues

## Installation

### Quick Install (Recommended)

**Bash/Zsh:**
```bash
curl -fsSL https://raw.githubusercontent.com/olashina201/php-vm/main/scripts/install.sh | bash
```

**Using wget:**
```bash
wget -qO- https://raw.githubusercontent.com/olashina201/php-vm/main/scripts/install.sh | bash
```

**Pin a specific version:**
```bash
PHPVM_VERSION=v0.1.0 \
bash -c "$(curl -fsSL https://raw.githubusercontent.com/olashina201/php-vm/main/scripts/install.sh)"
```

### Manual Installation

1. Download the latest release for your platform from [GitHub Releases](https://github.com/olashina201/php-vm/releases)
2. Extract the archive
3. Move the `phpvm` binary to a directory in your PATH (e.g., `/usr/local/bin`)
4. Run `phpvm init` to set up shell integration

### From Source

```bash
git clone https://github.com/olashina201/php-vm.git
cd php-vm
cargo build --release
sudo cp target/release/phpvm /usr/local/bin/
```

## Quick Start

After installation, restart your shell or run:

```bash
export PATH="$HOME/.phpvm/bin:$PATH"
phpvm doctor
```

## Usage

### Shell Integration

Initialize phpvm for your shell:

```bash
# Auto-detect shell
phpvm init

# Specify shell
phpvm init --shell bash
phpvm init --shell zsh
phpvm init --shell fish
phpvm init --shell powershell  # Windows
```

### Version Management

**List available PHP versions:**
```bash
# List all available versions
phpvm list-remote

# List versions for a specific major version
phpvm list-remote --major 8.2
phpvm list-remote --major 8.3
```

**List installed versions:**
```bash
phpvm list
```

**Install PHP:**
```bash
# Install latest 8.3.x
phpvm install 8.3

# Install specific version
phpvm install 8.3.11

# Install from source (Unix only)
phpvm install 8.3.11 --from source

# Install thread-safe build (Windows)
phpvm install 8.3.11 --ts
```

**Uninstall PHP:**
```bash
phpvm uninstall 8.3.11
```

### Version Switching

**Use a version for current shell session:**
```bash
# Use specific version
phpvm use 8.3.11

# Use latest 8.3.x
phpvm use 8.3
```

**Set global default version:**
```bash
phpvm global 8.3.11
```

**Set local version for a project:**
```bash
# In your project directory
phpvm local 8.3.11
# Creates .php-version file
```

### Executing Commands

**Run commands with specific PHP version:**
```bash
# Run PHP with specific version
phpvm exec 8.3.11 php --version

# Run Composer with specific PHP version
phpvm exec 8.3.11 composer install

# Run any command
phpvm exec 8.3.11 php artisan migrate
```

**Find PHP binary path:**
```bash
# Find current PHP binary
phpvm which php

# Find specific program
phpvm which composer
phpvm which pecl
```

### Aliases

**Create version aliases:**
```bash
# Create alias
phpvm alias latest 8.3.11
phpvm alias stable 8.2.25

# List aliases
phpvm alias

# Delete alias
phpvm alias --delete latest
```

**Use aliases:**
```bash
# Use alias
phpvm use latest
phpvm global stable
```

### Cache Management

**Clean up cache:**
```bash
phpvm cache prune
```

### Self-Update

**Update phpvm itself:**
```bash
phpvm self-update
```

### Troubleshooting

**Diagnose issues:**
```bash
phpvm doctor
```

This will check:
- Installation paths
- Shell integration
- Required dependencies
- Common configuration issues

## Examples

### Complete Workflow

```bash
# 1. Install phpvm
curl -fsSL https://raw.githubusercontent.com/olashina201/php-vm/main/scripts/install.sh | bash

# 2. Restart shell or source profile
source ~/.bashrc  # or ~/.zshrc

# 3. Check installation
phpvm doctor

# 4. List available versions
phpvm list-remote --major 8.3

# 5. Install PHP 8.3.11
phpvm install 8.3.11

# 6. Set as global default
phpvm global 8.3.11

# 7. Verify installation
php --version

# 8. Create project with specific version
mkdir my-project
cd my-project
phpvm local 8.2.25
echo "<?php phpinfo(); ?>" > index.php

# 9. Run with project's PHP version
php index.php
```

### Development Workflow

```bash
# Switch between PHP versions for different projects
cd project-a
phpvm local 8.3.11
php artisan serve

cd ../project-b  
phpvm local 8.2.25
composer install
phpunit

# Use specific version for one command
phpvm exec 8.1.30 php -m | grep -i curl
```

### CI/CD Integration

```bash
# In your CI script
phpvm install 8.3.11
phpvm use 8.3.11
composer install
phpunit
```

## Configuration

### Environment Variables

- `PHPVM_ROOT`: Override default installation directory (default: `~/.phpvm`)
- `PHPVM_CACHE_DIR`: Override cache directory (default: `~/.phpvm/cache`)

### File Locations

- **Installation root**: `~/.phpvm/`
- **PHP versions**: `~/.phpvm/versions/`
- **Shims**: `~/.phpvm/shims/`
- **Cache**: `~/.phpvm/cache/`
- **Global version**: `~/.phpvm/global`
- **Local version**: `.php-version` (in project directory)

## Shell Integration

### Bash/Zsh

Add to your `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.phpvm/bin:$PATH"
eval "$(phpvm init)"
```

### Fish

Add to your `~/.config/fish/config.fish`:
```fish
set -gx PATH $HOME/.phpvm/bin $PATH
phpvm init | source
```

### PowerShell (Windows)

Add to your PowerShell profile:
```powershell
# Run once to get the import command
phpvm init powershell
# Then add the printed Import-Module command to your profile
```

## Requirements

### Unix (macOS/Linux)

- `curl` or `wget` for downloading
- `tar` for extracting archives
- `gcc` and `make` for source builds
- Standard build tools (`autoconf`, `libtool`, etc.)

### Windows

- PowerShell 5.1+ or PowerShell Core
- Visual Studio Build Tools (for source builds)

## Troubleshooting

### Common Issues

**"phpvm: command not found"**
```bash
# Add to PATH
export PATH="$HOME/.phpvm/bin:$PATH"
# Or restart your shell
```

**"Permission denied"**
```bash
# Fix permissions
chmod +x ~/.phpvm/bin/phpvm
```

**Shell integration not working**
```bash
# Re-initialize
phpvm init
# Or manually add to your shell config
eval "$(phpvm init)"
```

**PHP not found after install**
```bash
# Check installation
phpvm doctor
# Reinstall shims
phpvm use 8.3.11
```

### Getting Help

- Run `phpvm doctor` to diagnose issues
- Check the [Issues](https://github.com/olashina201/php-vm/issues) page
- Review the [CI logs](https://github.com/olashina201/php-vm/actions) for build issues

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
git clone https://github.com/olashina201/php-vm.git
cd php-vm
cargo build
cargo test
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Inspired by [NVM](https://github.com/nvm-sh/nvm) and [RVM](https://rvm.io/)
- Built with [Rust](https://www.rust-lang.org/) for performance and reliability
- Uses [clap](https://github.com/clap-rs/clap) for command-line parsing