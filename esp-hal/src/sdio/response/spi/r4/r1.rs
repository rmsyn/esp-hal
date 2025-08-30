use bitfielder::bitfield;

bitfield! {
    /// Represents the Modified R1 response type used in SPI mode R4 response.
    pub ModifiedR1(u8): u8,
    mask: 0x5d,
    default: 0x00,
    {
        /// Represents whether a command's argument was invalid.
        pub parameter_error: 6;
        /// Represents whether there was an error in command function number.
        pub function_number_error: 4;
        /// Represents whether the COM CRC check of the last command failed.
        pub crc_error: 3;
        /// Represents whether an illegal command was received.
        pub illegal_command: 2;
        /// Represents whether the card is in the idle state.
        pub idle: 0;
    }
}
