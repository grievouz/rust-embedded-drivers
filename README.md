# ADS111x 16bit ADCs I2C rust driver
Tested on ESP32 and ADS1115
[Documentation](https://www.ti.com/product/ADS1115)

## Example
```rust
let i2c_speed = RateExtU32::kHz(100);

let i2c = I2C::new(peripherals.I2C0, io.pins.gpio19, io.pins.gpio23, i2c_speed, &mut system.peripheral_clock_control, &clocks);

let config = ADS111xConfig::default()
    .mux(ads111x::InputMultiplexer::AIN0GND)
    .dr(ads111x::DataRate::SPS8)
    .pga(ads111x::ProgramableGainAmplifier::V4_096);

let mut adc = match ADS111x::new(i2c, 0x48u8, config){
    Err(e) => panic!("Error {:?}", e),
    Ok(x) => x,
};

match adc.read_single_voltage(None){
    Ok(v) => println!("Val single {} {:.3}", v, f32::from(v) * 4.096f32 / f32::from(i16::MAX)),
    Err(e) => println!("Error {:?}", e),
}
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.