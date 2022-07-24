# wxwarn

Display NOAA Weather Alerts For A Given Lat/Lon

Grabs the NOAA weather alerts shapefile, checks to see if
there are any alerts for the given coordinate, and prints
them if there are.

## Examples

### Rust

```rust
extern crate wxwarn;
print_alert(43.2683199, -70.8635506);
```

### Command line

```rust
$ wxwarn --lat="43.2683199" --lon="-70.8635506"
```

### Building

```rust
git clone git@github.com:hrbrmstr/wxwarn
cargo build --release
```

### Installing

The following will put:

- `wxwarn`

into `~/.cargo/bin` unless you've modified the behaviour of `cargo install`.

```rust
$ cargo install --git https://github.com/hrbrmstr/wxwarn
```

License: MIT
