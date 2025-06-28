use super::error::{
    DecodeError,
    DecodeResult,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralType {
    /// Regular bytes.
    Raw = 1,
    /// Difference between byte and byte from the most recent offset.
    Sub = 0,
    /// Same as [`LiteralType::Sub`] but the first byte of each run is coded in
    /// its own entropy stream.
    LamSub = 2,
    /// Same as [`LiteralType::Sub`] but there's four literal streams separately
    /// entropy coded. `DST_BYTE_OFFSET % 4` controls which literal stream to
    /// read from.
    SubAnd3 = 3,
    /// Uses 16 literal streams where `LAST_BYTE >> 4` controls which literal
    /// stream to read from.
    Order1 = 4,
    /// Same as [`LiteralType::SubAnd3`] but 16 streams.
    SubAndF = 5,
}

impl TryFrom<usize> for LiteralType {
    type Error = DecodeError;

    fn try_from(value: usize) -> DecodeResult<Self> {
        match value {
            0 => Ok(Self::Sub),
            1 => Ok(Self::Raw),
            2 => Ok(Self::LamSub),
            3 => Ok(Self::SubAnd3),
            4 => Ok(Self::Order1),
            5 => Ok(Self::SubAndF),
            _ => Err(DecodeError::InvalidLiteralType(value)),
        }
    }
}

impl From<LiteralType> for i32 {
    fn from(value: LiteralType) -> Self { value as Self }
}
