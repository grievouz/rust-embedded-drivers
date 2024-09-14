//! Operating modes for the oled_async
//!
//! This driver can be used in different modes. A mode defines how the driver
//! will behave, and what methods it exposes. Look at the modes below for more
//! information on what they expose.

use display_interface::{AsyncWriteOnlyDataCommand, DisplayError};

pub mod sh1106;
pub mod sh1107;
pub mod sh1108;
pub mod ssd1309;

pub trait DisplayVariant {
    /// Width of display
    const WIDTH: u8;
    /// Height of display
    const HEIGHT: u8;
    /// Coumn offset
    const COLUMN_OFFSET: u8 = 0;
    /// Large Page AddressP
    const LARGE_PAGE_ADDRESS: bool = false;

    /// Get integral dimensions from DisplaySize
    fn dimensions() -> (u8, u8) {
        (Self::WIDTH, Self::HEIGHT)
    }

    /// Initialise the display for column mode
    #[allow(async_fn_in_trait)]
    async fn init_column_mode<DI>(iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: AsyncWriteOnlyDataCommand;
}

pub trait ScreenSize {}

pub struct Screen128x64;
impl ScreenSize for Screen128x64 {}

pub struct Screen64x128;
impl ScreenSize for Screen64x128 {}

pub struct Screen128x128;
impl ScreenSize for Screen128x128 {}

pub struct Screen64x160;
impl ScreenSize for Screen64x160 {}

pub struct Screen96x160;
impl ScreenSize for Screen96x160 {}

pub struct Screen128x160;
impl ScreenSize for Screen128x160 {}

pub struct Screen160x160;
impl ScreenSize for Screen160x160 {}
