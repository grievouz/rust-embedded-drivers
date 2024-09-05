#![no_std]

pub mod config;
pub mod error;

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

pub use crate::config::ADS111xConfig;
use crate::{config::*, error::*};

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")),),
    async(feature = "async"),
    keep_self
)]
pub struct ADS111x<I2C> {
    i2c: I2C,
    address: u8,
    config: ADS111xConfig,
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")),),
    async(feature = "async"),
    keep_self
)]
impl<I2C, E> ADS111x<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(
        i2c: I2C,
        address: u8,
        config: ADS111xConfig,
    ) -> Result<Self, ADSError<E>> {
        if (address & 0b1111100) != 0b1001000 {
            return Err(ADSError::<E>::WrongAddress);
        }
        Ok(ADS111x {
            i2c,
            address,
            config,
        })
    }

    pub async fn new_and_configure(
        i2c: I2C,
        address: u8,
        config: ADS111xConfig,
    ) -> Result<Self, ADSError<E>> {
        let mut ads = Self::new(i2c, address, config)?;
        ads.write_config().await?;
        Ok(ads)
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }

    pub async fn check_conversion_ready(&mut self) -> Result<bool, ADSError<E>> {
        let config = self.read_config().await?;
        Ok(config.operational_status() == OperationalStatus::NotBusy)
    }

    pub async fn read_config(&mut self) -> Result<ADS111xConfig, ADSError<E>> {
        let mut conf = [0, 0];
        self.i2c
            .write_read(self.address, &[Register::CONFIG.addr()], &mut conf)
            .await?;

        let config = ADS111xConfig::from_bits(u16::from_be_bytes(conf))
            .ok_or(ADSError::ConfigConversionError)?;

        Ok(config)
    }

    pub async fn read_single_voltage(
        &mut self,
        mux: Option<InputMultiplexer>,
    ) -> Result<f32, ADSError<E>> {
        if let Some(m) = mux {
            self.config = self.config.with_multiplexer(m);
        }

        // Tell the chip to start a conversion
        self.config = self.config.with_operational_status(OperationalStatus::Busy);
        self.write_config().await?;

        while !self.check_conversion_ready().await? {
            // Might want to add a small delay here to avoid busy-waiting
            // For now, we'll just continue looping
        }

        self.read_voltage().await
    }

    pub async fn read_voltage(&mut self) -> Result<f32, ADSError<E>> {
        let mut voltage = [0, 0];
        self.i2c
            .write_read(self.address, &[Register::CONVERSION.addr()], &mut voltage)
            .await?;
        let val = i16::from_be_bytes(voltage);
        let pga = self.config.gain_amplifier().voltage();

        Ok(f32::from(val) / i16::MAX as f32 * pga)
    }

    pub async fn set_config<F>(&mut self, f: F) -> Result<(), E>
    where
        F: FnOnce(ADS111xConfig) -> ADS111xConfig,
    {
        self.config = f(self.config);
        self.write_config().await
    }

    pub async fn write_config(&mut self) -> Result<(), E> {
        let conf = self.config.bits().to_be_bytes();
        self.i2c
            .write(self.address, &[Register::CONFIG.addr(), conf[0], conf[1]])
            .await
    }

    pub async fn write_low_treshold(
        &mut self,
        low_tresh: i16,
    ) -> Result<(), E> {
        let lt = low_tresh.to_be_bytes();
        self.i2c
            .write(self.address, &[Register::LOW_THRESHOLD.addr(), lt[0], lt[1]])
            .await
    }

    pub async fn write_high_treshold(
        &mut self,
        high_tresh: i16,
    ) -> Result<(), E> {
        let ht = high_tresh.to_be_bytes();
        self.i2c
            .write(self.address, &[Register::HIGH_THRESHOLD.addr(), ht[0], ht[1]])
            .await
    }
}
