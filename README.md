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

```shell
$ wxwarn --lat="43.2683199" --lon="-70.8635506"
```

```plain
Heat Advisory issued July 22 at 2:51PM EDT until July 24 at 8:00PM EDT by NWS Gray ME

* WHAT...Heat index values into the middle to upper 90s.

* WHERE...In New Hampshire, Western and Central Hillsborough,
Belknap, Merrimack, Strafford and Southern Carroll Counties. In
Maine, Central Interior Cumberland, Interior York and Interior
Cumberland Highlands Counties.

* WHEN...From 11 AM to 8 PM EDT Sunday.

* IMPACTS...Hot temperatures and high humidity may cause heat
illnesses.

* ADDITIONAL DETAILS...Overnight low temperatures will only fall
into the lower to middle 70s on Sunday night, which will lead to
cumulative heat impacts to non-air conditioned buildings.

Drink plenty of fluids, stay in an air-conditioned room, stay out of
the sun, and check up on relatives and neighbors.

Interior York; Central Interior Cumberland; Interior Cumberland Highlands; Southern Carroll; Merrimack; Belknap; Strafford; Western And Central Hillsborough

================================

Heat Advisory issued July 24 at 10:20AM EDT until July 24 at 8:00PM EDT by NWS Gray ME

* WHAT...Heat index values up to 99 expected.

* WHERE...Portions of south central, southwest, and western Maine.
Portions of central, northern, and southern New Hampshire.

* WHEN...Until 8 PM EDT this evening.

* IMPACTS...Hot temperatures and high humidity may cause heat
illnesses.

Drink plenty of fluids, stay in an air-conditioned room, stay out of
the sun, and check up on relatives and neighbors.

Southern Oxford; Southern Franklin; Southern Somerset; Interior York; Central Interior Cumberland; Androscoggin; Kennebec; Interior Waldo; Interior Cumberland Highlands; Southern Carroll; Merrimack; Belknap; Strafford; Coastal Rockingham; Western And Central Hillsborough
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
