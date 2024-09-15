use core::marker::PhantomData;

use display_interface::{AsyncWriteOnlyDataCommand, DisplayError};

use super::{
    DisplayVariant,
    Screen128x160,
    Screen160x160,
    Screen64x160,
    Screen96x160,
};
use crate::command::{Command, VcomhLevel};

/// Generic 64x160 with SH1108 controller
#[derive(Debug, Clone, Copy)]
pub struct Sh1108<SS> {
    _ss: PhantomData<SS>,
}

impl DisplayVariant for Sh1108<Screen64x160> {
    const WIDTH: u8 = 64;
    const HEIGHT: u8 = 160;
    const COLUMN_OFFSET: u8 = 48;
    const LARGE_PAGE_ADDRESS: bool = true;

    async fn init_column_mode<DI>(iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: AsyncWriteOnlyDataCommand,
    {
        init_column_mode_common(iface, Self::dimensions(), 0).await?;
        Command::DisplayOffset(0).send(iface).await?;
        Command::ComPinConfig(true).send(iface).await?;

        Ok(())
    }
}

impl DisplayVariant for Sh1108<Screen96x160> {
    const WIDTH: u8 = 96;
    const HEIGHT: u8 = 160;
    const COLUMN_OFFSET: u8 = 32;
    const LARGE_PAGE_ADDRESS: bool = true;

    async fn init_column_mode<DI>(iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: AsyncWriteOnlyDataCommand,
    {
        init_column_mode_common(iface, Self::dimensions(), 1).await?;
        Command::DisplayOffset(0).send(iface).await?;
        Command::ComPinConfig(true).send(iface).await?;

        Ok(())
    }
}

impl DisplayVariant for Sh1108<Screen128x160> {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 160;
    const COLUMN_OFFSET: u8 = 16;
    const LARGE_PAGE_ADDRESS: bool = true;

    async fn init_column_mode<DI>(iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: AsyncWriteOnlyDataCommand,
    {
        init_column_mode_common(iface, Self::dimensions(), 2).await?;
        Command::DisplayOffset(0).send(iface).await?;
        Command::ComPinConfig(true).send(iface).await?;

        Ok(())
    }
}

impl DisplayVariant for Sh1108<Screen160x160> {
    const WIDTH: u8 = 160;
    const HEIGHT: u8 = 160;
    const COLUMN_OFFSET: u8 = 0;
    const LARGE_PAGE_ADDRESS: bool = true;

    async fn init_column_mode<DI>(
        iface: &mut DI,
        //display_rotation: DisplayRotation,
    ) -> Result<(), DisplayError>
    where
        DI: AsyncWriteOnlyDataCommand,
    {
        init_column_mode_common(iface, Self::dimensions(), 3).await?;
        Command::DisplayOffset(0).send(iface).await?;
        Command::ComPinConfig(true).send(iface).await?;

        Ok(())
    }
}

/// Initialise the display in column mode (i.e. a byte walks down a column of 8
/// pixels) with column 0 on the left and column _(display_width - 1)_ on the
/// right.
async fn init_column_mode_common<DI>(
    iface: &mut DI,
    dimensions: (u8, u8),
    resolution: u8,
) -> Result<(), DisplayError>
where
    DI: AsyncWriteOnlyDataCommand,
{
    //iface.init().await?;
    // TODO: Break up into nice bits so display modes can pick whathever they
    // need
    let (_, display_height) = dimensions;

    Command::DisplayOn(false).send(iface).await?;
    Command::DisplayClockDiv(0x6, 0x0).send(iface).await?;
    Command::DisplayResolution(resolution).send(iface).await?;
    Command::PreChargePeriod(0x8, 0x2).send(iface).await?;
    Command::DisplayOn(true).send(iface).await?;

    Command::Multiplex(display_height - 1).send(iface).await?;

    Command::StartLine(0).send(iface).await?;
    // TODO: Ability to turn charge pump on/off
    // Display must be off when performing this command
    Command::ChargePump(true).send(iface).await?;

    Command::Contrast(0x80).send(iface).await?;
    Command::PreChargePeriod(0x1, 0xF).send(iface).await?;
    Command::VcomhDeselect(VcomhLevel::Auto).send(iface).await?;
    Command::AllOn(false).send(iface).await?;
    Command::Invert(false).send(iface).await?;
    Command::DisplayOn(true).send(iface).await?;

    Ok(())
}
