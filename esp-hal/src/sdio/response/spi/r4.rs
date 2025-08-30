use bitfielder::bitfield;

use crate::sdio::{Error, IoOcr};

mod r1;

pub use r1::ModifiedR1;

bitfield! {
    /// Represents the SPI modes R4 (`IO_SEND_OP_COND`) response.
    pub R4(MSB0 [u8; 5]): u32 {
        raw_modified_r1: 39, 32;
        pub ready: 31;
        pub number_of_functions: 30, 28;
        pub memory_present: 27;
        raw_io_ocr: 23, 0;
    }
}

impl R4 {
    /// Represents the byte length of the [R4] response.
    pub const LEN: usize = 5;

    /// Gets the Modified R1 response.
    pub const fn modified_r1(&self) -> ModifiedR1 {
        ModifiedR1::from_inner(self.raw_modified_r1() as u8)
    }

    /// Sets the Modified R1 response.
    pub fn set_modified_r1(&mut self, val: ModifiedR1) {
        self.set_raw_modified_r1(val.into_inner() as u32);
    }

    /// Gets the I/O OCR register.
    pub const fn io_ocr(&self) -> IoOcr {
        IoOcr::from_u32(self.raw_io_ocr())
    }

    /// Sets the I/O OCR register.
    pub fn set_io_ocr(&mut self, val: IoOcr) {
        self.set_raw_io_ocr(val.into_u32());
    }

    /// Attempts to convert a byte slice into a [R4].
    pub const fn try_from_bytes(val: &[u8]) -> Result<Self, Error> {
        match val.len() {
            len if len < Self::LEN => Err(Error::General),
            _ => Ok(Self([val[0], val[1], val[2], val[3], val[4]])),
        }
    }

    /// Converts the [R4] into a byte array.
    pub const fn into_bytes(self) -> [u8; Self::LEN] {
        self.0
    }
}
