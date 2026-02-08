# weathr

A terminal weather app with ASCII animations.

## Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [License](#license)

## Installation

### Build

You need Rust installed.

```bash
git clone https://github.com/veirt/weathr.git
cd weathr
cargo install --path .
```

## Configuration

The config file is at `~/.config/weathr/config.toml`.

### Setup

```bash
mkdir -p ~/.config/weathr
```

Edit `~/.config/weathr/config.toml`:

```toml
[location]
latitude = 52.52
longitude = 13.41
```

## Usage

Run:

```bash
weathr
```

## License

GPL-3.0-or-later
