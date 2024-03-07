use packed_struct::prelude::*;

#[derive(PackedStruct, Copy, Clone, Debug, Default)]
#[packed_struct(bit_numbering = "lsb0", size_bytes = "1")]
pub struct Ctf {
    #[packed_field(bits = "0..=4")]
    mantissa: Integer<u8, packed_bits::Bits<5>>,
    #[packed_field(bits = "5..=7")]
    exponent: Integer<u8, packed_bits::Bits<3>>,
}

impl Ctf {
    pub fn saturated() -> Self {
        Ctf {
            mantissa: 0x1F.into(),
            exponent: 0x7.into(),
        }
    }

    pub fn floor(value: usize) -> Self {
        let mut m = value;
        let mut e = 0;

        // Search best (smallest) exponent
        loop {
            if m < 32 {
                break;
            }
            e += 1;
            m >>= 2;
        }

        if e >= 8 {
            Ctf::saturated()
        } else {
            let m = m as u8;

            Ctf {
                mantissa: m.into(),
                exponent: e.into(),
            }
        }
    }

    pub fn ceil(value: usize) -> Self {
        let ctf = Self::floor(value);

        if Ctf::saturated() == ctf {
            return ctf;
        }

        let mut m: u8 = ctf.mantissa.into();
        let mut e: u8 = ctf.exponent.into();
        let val: usize = ctf.into();

        // manage floor
        if val < value {
            if m < 31 {
                m += 1;
            } else if e < 7 {
                // mant = 31+1 = 4*8
                e += 1;
                m = 8;
            } else {
                // exp = 7+1 -> overflow
                return Ctf::saturated();
            }
        }

        Ctf {
            mantissa: m.into(),
            exponent: e.into(),
        }
    }
}

impl From<Ctf> for usize {
    fn from(value: Ctf) -> usize {
        let m: u8 = value.mantissa.into();
        let e: u8 = value.exponent.into();

        (1 << (2 * e as usize)) * m as usize
    }
}

impl From<&Ctf> for usize {
    fn from(value: &Ctf) -> usize {
        let m: u8 = value.mantissa.into();
        let e: u8 = value.exponent.into();

        (1 << (2 * e as usize)) * m as usize
    }
}

impl PartialEq for Ctf {
    fn eq(&self, other: &Self) -> bool {
        <&Ctf as Into<usize>>::into(self) == other.into()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    use super::*;

    macro_rules! ctf {
        ($e:literal, $m:literal) => {
            ($e << 5) | $m
        };
    }

    #[test]
    fn test_ctf() {
        assert_eq!(0usize, Ctf::default().into());
        assert_eq!([0x00], Ctf::default().pack().unwrap());

        assert_eq!(507904usize, Ctf::saturated().into());
        assert_eq!([0xFF], Ctf::saturated().pack().unwrap());

        assert_eq!([ctf!(0, 0)], Ctf::floor(0).pack().unwrap());
        assert_eq!([ctf!(0, 1)], Ctf::floor(1).pack().unwrap());
        assert_eq!([ctf!(0, 31)], Ctf::floor(31).pack().unwrap());
        assert_eq!([ctf!(1, 8)], Ctf::floor(32).pack().unwrap());
        assert_eq!([ctf!(1, 8)], Ctf::floor(33).pack().unwrap());
        assert_eq!([ctf!(1, 8)], Ctf::floor(34).pack().unwrap());
        assert_eq!([ctf!(1, 8)], Ctf::floor(35).pack().unwrap());
        assert_eq!([ctf!(1, 9)], Ctf::floor(36).pack().unwrap());

        assert_eq!([ctf!(3, 16)], Ctf::floor(1024).pack().unwrap());

        assert_eq!(960usize, Ctf::floor(1023).into());
        assert_eq!(1024usize, Ctf::floor(1024).into());
        assert_eq!(1024usize, Ctf::floor(1025).into());
        assert_eq!(1024usize, Ctf::ceil(1023).into());
        assert_eq!(1024usize, Ctf::ceil(1024).into());
        assert_eq!(1088usize, Ctf::ceil(1025).into());
    }
}
