use crate::error::Error;
use packed_struct::{prelude::*, PackingResult};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Uid(u64);

const _DEFAULT_UID_U64: u64 = 0xDEADBEEF8BADF00D;
const _DEFAULT_UID_SLICE: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0x8B, 0xAD, 0xF0, 0x0D];
const _DEFAULT_UID_STR: &str = "DEADBEEF8BADF00D";

impl Default for Uid {
    fn default() -> Self {
        Self(_DEFAULT_UID_U64)
    }
}

impl PackedStruct for Uid {
    type ByteArray = [u8; 8];

    fn pack(&self) -> PackingResult<Self::ByteArray> {
        PackingResult::Ok(u64::to_be_bytes(self.0))
    }

    fn unpack(src: &Self::ByteArray) -> PackingResult<Self> {
        PackingResult::Ok(Uid(u64::from_be_bytes(*src)))
    }
}

impl TryFrom<&str> for Uid {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let decoded_hex = hex::decode(value).map_err(|e| Error::Decode(format!("{}", e)))?;
        let byte_array: &[u8; 8] = decoded_hex
            .as_slice()
            .try_into()
            .map_err(|e| Error::Convert(format!("{}", e)))?;
        let uid = Uid::unpack(byte_array).map_err(|e| Error::Unpack(format!("{}", e)))?;
        Ok(uid)
    }
}

impl From<u64> for Uid {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<i64> for Uid {
    fn from(value: i64) -> Self {
        Self(value as u64)
    }
}

impl Into<u64> for Uid {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<i64> for Uid {
    fn into(self) -> i64 {
        self.0 as i64
    }
}

impl std::fmt::Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

#[cfg(test)]
pub mod test {
    #![allow(clippy::panic, clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_try_from() {
        assert_eq!(Uid::default(), Uid::try_from(_DEFAULT_UID_STR).unwrap());
    }

    #[test]
    fn test_format() {
        assert_eq!(_DEFAULT_UID_STR, format!("{}", Uid::default()));
    }

    #[test]
    fn test_pack() {
        assert_eq!(_DEFAULT_UID_SLICE, Uid::default().pack().unwrap());
    }

    #[test]
    fn test_unpack() {
        assert_eq!(Uid::default(), Uid::unpack(&_DEFAULT_UID_SLICE).unwrap());
    }
}
