#![no_std]
use byteorder::{ByteOrder, LittleEndian};
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

    pub fn get_temperature(&mut self) -> Result<f32, Error<I2cError>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(
            Address::Dev as u8,
            &[Cmd::TemperatureReg as u8],
            &mut buffer,
        )?;
        Ok(convert_temperature(LittleEndian::read_u16(&buffer[0..2])))
    }

    pub fn get_voltage(&mut self) -> Result<u32, Error<I2cError>> {
        let mut buffer = [0u8; 2];
        self.i2c
            .write_read(Address::Dev as u8, &[Cmd::VoltageReg as u8], &mut buffer)?;
        Ok(LittleEndian::read_u32(&buffer[0..2]))
    }

    pub fn get_current(&mut self) -> Result<u32, Error<I2cError>> {
        let mut buffer = [0u8; 2];
        self.i2c
            .write_read(Address::Dev as u8, &[Cmd::CurrentReg as u8], &mut buffer)?;
        Ok(LittleEndian::read_u32(&buffer[0..2]))
    }

    pub fn get_cell_voltage_1(&mut self) -> Result<u32, Error<I2cError>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(
            Address::Dev as u8,
            &[Cmd::CellVoltage1Reg as u8],
            &mut buffer,
        )?;
        Ok(LittleEndian::read_u32(&buffer[0..2]))
    }

    pub fn get_cell_voltage_2(&mut self) -> Result<u32, Error<I2cError>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(
            Address::Dev as u8,
            &[Cmd::CellVoltage2Reg as u8],
            &mut buffer,
        )?;
        Ok(LittleEndian::read_u32(&buffer[0..2]))
    }

    pub fn get_cell_voltage_3(&mut self) -> Result<u32, Error<I2cError>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(
            Address::Dev as u8,
            &[Cmd::CellVoltage3Reg as u8],
            &mut buffer,
        )?;
        Ok(LittleEndian::read_u32(&buffer[0..2]))
    }

    pub fn get_cell_voltage_4(&mut self) -> Result<u32, Error<I2cError>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(
            Address::Dev as u8,
            &[Cmd::CellVoltage4Reg as u8],
            &mut buffer,
        )?;
        Ok(LittleEndian::read_u32(&buffer[0..2]))
    }
}

fn convert_temperature(raw: u16) -> f32 {
    raw as f32 / 10.0 - 273.15
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}
