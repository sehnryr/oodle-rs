#[cfg(feature = "simd")]
use std::simd::Simd;

use super::Element;

#[derive(Clone, Copy)]
pub struct Vec4D<T>
where
    T: Element,
{
    #[cfg(feature = "simd")]
    data: Simd<T, 4>,
    #[cfg(not(feature = "simd"))]
    data: [T; 4],
}

impl<T> Vec4D<T>
where
    T: Element,
{
    pub fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            #[cfg(feature = "simd")]
            data: Simd::splat(value),
            #[cfg(not(feature = "simd"))]
            data: [value; 4],
        }
    }

    pub fn from_array(data: [T; 4]) -> Self {
        Self {
            #[cfg(feature = "simd")]
            data: Simd::from_array(data),
            #[cfg(not(feature = "simd"))]
            data,
        }
    }
}

impl Vec4D<u32> {
    pub fn rotate_left(mut self, shift: u32) -> Self {
        #[cfg(feature = "simd")]
        {
            self.data = (self.data << shift) | (self.data >> (32 - shift));
        }
        #[cfg(not(feature = "simd"))]
        {
            self.data[0] = self.data[0].rotate_left(shift);
            self.data[1] = self.data[1].rotate_left(shift);
            self.data[2] = self.data[2].rotate_left(shift);
            self.data[3] = self.data[3].rotate_left(shift);
        }
        self
    }

    pub fn wrapping_add(mut self, rhs: Self) -> Self {
        #[cfg(feature = "simd")]
        {
            self.data += rhs.data;
        }
        #[cfg(not(feature = "simd"))]
        {
            self.data[0] = self.data[0].wrapping_add(rhs.data[0]);
            self.data[1] = self.data[1].wrapping_add(rhs.data[1]);
            self.data[2] = self.data[2].wrapping_add(rhs.data[2]);
            self.data[3] = self.data[3].wrapping_add(rhs.data[3]);
        }
        self
    }

    pub fn wrapping_sub(mut self, rhs: Self) -> Self {
        #[cfg(feature = "simd")]
        {
            self.data -= rhs.data;
        }
        #[cfg(not(feature = "simd"))]
        {
            self.data[0] = self.data[0].wrapping_sub(rhs.data[0]);
            self.data[1] = self.data[1].wrapping_sub(rhs.data[1]);
            self.data[2] = self.data[2].wrapping_sub(rhs.data[2]);
            self.data[3] = self.data[3].wrapping_sub(rhs.data[3]);
        }
        self
    }
}

impl<T> From<Vec4D<T>> for [T; 4]
where
    T: Element,
{
    fn from(value: Vec4D<T>) -> Self {
        #[cfg(feature = "simd")]
        {
            value.data.to_array()
        }
        #[cfg(not(feature = "simd"))]
        {
            value.data
        }
    }
}

impl std::ops::BitXorAssign for Vec4D<u32> {
    fn bitxor_assign(&mut self, rhs: Self) {
        #[cfg(feature = "simd")]
        {
            self.data ^= rhs.data;
        }
        #[cfg(not(feature = "simd"))]
        {
            self.data[0] ^= rhs.data[0];
            self.data[1] ^= rhs.data[1];
            self.data[2] ^= rhs.data[2];
            self.data[3] ^= rhs.data[3];
        }
    }
}
