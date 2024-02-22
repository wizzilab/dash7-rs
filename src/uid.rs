use crate::error::Error;
use packed_struct::{prelude::*, PackingResult};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Uid(u64);

impl Default for Uid {
    fn default() -> Self {
        Self(0xDEADBEEF8BADF00D)
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

impl std::fmt::Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

#[cfg(test)]
pub mod test {
    #![allow(clippy::panic, clippy::unwrap_used)]
    use super::*;

    const DEFAULT_UID_SLICE: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0x8B, 0xAD, 0xF0, 0x0D];
    const DEFAULT_UID_STR: &str = "DEADBEEF8BADF00D";

    #[test]
    fn test_try_from() {
        assert_eq!(Uid::default(), Uid::try_from(DEFAULT_UID_STR).unwrap());
    }

    #[test]
    fn test_format() {
        assert_eq!(DEFAULT_UID_STR, format!("{}", Uid::default()));
    }

    #[test]
    fn test_pack() {
        assert_eq!(DEFAULT_UID_SLICE, Uid::default().pack().unwrap());
    }

    #[test]
    fn test_unpack() {
        assert_eq!(Uid::default(), Uid::unpack(&DEFAULT_UID_SLICE).unwrap());
    }
}
