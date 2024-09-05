use ads111x_hal_derive::ConfigConversion;
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ADS111xConfig: u16 {
        const MUX_MASK = 0b111 << 12;
        const MUX_AIN0_AIN1 = 0b000 << 12;
        const MUX_AIN0_AIN3 = 0b001 << 12;
        const MUX_AIN1_AIN3 = 0b010 << 12;
        const MUX_AIN2_AIN3 = 0b011 << 12;
        const MUX_AIN0_GND = 0b100 << 12;
        const MUX_AIN1_GND = 0b101 << 12;
        const MUX_AIN2_GND = 0b110 << 12;
        const MUX_AIN3_GND = 0b111 << 12;

        const PGA_MASK = 0b111 << 9;
        const PGA_6_144V = 0b000 << 9;
        const PGA_4_096V = 0b001 << 9;
        const PGA_2_048V = 0b010 << 9;
        const PGA_1_024V = 0b011 << 9;
        const PGA_0_512V = 0b100 << 9;
        const PGA_0_256V = 0b101 << 9;

        const MODE_MASK = 1 << 8;
        const MODE_CONTINUOUS = 0 << 8;
        const MODE_SINGLE = 1 << 8;

        const DR_MASK = 0b111 << 5;
        const DR_8SPS = 0b000 << 5;
        const DR_16SPS = 0b001 << 5;
        const DR_32SPS = 0b010 << 5;
        const DR_64SPS = 0b011 << 5;
        const DR_128SPS = 0b100 << 5;
        const DR_250SPS = 0b101 << 5;
        const DR_475SPS = 0b110 << 5;
        const DR_860SPS = 0b111 << 5;

        const COMP_MODE_MASK = 1 << 4;
        const COMP_MODE_TRADITIONAL = 0 << 4;
        const COMP_MODE_WINDOW = 1 << 4;

        const COMP_POL_MASK = 1 << 3;
        const COMP_POL_ACTIVE_LOW = 0 << 3;
        const COMP_POL_ACTIVE_HIGH = 1 << 3;

        const COMP_LAT_MASK = 1 << 2;
        const COMP_LAT_NON_LATCHING = 0 << 2;
        const COMP_LAT_LATCHING = 1 << 2;

        const COMP_QUE_MASK = 0b11;
        const COMP_QUE_ASSERT_1 = 0b00;
        const COMP_QUE_ASSERT_2 = 0b01;
        const COMP_QUE_ASSERT_4 = 0b10;
        const COMP_QUE_DISABLE = 0b11;

        const OS_MASK = 1 << 15;
        const OS_BUSY = 0 << 15;
        const OS_NOT_BUSY = 1 << 15;
    }
}

bitflags! {
    #[derive(Default, Debug, Clone, Copy)]
    pub(crate) struct Register: u8 {
        const CONVERSION = 0b00;
        const CONFIG = 0b01;
        const LOW_THRESHOLD = 0b10;
        const HIGH_THRESHOLD = 0b11;
    }
}

impl Register {
    pub fn addr(self) -> u8 {
        self.bits()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, ConfigConversion)]
#[config_mask(ADS111xConfig::OS_MASK)]
pub enum OperationalStatus {
    #[config_flag(ADS111xConfig::OS_BUSY)]
    Busy,
    #[config_flag(ADS111xConfig::OS_NOT_BUSY)]
    NotBusy,
}

#[derive(Debug, Clone, Copy, ConfigConversion)]
#[config_mask(ADS111xConfig::MUX_MASK)]
pub enum InputMultiplexer {
    #[config_flag(ADS111xConfig::MUX_AIN0_AIN1)]
    AIN0AIN1,
    #[config_flag(ADS111xConfig::MUX_AIN0_AIN3)]
    AIN0AIN3,
    #[config_flag(ADS111xConfig::MUX_AIN1_AIN3)]
    AIN1AIN3,
    #[config_flag(ADS111xConfig::MUX_AIN2_AIN3)]
    AIN2AIN3,
    #[config_flag(ADS111xConfig::MUX_AIN0_GND)]
    AIN0GND,
    #[config_flag(ADS111xConfig::MUX_AIN1_GND)]
    AIN1GND,
    #[config_flag(ADS111xConfig::MUX_AIN2_GND)]
    AIN2GND,
    #[config_flag(ADS111xConfig::MUX_AIN3_GND)]
    AIN3GND,
}

#[derive(Debug, Clone, Copy, ConfigConversion)]
#[config_mask(ADS111xConfig::PGA_MASK)]
pub enum GainAmplifier {
    #[config_flag(ADS111xConfig::PGA_6_144V)]
    V6_144,
    #[config_flag(ADS111xConfig::PGA_4_096V)]
    V4_096,
    #[config_flag(ADS111xConfig::PGA_2_048V)]
    V2_048,
    #[config_flag(ADS111xConfig::PGA_1_024V)]
    V1_024,
    #[config_flag(ADS111xConfig::PGA_0_512V)]
    V0_512,
    #[config_flag(ADS111xConfig::PGA_0_256V)]
    V0_256,
}

impl GainAmplifier {
    pub fn voltage(&self) -> f32 {
        match self {
            GainAmplifier::V6_144 => 6.144,
            GainAmplifier::V4_096 => 4.096,
            GainAmplifier::V2_048 => 2.048,
            GainAmplifier::V1_024 => 1.024,
            GainAmplifier::V0_512 => 0.512,
            GainAmplifier::V0_256 => 0.256,
        }
    }
}

#[derive(Debug, Clone, Copy, ConfigConversion)]
#[config_mask(ADS111xConfig::MODE_MASK)]
pub enum Mode {
    #[config_flag(ADS111xConfig::MODE_CONTINUOUS)]
    Continuous,
    #[config_flag(ADS111xConfig::MODE_SINGLE)]
    Single,
}

#[derive(Debug, Clone, Copy, ConfigConversion)]
#[config_mask(ADS111xConfig::DR_MASK)]
pub enum DataRate {
    #[config_flag(ADS111xConfig::DR_8SPS)]
    SPS8,
    #[config_flag(ADS111xConfig::DR_16SPS)]
    SPS16,
    #[config_flag(ADS111xConfig::DR_32SPS)]
    SPS32,
    #[config_flag(ADS111xConfig::DR_64SPS)]
    SPS64,
    #[config_flag(ADS111xConfig::DR_128SPS)]
    SPS128,
    #[config_flag(ADS111xConfig::DR_250SPS)]
    SPS250,
    #[config_flag(ADS111xConfig::DR_475SPS)]
    SPS475,
    #[config_flag(ADS111xConfig::DR_860SPS)]
    SPS860,
}

#[derive(Debug, Clone, Copy, ConfigConversion)]
#[config_mask(ADS111xConfig::COMP_MODE_MASK)]
pub enum ComparatorMode {
    #[config_flag(ADS111xConfig::COMP_MODE_TRADITIONAL)]
    Traditional,
    #[config_flag(ADS111xConfig::COMP_MODE_WINDOW)]
    Window,
}

#[derive(Debug, Clone, Copy, ConfigConversion)]
#[config_mask(ADS111xConfig::COMP_POL_MASK)]
pub enum ComparatorPolarity {
    #[config_flag(ADS111xConfig::COMP_POL_ACTIVE_LOW)]
    ActiveLow,
    #[config_flag(ADS111xConfig::COMP_POL_ACTIVE_HIGH)]
    ActiveHigh,
}

#[derive(Debug, Clone, Copy, ConfigConversion)]
#[config_mask(ADS111xConfig::COMP_LAT_MASK)]
pub enum ComparatorLatching {
    #[config_flag(ADS111xConfig::COMP_LAT_NON_LATCHING)]
    NonLatching,
    #[config_flag(ADS111xConfig::COMP_LAT_LATCHING)]
    Latching,
}

#[derive(Debug, Clone, Copy, ConfigConversion)]
#[config_mask(ADS111xConfig::COMP_QUE_MASK)]
pub enum ComparatorQueue {
    #[config_flag(ADS111xConfig::COMP_QUE_ASSERT_1)]
    AsserAfterOne,
    #[config_flag(ADS111xConfig::COMP_QUE_ASSERT_2)]
    AsserAfterTwo,
    #[config_flag(ADS111xConfig::COMP_QUE_ASSERT_4)]
    AsserAfterFour,
    #[config_flag(ADS111xConfig::COMP_QUE_DISABLE)]
    Disable,
}

impl Default for ADS111xConfig {
    fn default() -> Self {
        ADS111xConfig::MUX_AIN0_AIN1
            | ADS111xConfig::PGA_2_048V
            | ADS111xConfig::MODE_SINGLE
            | ADS111xConfig::DR_128SPS
            | ADS111xConfig::COMP_MODE_TRADITIONAL
            | ADS111xConfig::COMP_POL_ACTIVE_LOW
            | ADS111xConfig::COMP_LAT_NON_LATCHING
            | ADS111xConfig::COMP_QUE_DISABLE
    }
}

impl ADS111xConfig {
    pub fn new() -> Self {
        ADS111xConfig::default()
    }

    pub fn with_multiplexer(mut self, mux: InputMultiplexer) -> Self {
        self.remove(Self::MUX_MASK);
        self.insert(mux.into());
        self
    }

    pub fn with_gain_amplifier(mut self, pga: GainAmplifier) -> Self {
        self.remove(Self::PGA_MASK);
        self.insert(pga.into());
        self
    }

    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.remove(Self::MODE_MASK);
        self.insert(mode.into());
        self
    }

    pub fn with_data_rate(mut self, dr: DataRate) -> Self {
        self.remove(Self::DR_MASK);
        self.insert(dr.into());
        self
    }

    pub fn with_comparator_mode(mut self, cm: ComparatorMode) -> Self {
        self.remove(Self::COMP_MODE_MASK);
        self.insert(cm.into());
        self
    }

    pub fn with_comparator_polarity(mut self, cp: ComparatorPolarity) -> Self {
        self.remove(Self::COMP_POL_MASK);
        self.insert(cp.into());
        self
    }

    pub fn with_comparator_latching(mut self, cl: ComparatorLatching) -> Self {
        self.remove(Self::COMP_LAT_MASK);
        self.insert(cl.into());
        self
    }

    pub fn with_comparator_queue(mut self, cq: ComparatorQueue) -> Self {
        self.remove(Self::COMP_QUE_MASK);
        self.insert(cq.into());
        self
    }

    pub(crate) fn with_operational_status(
        mut self,
        osr: OperationalStatus,
    ) -> Self {
        self.remove(Self::OS_MASK);
        self.insert(osr.into());
        self
    }

    pub fn gain_amplifier(&self) -> GainAmplifier {
        GainAmplifier::from(*self)
    }

    pub(crate) fn operational_status(&self) -> OperationalStatus {
        OperationalStatus::from(*self)
    }
}
