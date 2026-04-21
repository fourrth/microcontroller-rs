//! https://github.com/pololu/drv8434s-arduino

/// Addresses of control and status registers.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterAddress {
    Fault = 0x00,
    Diag1 = 0x01,
    Diag2 = 0x02,
    Ctrl1 = 0x03,
    Ctrl2 = 0x04,
    Ctrl3 = 0x05,
    Ctrl4 = 0x06,
    Ctrl5 = 0x07,
    Ctrl6 = 0x08,
    Ctrl7 = 0x09,
    Ctrl8 = 0x0A,
    Ctrl9 = 0x0B,
}

/// Bits that are set in the return value of `read_fault()` to indicate warning and
/// fault conditions.
///
/// See the DRV8434S datasheet for detailed descriptions of these conditions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultBit {
    /// Fault indication (0 when nFAULT pin is high, 1 when nFAULT pin is low)
    Fault = 7,
    /// SPI protocol error (latched)
    SpiError = 6,
    /// Supply undervoltage lockout fault
    Uvlo = 5,
    /// Charge pump undervoltage fault
    Cpuv = 4,
    /// Overcurrent fault
    Ocp = 3,
    /// Motor stall
    Stl = 2,
    /// Overtemperature warning or shutdown
    Tf = 1,
    /// Open load
    Ol = 0,
}

/// Bits that are set in the return value of `read_diag1()` to indicate warning and
/// fault conditions.
///
/// See the DRV8434S datasheet for detailed descriptions of these conditions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diag1Bit {
    /// Overcurrent fault on low-side FET of half bridge 2 in BOUT
    OcpLs2B = 7,
    /// Overcurrent fault on high-side FET of half bridge 2 in BOUT
    OcpHs2B = 6,
    /// Overcurrent fault on low-side FET of half bridge 1 in BOUT
    OcpLs1B = 5,
    /// Overcurrent fault on high-side FET of half bridge 1 in BOUT
    OcpHs1B = 4,
    /// Overcurrent fault on low-side FET of half bridge 2 in AOUT
    OcpLs2A = 3,
    /// Overcurrent fault on high-side FET of half bridge 2 in AOUT
    OcpHs2A = 2,
    /// Overcurrent fault on low-side FET of half bridge 1 in AOUT
    OcpLs1A = 1,
    /// Overcurrent fault on high-side FET of half bridge 1 in AOUT
    OcpHs1A = 0,
}

/// Bits that are set in the return value of `read_diag2()` to indicate warning and
/// fault conditions.
///
/// See the DRV8434S datasheet for detailed descriptions of these conditions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diag2Bit {
    /// Overtemperature warning
    Otw = 6,
    /// Overtemperature shutdown
    Ots = 5,
    /// Stall detection learning successful
    StlLrnOk = 4,
    /// Motor stall condition
    Stall = 3,
    /// Open load on BOUT
    OlB = 1,
    /// Open load on AOUT
    OlA = 0,
}

/// Possible arguments to `set_decay_mode()`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecayMode {
    Slow = 0b000,
    IncSlowDecMixed30 = 0b001,
    IncSlowDecMixed60 = 0b010,
    IncSlowDecFast = 0b011,
    Mixed30 = 0b100,
    Mixed60 = 0b101,
    SmartTuneDynamicDecay = 0b110,
    SmartTuneRippleControl = 0b111,
}

/// Possible arguments to `set_step_mode()`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepMode {
    /// Full step with 100% current
    MicroStep1_100 = 0b0000,
    /// Full step with 71% current
    MicroStep1 = 0b0001,
    /// Non-circular 1/2 step
    MicroStep2_NC = 0b0010,
    /// Circular 1/2 step
    MicroStep2 = 0b0011,
    MicroStep4 = 0b0100,
    MicroStep8 = 0b0101,
    MicroStep16 = 0b0110,
    MicroStep32 = 0b0111,
    MicroStep64 = 0b1000,
    MicroStep128 = 0b1001,
    MicroStep256 = 0b1010,
}

pub fn spi_data_address(data: u8, address: u8, is_read: bool) -> u16 {
    // data is a full byte
    let data_ = data as u16;
    // address is only 5 bits
    let address_ = (address & 0b00011111) as u16;

    (data_ << 0) | (1 << 8) | (address_ << 9) | ((is_read as u16) << 14) | 0u16 << 15
}
