# batlens-core

Battery data library behind [batlens-cli](https://github.com/onlyatuna/batlens-cli). Windows only.

## Features

- Live battery state through Win32 ACPI/IOCTL (`sys`): charge, capacity, power, voltage,
  per-module data for multi-battery machines.
- Windows battery report (`powercfg /batteryreport`) parsing and 10-minute cache (`xml`).
- History cleaning, wear rate and battery replacement detection (`stats`).
- Design-capacity cache and report matching for modules that do not report a design capacity.
- Unknown values are `None` or `0`, never a guessed number.

## Usage

```toml
[dependencies]
batlens-core = { git = "https://github.com/onlyatuna/batlens-core" }
```

```rust
let s = batlens_core::sys::sample()?;
println!("{:?}%", s.percent);
```

## Test

```
cargo test
```

## License

MIT, see [LICENSE](LICENSE).
