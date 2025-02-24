# findfont-rs

[![Crates.io](https://img.shields.io/crates/v/findfont-rs)](https://crates.io/crates/findfont)
[![Docs.rs](https://img.shields.io/docsrs/findfont-rs)](https://docs.rs/findfont)

A cross-platform Rust library to find font files in system font directories.

## Features

- **Multi-platform support** (Linux/macOS/Windows)
- Searches standard system font directories
- Supports common font formats: TTF, TTC, OTF
- Automatic font variant detection (Light, Medium)
- Simple and intuitive API

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
findfont = "0.1"
```

## Usage

Basic usage:

```rust
use findfont::find;

fn main() {
    if let Some(path) = find("Arial") {
        println!("Found font at: {}", path.display());
    } else {
        println!("Font not found");
    }
}
```

With error handling:

```rust
use findfont::find;

fn load_font(font_name: &str) -> std::io::Result<Vec<u8>> {
    find(font_name)
        .map(std::fs::read)
        .unwrap_or_else(|| Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Font not found"
        )))
}
```

## Supported Platforms

### Linux

- `~/.fonts/`
- `~/.local/share/fonts/`
- `/usr/local/share/fonts/`
- `/usr/share/fonts/`
- XDG_DATA_HOME and XDG_DATA_DIRS locations

### macOS

- `~/Library/Fonts/`
- `/Library/Fonts/`
- `/System/Library/Fonts/`
- System supplemental fonts

### Windows

- `%windir%\Fonts`
- `%LOCALAPPDATA%\Microsoft\Windows\Fonts`

## Search Logic

1. Checks for exact filename matches with extensions in this order:
   - `.ttf`
   - `.ttc`
   - `.otf`
2. If no exact match found, checks for common variants:
   - `{font_name} Light.{ext}`
   - `{font_name} Medium.{ext}`
3. Returns first match found in system font directories

Note: Search is case-sensitive and uses platform-specific path separators

## Limitations

- Does not parse font files to verify actual font names
- Limited variant detection (only Light/Medium)
- First match in search order is returned
- Doesn't handle font collections (.ttc) differently

## Contributing

Contributions are welcome! Please open an issue or PR for:

- Additional font variants
- Improved search algorithms
- More comprehensive test coverage
- Platform-specific enhancements

## License

MIT/Apache
