use crate::error::Error;
use packed_struct::{prelude::*, PackingResult};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vid(u16);

const VID_SIZE: usize = 2;
const _DEFAULT_VID_U16: u16 = 0xFFFF;
const _DEFAULT_VID_SLICE: [u8; VID_SIZE] = [0xFF, 0xFF];
const _DEFAULT_VID_STR: &str = "FFFF";

impl Default for Vid {
    fn default() -> Self {
        Self(_DEFAULT_VID_U16)
    }
}

impl PackedStruct for Vid {
    type ByteArray = [u8; VID_SIZE];

    fn pack(&self) -> PackingResult<Self::ByteArray> {
        PackingResult::Ok(u16::to_be_bytes(self.0))
    }

    fn unpack(src: &Self::ByteArray) -> PackingResult<Self> {
        PackingResult::Ok(Vid(u16::from_be_bytes(*src)))
    }
}

impl TryFrom<&str> for Vid {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let decoded_hex = hex::decode(value).map_err(|e| Error::Decode(format!("{}", e)))?;
        let byte_array: &[u8; VID_SIZE] = decoded_hex
            .as_slice()
            .try_into()
            .map_err(|e| Error::Convert(format!("{}", e)))?;
        let uid = Vid::unpack(byte_array).map_err(|e| Error::Unpack(format!("{}", e)))?;
        Ok(uid)
    }
}

impl From<u16> for Vid {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<i16> for Vid {
    fn from(value: i16) -> Self {
        Self(value as u16)
    }
}

impl Into<u16> for Vid {
    fn into(self) -> u16 {
        self.0
    }
}

impl Into<i16> for Vid {
    fn into(self) -> i16 {
        self.0 as i16
    }
}

impl std::fmt::Display for Vid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}", self.0)
    }
}

#[cfg(test)]
pub mod test {
    #![allow(clippy::panic, clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(Vid::default(), Vid::try_from(_DEFAULT_VID_STR).unwrap());
    }

    #[test]
    fn test_format() {
        assert_eq!(_DEFAULT_VID_STR, format!("{}", Vid::default()));
    }

    #[test]
    fn test_pack() {
        assert_eq!(_DEFAULT_VID_SLICE, Vid::default().pack().unwrap());
    }

    #[test]
    fn test_unpack() {
        assert_eq!(Vid::default(), Vid::unpack(&_DEFAULT_VID_SLICE).unwrap());
    }
}
