use display_interface::AsyncWriteOnlyDataCommand;

use crate::{
    displays::DisplayVariant,
    mode::displaymode::DisplayModeTrait,
    properties::DisplayProperties,
};

/// Raw display mode
pub struct RawMode<DV, DI>
where
    DI: AsyncWriteOnlyDataCommand,
{
    properties: DisplayProperties<DV, DI>,
}

impl<DV, DI> DisplayModeTrait<DV, DI> for RawMode<DV, DI>
where
    DI: AsyncWriteOnlyDataCommand,
{
    /// Create new RawMode instance
    fn new(properties: DisplayProperties<DV, DI>) -> Self {
        RawMode { properties }
    }

    /// Release all resources used by RawMode
    fn release(self) -> DisplayProperties<DV, DI> {
        self.properties
    }
}

impl<DV, DI: AsyncWriteOnlyDataCommand> RawMode<DV, DI>
where
    DV: DisplayVariant,
{
    /// Create a new raw display mode
    pub fn new(properties: DisplayProperties<DV, DI>) -> Self {
        RawMode { properties }
    }
}
