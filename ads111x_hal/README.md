# ADS111x Hal

> **Note**: This library is under active development. Breaking changes may occur in future minor releases.

This is a platform-agnostic Rust driver for the ADS1113, ADS1114, and ADS1115 analog-to-digital converters (ADCs), based on the [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.

## Features

- Supports ADS1113, ADS1114, and ADS1115 devices
- Configurable input multiplexer, gain amplifier, mode, data rate, and comparator settings
- Single-shot and continuous conversion modes
- Async support (optional feature)
- No-std compatible

## Example

```toml
[dependencies]
ads111x_hal = "0.1.0"
```

```rust
use ads111x::{ADS111x, ADS111xConfig, InputMultiplexer, GainAmplifier, Mode, DataRate};
let i2c = /* initialize your I2C bus */;

let config = ADS111xConfig::new()
    .with_multiplexer(InputMultiplexer::AIN0GND)
    .with_gain_amplifier(GainAmplifier::V4_096)
    .with_mode(Mode::Single)
    .with_data_rate(DataRate::SPS128);

let mut adc = ADS111x::new_and_configure(i2c, 0x48, config).await?;

let voltage = adc.read_single_voltage(None).await?;
println!("Voltage: {} V", voltage);

Ok(())
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.