use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c as BlockingI2c;

use crate::configuration::Configuration;
use crate::constants::DEFAULT_I2C_ADDRESS;
use crate::error::Error;
use crate::register::Register;
use crate::status::Status;

/// As5600 driver instance.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct As5600<I2C> {
    address: u8,
    bus: I2C,
}

impl<I, E> As5600<I>
where
    I: BlockingI2c<Error = E>,
{
    /// Create a new As5600 driver instance.
    pub fn new(bus: I) -> Self {
        Self::with_address(DEFAULT_I2C_ADDRESS, bus)
    }

    /// Create a new As5600 driver instance.
    pub fn with_address(address: u8, bus: I) -> Self {
        Self { address, bus }
    }

    /// Release the bus, consuming the driver.
    pub fn release(self) -> I {
        self.bus
    }

    /// Get value of register `RAW_ANGLE`.
    pub fn raw_angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::RawAngle)? & 0x0FFF)
    }

    /// Get value of register `ANGLE`.
    pub fn angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Angle)? & 0x0FFF)
    }

    /// Get value of register `ZMCO`.
    pub fn zmco(&mut self) -> Result<u8, Error<E>> {
        let mut buffer = [0u8; 1];
        self.bus
            .write_read(self.address, &[Register::Zmco.into()], &mut buffer)
            .map_err(Error::Communication)?;
        Ok(buffer[0] & 0b0000_0011)
    }

    /// Get value of register `STATUS`.
    pub fn magnet_status(&mut self) -> Result<Status, Error<E>> {
        let mut buffer = [0u8; 1];
        self.bus
            .write_read(self.address, &[Register::Status.into()], &mut buffer)
            .map_err(Error::Communication)?;
        Status::try_from(buffer).map_err(Error::Status)
    }

    /// Get value of register `ZPOS`.
    pub fn zero_position(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Zpos)? & 0x0FFF)
    }

    /// Set value of register `ZPOS`.
    pub fn set_zero_position(&mut self, bytes: u16) -> Result<(), Error<E>> {
        // 12-bit value.
        self.write_u16(Register::Zpos, bytes & 0x0FFF)
    }

    /// Get value of register `MPOS`.
    pub fn maximum_position(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Mpos)? & 0x0FFF)
    }

    /// Set value of register `MPOS`.
    pub fn set_maximum_position(&mut self, bytes: u16) -> Result<(), Error<E>> {
        // 12-bit value.
        self.write_u16(Register::Mpos, bytes & 0x0FFF)
    }

    /// Get value of register `MANG`.
    pub fn maximum_angle(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Mang)? & 0x0FFF)
    }

    /// Set value of register `MANG`.
    pub fn set_maximum_angle(&mut self, bytes: u16) -> Result<(), Error<E>> {
        // 12-bit value.
        self.write_u16(Register::Mang, bytes & 0x0FFF)
    }

    /// Get value of register `CONF` and parse it.
    pub fn config(&mut self) -> Result<Configuration, Error<E>> {
        let bytes = self.read_u16(Register::Conf)?;
        Configuration::try_from(bytes).map_err(Error::Configuration)
    }

    /// Set value of register `CONF`.
    pub fn set_config(&mut self, config: Configuration) -> Result<(), Error<E>> {
        // See note in datasheet about "blank fields may contain factory settings" on page 18.
        let current_config = self.read_u16(Register::Conf)?;
        let blank_fields = current_config & 0b1100_0000_0000_0000;
        let mut bytes = u16::from(config);
        bytes |= blank_fields;
        self.write_u16(Register::Conf, bytes)
    }

    /// Get value of register `AGC`.
    /// This value differs depending on the supply voltage (5V or 3v3), see datasheet.
    pub fn automatic_gain_control(&mut self) -> Result<u8, Error<E>> {
        let mut buffer = [0u8; 1];
        self.bus
            .write_read(self.address, &[0x1a], &mut buffer)
            .map_err(Error::Communication)?;
        Ok(buffer[0])
    }

    /// Get value of register `MAGNITUDE`.
    pub fn magnitude(&mut self) -> Result<u16, Error<E>> {
        // 12-bit value.
        Ok(self.read_u16(Register::Magnitude)? & 0x0FFF)
    }

    /// Burn maximum angle and config register.
    /// Only proceeds if position settings (MPOS and ZPOS) have never been persisted before.
    /// See datasheet for constraints.
    pub fn persist_maximum_angle_and_config_settings<D>(
        &mut self,
        delay: &mut D,
    ) -> Result<(), Error<E>>
    where
        D: DelayNs,
    {
        let zmco = self.zmco()?;
        if zmco != 0 {
            return Err(Error::MangConfigPersistenceExhausted);
        }
        self.bus
            .write(self.address, &[Register::Burn.into(), 0x40])
            .map_err(Error::Communication)?;
        delay.delay_ms(1);
        Ok(())
    }

    /// Burn zero position and maximum to As5600 memory, if ZMCO permits it and a magnet is detected.
    /// See datasheet for constraints.
    pub fn persist_position_settings<D>(&mut self, delay: &mut D) -> Result<(), Error<E>>
    where
        D: DelayNs,
    {
        let zmco = self.zmco()?;
        if zmco >= 3 {
            return Err(Error::MaximumPositionPersistsReached);
        }
        if self.magnet_status()? != Status::MagnetDetected {
            return Err(Error::MagnetRequired);
        }
        self.bus
            .write(self.address, &[Register::Burn.into(), 0x80])
            .map_err(Error::Communication)?;
        delay.delay_ms(1);
        Ok(())
    }

    /// Helper function for write-reading 2 bytes from the given register.
    fn read_u16(&mut self, command: Register) -> Result<u16, Error<E>> {
        let mut buffer = [0u8; 2];
        self.bus
            .write_read(self.address, &[command.into()], &mut buffer)
            .map_err(Error::Communication)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Helper function for writing 2 bytes to the given register.
    fn write_u16(&mut self, command: Register, bytes: u16) -> Result<(), Error<E>> {
        let bytes: [u8; 2] = bytes.to_be_bytes();
        let buffer = [u8::from(command), bytes[0], bytes[1]];
        self.bus
            .write(self.address, &buffer)
            .map_err(Error::Communication)
    }
}
