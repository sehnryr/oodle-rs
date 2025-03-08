use crate::util::array_range;

pub(crate) fn compute_crc(data: &[u8]) -> u32 {
    let crc = big_hash(data);

    (crc & 0xFFFFFF) as u32
}

#[cfg(feature = "simd")]
mod quad {
    use std::simd::{Simd, SimdElement};

    #[derive(Clone, Copy)]
    pub struct Quad<T>
    where
        T: SimdElement,
    {
        data: Simd<T, 4>,
    }

    impl<T> Quad<T>
    where
        T: SimdElement,
    {
        pub fn new(data: [T; 4]) -> Self {
            Self {
                data: Simd::from_array(data),
            }
        }
    }

    impl Quad<u32> {
        pub fn rotate_left(mut self, shift: u32) -> Self {
            self.data = (self.data << shift) | (self.data >> (32 - shift));
            self
        }

        pub fn wrapping_add(mut self, rhs: Self) -> Self {
            self.data += rhs.data;
            self
        }

        pub fn wrapping_sub(mut self, rhs: Self) -> Self {
            self.data -= rhs.data;
            self
        }
    }

    impl From<Quad<u32>> for [u32; 4] {
        fn from(value: Quad<u32>) -> Self {
            value.data.to_array()
        }
    }

    impl std::ops::BitXorAssign for Quad<u32> {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.data ^= rhs.data;
        }
    }
}

#[cfg(not(feature = "simd"))]
mod quad {
    #[derive(Clone, Copy)]
    pub struct Quad<T> {
        data: [T; 4],
    }

    impl<T> Quad<T> {
        pub fn new(data: [T; 4]) -> Self {
            Self { data }
        }
    }

    impl Quad<u32> {
        pub fn rotate_left(mut self, shift: u32) -> Self {
            self.data[0] = self.data[0].rotate_left(shift);
            self.data[1] = self.data[1].rotate_left(shift);
            self.data[2] = self.data[2].rotate_left(shift);
            self.data[3] = self.data[3].rotate_left(shift);
            self
        }

        pub fn wrapping_add(mut self, rhs: Self) -> Self {
            self.data[0] = self.data[0].wrapping_add(rhs.data[0]);
            self.data[1] = self.data[1].wrapping_add(rhs.data[1]);
            self.data[2] = self.data[2].wrapping_add(rhs.data[2]);
            self.data[3] = self.data[3].wrapping_add(rhs.data[3]);
            self
        }

        pub fn wrapping_sub(mut self, rhs: Self) -> Self {
            self.data[0] = self.data[0].wrapping_sub(rhs.data[0]);
            self.data[1] = self.data[1].wrapping_sub(rhs.data[1]);
            self.data[2] = self.data[2].wrapping_sub(rhs.data[2]);
            self.data[3] = self.data[3].wrapping_sub(rhs.data[3]);
            self
        }
    }

    impl From<Quad<u32>> for [u32; 4] {
        fn from(value: Quad<u32>) -> Self {
            value.data
        }
    }

    impl std::ops::BitXorAssign for Quad<u32> {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.data[0] ^= rhs.data[0];
            self.data[1] ^= rhs.data[1];
            self.data[2] ^= rhs.data[2];
            self.data[3] ^= rhs.data[3];
        }
    }
}

use quad::Quad;

macro_rules! mix {
    ($a:expr, $b:expr, $c:expr) => {
        mix!(; $a, $b, $c, 4);
        mix!(; $b, $c, $a, 6);
        mix!(; $c, $a, $b, 8);
        mix!(; $a, $b, $c, 16);
        mix!(; $b, $c, $a, 19);
        mix!(; $c, $a, $b, 4);
    };
    (; $a:expr, $b:expr, $c:expr, $shift:expr) => {
        $a = $a.wrapping_sub($c);
        $a ^= $c.rotate_left($shift);
        $c = $c.wrapping_add($b);
    }
}

macro_rules! final_mix {
    ($a:expr, $b:expr, $c:expr) => {
        final_mix!(; $c, $b, 14);
        final_mix!(; $a, $c, 11);
        final_mix!(; $b, $a, 25);
        final_mix!(; $c, $b, 16);
        final_mix!(; $a, $c, 4);
        final_mix!(; $b, $a, 14);
        final_mix!(; $c, $b, 24);
    };
    (; $a:expr, $c:expr, $shift:expr) => {
        $a ^= $c;
        $a = $a.wrapping_sub($c.rotate_left($shift));
    }
}

fn big_hash(data: &[u8]) -> u64 {
    let length = data.len() as u32;

    let mut a = Quad::new([0xdeadbeef_u32.wrapping_add(length); 4]);
    let mut b = Quad::new([0x206F85B3_u32; 4]);
    let mut c = Quad::new([0x5768B525_u32.wrapping_sub(length); 4]);

    let mut chunks = data.chunks_exact(16 * 3);

    while let Some(chunk) = chunks.next() {
        let mut quads = chunk.chunks_exact(16).map(|chunk| {
            Quad::new([
                u32::from_be_bytes(array_range!(chunk, 0; .. 4)),
                u32::from_be_bytes(array_range!(chunk, 4; .. 8)),
                u32::from_be_bytes(array_range!(chunk, 8; .. 12)),
                u32::from_be_bytes(array_range!(chunk, 12; .. 16)),
            ])
        });

        a = a.wrapping_add(quads.next().unwrap());
        b = b.wrapping_add(quads.next().unwrap());
        c = c.wrapping_add(quads.next().unwrap());

        mix!(a, b, c);
    }

    if let remainder @ [_, ..] = chunks.remainder() {
        let mut last = [0; 16 * 3];
        last[..remainder.len()].copy_from_slice(remainder);

        let mut quads = last.chunks_exact(16).map(|chunk| {
            Quad::new([
                u32::from_be_bytes(array_range!(chunk, 0; .. 4)),
                u32::from_be_bytes(array_range!(chunk, 4; .. 8)),
                u32::from_be_bytes(array_range!(chunk, 8; .. 12)),
                u32::from_be_bytes(array_range!(chunk, 12; .. 16)),
            ])
        });

        a = a.wrapping_add(quads.next().unwrap());
        b = b.wrapping_add(quads.next().unwrap());
        c = c.wrapping_add(quads.next().unwrap());

        mix!(a, b, c);
    }

    let a: [u32; 4] = a.into();
    let b: [u32; 4] = b.into();
    let c: [u32; 4] = c.into();

    let mut fa;
    let mut fb;
    let mut fc;

    fa = a[0];
    fb = a[1];
    fc = a[2];
    mix!(fa, fb, fc);
    fa = fa.wrapping_add(a[3]);
    fb = fb.wrapping_add(b[0]);
    fc = fc.wrapping_add(b[1]);
    mix!(fa, fb, fc);
    fa = fa.wrapping_add(b[2]);
    fb = fb.wrapping_add(b[3]);
    fc = fc.wrapping_add(c[0]);
    mix!(fa, fb, fc);
    fa = fa.wrapping_add(c[1]);
    fb = fb.wrapping_add(c[2]);
    fc = fc.wrapping_add(c[3]);
    final_mix!(fa, fb, fc);

    (fb as u64) << 32 | fc as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_crc() {
        let data = b"Hello, world!";
        assert_eq!(compute_crc(data), 0xEF5D64);
    }
}
