//! https://github.com/sparkfun/SparkFun_LIS3DH_Arduino_Library

/// Addresses of control and status registers.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterAddress {
    // Device Registers
    StatusRegAux = 0x07,
    OutAdc1L = 0x08,
    OutAdc1H = 0x09,
    OutAdc2L = 0x0A,
    OutAdc2H = 0x0B,
    OutAdc3L = 0x0C,
    OutAdc3H = 0x0D,
    IntCounterReg = 0x0E,
    WhoAmI = 0x0F,

    TempCfgReg = 0x1F,
    CtrlReg1 = 0x20,
    CtrlReg2 = 0x21,
    CtrlReg3 = 0x22,
    CtrlReg4 = 0x23,
    CtrlReg5 = 0x24,
    CtrlReg6 = 0x25,
    Reference = 0x26,
    StatusReg2 = 0x27,
    OutXL = 0x28,
    OutXH = 0x29,
    OutYL = 0x2A,
    OutYH = 0x2B,
    OutZL = 0x2C,
    OutZH = 0x2D,
    FifoCtrlReg = 0x2E,
    FifoSrcReg = 0x2F,
    Int1Cfg = 0x30,
    Int1Src = 0x31,
    Int1Ths = 0x32,
    Int1Duration = 0x33,

    ClickCfg = 0x38,
    ClickSrc = 0x39,
    ClickThs = 0x3A,
    TimeLimit = 0x3B,
    TimeLatency = 0x3C,
    TimeWindow = 0x3D,
}
fn is_valid_register_address(val: u8) -> bool {
    if (0x07..=0x0F).contains(&val) || (0x1F..=0x33).contains(&val) || (0x38..=0x3D).contains(&val)
    {
        true
    } else {
        false
    }
}
impl TryFrom<u8> for RegisterAddress {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if is_valid_register_address(value) {
            unsafe { Ok(core::mem::transmute::<u8, RegisterAddress>(value)) }
        } else {
            Err(())
        }
    }
}
pub fn spi_data_address(data: u8, address: u8, is_read: bool) -> u16 {
    // data is a full byte
    let data_ = data as u16;
    // address is within [07,3F], but we'll ignore it for now
    // top most bit is the read/write bit which is weird
    // because some address have it set, but i guess
    // some bit is always not set when that bit is set
    // also second to last bit is the inc bit for reading regions,
    // though that's real specific so no need to change the structure of this
    let address_ = address as u16;

    (data_ << 0) | (address_ << 8) | ((is_read as u16) << 15)
}

pub fn write_register<T: embedded_hal::blocking::spi::Transfer<u16>>(
    spi: &mut T,
    address: RegisterAddress,
    data: u8,
) -> Result<(), T::Error> {
    // Toss out the read data
    match spi.transfer(&mut [spi_data_address(data, address as u8, false)]) {
        Ok(v) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn read_register<T: embedded_hal::blocking::spi::Transfer<u16>>(
    spi: &mut T,
    address: RegisterAddress,
) -> Result<u16, T::Error> {
    match spi.transfer(&mut [spi_data_address(0x00, address as u8, true)]) {
        Ok(val) => Ok(val[0]),
        Err(e) => Err(e),
    }
}

/// put in the low addres to get back [high,low]
pub fn read_high_low<T: embedded_hal::blocking::spi::Transfer<u16>>(
    spi: &mut T,
    address_low: RegisterAddress,
) -> Result<u16, T::Error> {
    let mut r = 0x00;
    // probably should have this just be a fn which reads two address
    // and make the caller pick which two addresses to read, but who cares
    // this is how this chip works so why not do it this way
    let address_high = match RegisterAddress::try_from(address_low as u8 + 1) {
        Ok(a) => a,
        Err(_) => panic!("Address was not correct todo make this an actual error"),
    };

    // low
    r |= read_register(spi, address_low)?;
    // high
    r |= read_register(spi, address_high)? << 8;
    Ok(r)
}

/// range is the max g the accel can read
/// can be 2,4,8,16
pub fn read_x_float<T: embedded_hal::blocking::spi::Transfer<u16>>(
    spi: &mut T,
    range: u8,
) -> Result<f32, T::Error> {
    let raw: u16 = read_high_low(spi, RegisterAddress::OutXL)?;
    // now depending on the range, we are going to have different divisors
    if range != 2 || range != 4 || range != 8 || range != 16 {
        // means invalid so panic, but TODO: err in the future
        panic!("Invalid Range");
    }
    // directly from sparkfun's arduino library:
    Ok((raw as f32) / (range as f32))
}
/// range is the max g the accel can read
/// can be 2,4,8,16
pub fn read_y_float<T: embedded_hal::blocking::spi::Transfer<u16>>(
    spi: &mut T,
    range: u8,
) -> Result<f32, T::Error> {
    let raw: u16 = read_high_low(spi, RegisterAddress::OutYL)?;
    // now depending on the range, we are going to have different divisors
    if range != 2 || range != 4 || range != 8 || range != 16 {
        // means invalid so panic, but TODO: err in the future
        panic!("Invalid Range");
    }
    // directly from sparkfun's arduino library:
    Ok((raw as f32) / (range as f32))
}

/// range is the max g the accel can read
/// can be 2,4,8,16
pub fn read_z_float<T: embedded_hal::blocking::spi::Transfer<u16>>(
    spi: &mut T,
    range: u8,
) -> Result<f32, T::Error> {
    let raw: u16 = read_high_low(spi, RegisterAddress::OutZL)?;
    // now depending on the range, we are going to have different divisors
    if range != 2 || range != 4 || range != 8 || range != 16 {
        // means invalid so panic, but TODO: err in the future
        panic!("Invalid Range");
    }
    // directly from sparkfun's arduino library:
    Ok((raw as f32) / (range as f32))
}

// pub fn read_register_region<T: embedded_hal::blocking::spi::Transfer<u16>, const N: usize>(
//     spi: &mut T,
//     address: RegisterAddress,
// ) -> Result<[u16; N], T::Error> {
//     // maybe should be passing output refs but this is good too
//     // don't know what it will actually look like in the future anyways
//     let mut r: [u16; N] = [0; N];

//     // also maybe theres some embedded_hal, spi lib thing that already does this kinda thing
//     // maybe, maybe not

//     //initial transfer with 0x40 for an inc read
//     spi.transfer(&mut [spi_data_address(0x00, address as u8 | 0x40, true)])?;
//     for cx in 0..N {
//         // transfer 0x00 to read
//         match spi.transfer(&mut [0x00]) {
//             Ok(val) => r[cx] = val[0],
//             // if we get an error, just stop and do that:
//             Err(e) => return Err(e),
//         }
//     }
//     return Ok(r);
// }
