use bitfielder::bitfield;

use crate::sdio::Error;

bitfield! {
    /// Represents the R1 response type.
    pub R1(u8): u8,
    mask: 0x7f,
    default: 0x00,
    {
        /// Represents whether a command's argument was invalid.
        pub parameter_error: 6;
        /// Represents whether a misaligned address was passed as a command argument.
        pub address_error: 5;
        /// Represents whether there was an error in the erase sequence commands.
        pub erase_sequence_error: 4;
        /// Represents whether the COM CRC check of the last command failed.
        pub crc_error: 3;
        /// Represents whether an illegal command was received.
        pub illegal_command: 2;
        /// Represents whether an erase sequence was cleared before executing.
        pub erase_reset: 1;
        /// Represents whether the card is in the idle state.
        pub idle: 0;
    }
}

impl R1 {
    /// Represents the byte length of the R1 response.
    pub const LEN: usize = 1;
    /// Represents the bitmask for the R1 response.
    pub const MASK: u8 = 0x7f;

    /// Attempts to convert a byte slice into a [R1].
    pub const fn try_from_bytes(val: &[u8]) -> Result<Self, Error> {
        match val.len() {
            len if len < Self::LEN => Err(Error::General),
            _ => Ok(Self(val[0] & Self::MASK)),
        }
    }

    /// Converts the [R1] into a byte array.
    pub const fn into_bytes(self) -> [u8; Self::LEN] {
        [self.0]
    }
}
