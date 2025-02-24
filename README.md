# findfont-rs

Fast font finding by match file path directly for windows/mac/linux.

## Usage

`cargo add findfont`

```rust
if let Some(font_path) = findfont::find("PingFang") {
  println!("{font_path:?}");
}
```
