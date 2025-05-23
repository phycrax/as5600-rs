use num_derive::FromPrimitive;
#[cfg(test)]
use proptest_derive::Arbitrary;

mod conversion;
/// Errors
pub mod error;
#[cfg(test)]
mod test;

/// Power mode.
#[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum PowerMode {
    /// Normal power.
    Nom = 0b00,
    /// Low Power Mode 1.
    Lpm1 = 0b01,
    /// Low Power Mode 2.
    Lpm2 = 0b10,
    /// Low Power Mode 3.
    Lpm3 = 0b11,
}

/// Hysteresis mode.
#[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum Hysteresis {
    /// No hysteresis.
    Off = 0b00,
    /// 1 least significant bit hysteresis.
    Lsb1 = 0b01,
    /// 2 least significant bit hysteresis.
    Lsb2 = 0b10,
    /// 3 least significant bit hysteresis.
    Lsb3 = 0b11,
}

/// Output stage mode.
/// Apart from digital i2c output, analog modes or PWM could be generated by the module.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum OutputStage {
    /// Full-range analog output (0..VDD).
    Analog = 0b00,
    /// Reduced-range analog output (10% VDD..90% VDD).
    ReducedAnalog = 0b01,
    /// PWM output at frequency given by [`PwmFreq`].
    DigitalPwm = 0b10,
}

/// PWM frequency.
#[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum PwmFreq {
    /// Frequency 1: 116Hz.
    PwmF1 = 0b00,
    /// Frequency 2: 230Hz.
    PwmF2 = 0b01,
    /// Frequency 3: 460.
    PwmF3 = 0b10,
    /// Frequency 4: 920Hz.
    PwmF4 = 0b11,
}

impl PwmFreq {
    /// Get the frequency for this [`PwmFreq`] setting in Hz.
    pub const fn to_hz(&self) -> usize {
        match self {
            Self::PwmF1 => 115,
            Self::PwmF2 => 230,
            Self::PwmF3 => 460,
            Self::PwmF4 => 920,
        }
    }
}

/// Slow filter mode.
#[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum SlowFilterMode {
    /// 16x filter.
    X16 = 0b00,
    /// 8x filter.
    X8 = 0b01,
    /// 4x filter.
    X4 = 0b10,
    /// 2x filter.
    X2 = 0b11,
}

/// Fast filter threshold.
#[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum FastFilterThreshold {
    /// No fast filter.
    SlowFilterOnly = 0b000,
    /// Fast filter at 6 LSB.
    Lsb6 = 0b001,
    /// Fast filter at 7 LSB.
    Lsb7 = 0b010,
    /// Fast filter at 9 LSB.
    Lsb9 = 0b011,
    /// Fast filter at 18 LSB.
    Lsb18 = 0b100,
    /// Fast filter at 21 LSB.
    Lsb21 = 0b101,
    /// Fast filter at 24 LSB.
    Lsb24 = 0b110,
    /// Fast filter at 10 LSB.
    Lsb10 = 0b111,
}

/// Watchdog state.
#[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(test, derive(Arbitrary))]
#[repr(u8)]
pub enum WatchdogState {
    /// Watchdog off.
    Off = 0,
    /// Watchdog on.
    On = 1,
}

/// As5600 configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Configuration {
    /// Power mode.
    pub power_mode: PowerMode,
    /// Hysteresis.
    pub hysteresis: Hysteresis,
    /// Output stage.
    pub output_stage: OutputStage,
    /// PWM frequency.
    pub pwm_frequency: PwmFreq,
    /// Slow filter mode.
    pub slow_filter: SlowFilterMode,
    /// Fast filter threshold.
    pub fast_filter_threshold: FastFilterThreshold,
    /// Watchdog state.
    pub watchdog_state: WatchdogState,
}
