#![no_std]
use embedded_hal::blocking::i2c::WriteRead;

/// I2C address
#[derive(Copy, Clone)]
pub enum Address {
    /// Device address
    Dev = 0x0B,
    /// Register of MAC
    Mac = 0x44,
}

// Read word
#[derive(Copy, Clone)]
pub enum Cmd {
    TemperatureReg = 0x08,
    VoltageReg = 0x09,
    CurrentReg = 0x0A,
    AverageCurrentReg = 0x0B,
    MaxErrorReg = 0x0C,
    RelativeSocReg = 0x0D,
    AbsoluteSocReg = 0x0E,
    RemainingCapacityReg = 0x0F,
    FullChargeCapacityReg = 0x10,
    ChargingCurrentReg = 0x14,
    ChargingVoltageReg = 0x15,
    BatteryStatusReg = 0x16,
    CycleCountReg = 0x17,
    CellVoltage4Reg = 0x3C,
    CellVoltage3Reg = 0x3D,
    CellVoltage2Reg = 0x3E,
    CellVoltage1Reg = 0x3F,
    SohReg = 0x4F,
}

// Read word
#[derive(Copy, Clone)]
pub enum CmdBlock {
    DEVICENAMEReg = 0x21,
}

#[derive(Clone, Copy, Debug)]
pub enum Error<I2cError> {
    I2cError(I2cError),
}

pub struct BQ40Z50<I2C> {
    i2c: I2C,
}

impl<I2C, I2cError> BQ40Z50<I2C>
where
    I2C: WriteRead<Error = I2cError>,
{
    pub fn new(i2c: I2C) -> Result<BQ40Z50<I2C>, Error<I2cError>> {
        let bq40z50 = BQ40Z50 { i2c: i2c };
        Ok(bq40z50)
    }

    fn get_voltage(&mut self) -> Result<u8, Error<I2cError>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(Address::Dev as u8, &[Cmd::VoltageReg as u8], &mut buffer)?;
        Ok(buffer[0])
    }
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
 }
 