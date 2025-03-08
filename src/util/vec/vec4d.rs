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

macro_rules! impl_ops_add {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Add for Vec4D<$t> {
                type Output = Self;

                fn add(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data + rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0].wrapping_add(rhs.data[0]),
                                self.data[1].wrapping_add(rhs.data[1]),
                                self.data[2].wrapping_add(rhs.data[2]),
                                self.data[3].wrapping_add(rhs.data[3]),
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::Add<$t> for Vec4D<$t> {
                type Output = Self;

                fn add(self, rhs: $t) -> Self {
                    self.add(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::AddAssign for Vec4D<$t> {
                fn add_assign(&mut self, rhs: Self) {
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
                }
            }

            impl ::std::ops::AddAssign<$t> for Vec4D<$t> {
                fn add_assign(&mut self, rhs: $t) {
                    self.add_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_sub {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Sub for Vec4D<$t> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data - rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0].wrapping_sub(rhs.data[0]),
                                self.data[1].wrapping_sub(rhs.data[1]),
                                self.data[2].wrapping_sub(rhs.data[2]),
                                self.data[3].wrapping_sub(rhs.data[3]),
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::Sub<$t> for Vec4D<$t> {
                type Output = Self;

                fn sub(self, rhs: $t) -> Self {
                    self.sub(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::SubAssign for Vec4D<$t> {
                fn sub_assign(&mut self, rhs: Self) {
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
                }
            }

            impl ::std::ops::SubAssign<$t> for Vec4D<$t> {
                fn sub_assign(&mut self, rhs: $t) {
                    self.sub_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_mul {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Mul for Vec4D<$t> {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data * rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0].wrapping_mul(rhs.data[0]),
                                self.data[1].wrapping_mul(rhs.data[1]),
                                self.data[2].wrapping_mul(rhs.data[2]),
                                self.data[3].wrapping_mul(rhs.data[3]),
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::Mul<$t> for Vec4D<$t> {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self {
                    self.mul(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::MulAssign for Vec4D<$t> {
                fn mul_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "simd")]
                    {
                        self.data *= rhs.data;
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        self.data[0] = self.data[0].wrapping_mul(rhs.data[0]);
                        self.data[1] = self.data[1].wrapping_mul(rhs.data[1]);
                        self.data[2] = self.data[2].wrapping_mul(rhs.data[2]);
                        self.data[3] = self.data[3].wrapping_mul(rhs.data[3]);
                    }
                }
            }

            impl ::std::ops::MulAssign<$t> for Vec4D<$t> {
                fn mul_assign(&mut self, rhs: $t) {
                    self.mul_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_div {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Div for Vec4D<$t> {
                type Output = Self;

                fn div(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data / rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0].wrapping_div(rhs.data[0]),
                                self.data[1].wrapping_div(rhs.data[1]),
                                self.data[2].wrapping_div(rhs.data[2]),
                                self.data[3].wrapping_div(rhs.data[3]),
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::Div<$t> for Vec4D<$t> {
                type Output = Self;

                fn div(self, rhs: $t) -> Self {
                    self.div(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::DivAssign for Vec4D<$t> {
                fn div_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "simd")]
                    {
                        self.data /= rhs.data;
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        self.data[0] = self.data[0].wrapping_div(rhs.data[0]);
                        self.data[1] = self.data[1].wrapping_div(rhs.data[1]);
                        self.data[2] = self.data[2].wrapping_div(rhs.data[2]);
                        self.data[3] = self.data[3].wrapping_div(rhs.data[3]);
                    }
                }
            }

            impl ::std::ops::DivAssign<$t> for Vec4D<$t> {
                fn div_assign(&mut self, rhs: $t) {
                    self.div_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_bitand {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::BitAnd for Vec4D<$t> {
                type Output = Self;

                fn bitand(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data & rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0] & rhs.data[0],
                                self.data[1] & rhs.data[1],
                                self.data[2] & rhs.data[2],
                                self.data[3] & rhs.data[3],
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::BitAnd<$t> for Vec4D<$t> {
                type Output = Self;

                fn bitand(self, rhs: $t) -> Self {
                    self.bitand(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::BitAndAssign for Vec4D<$t> {
                fn bitand_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "simd")]
                    {
                        self.data &= rhs.data;
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        self.data[0] &= rhs.data[0];
                        self.data[1] &= rhs.data[1];
                        self.data[2] &= rhs.data[2];
                        self.data[3] &= rhs.data[3];
                    }
                }
            }

            impl ::std::ops::BitAndAssign<$t> for Vec4D<$t> {
                fn bitand_assign(&mut self, rhs: $t) {
                    self.bitand_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_bitor {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::BitOr for Vec4D<$t> {
                type Output = Self;

                fn bitor(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data | rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0] | rhs.data[0],
                                self.data[1] | rhs.data[1],
                                self.data[2] | rhs.data[2],
                                self.data[3] | rhs.data[3],
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::BitOr<$t> for Vec4D<$t> {
                type Output = Self;

                fn bitor(self, rhs: $t) -> Self {
                    self.bitor(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::BitOrAssign for Vec4D<$t> {
                fn bitor_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "simd")]
                    {
                        self.data |= rhs.data;
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        self.data[0] |= rhs.data[0];
                        self.data[1] |= rhs.data[1];
                        self.data[2] |= rhs.data[2];
                        self.data[3] |= rhs.data[3];
                    }
                }
            }

            impl ::std::ops::BitOrAssign<$t> for Vec4D<$t> {
                fn bitor_assign(&mut self, rhs: $t) {
                    self.bitor_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_bitxor {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::BitXor for Vec4D<$t> {
                type Output = Self;

                fn bitxor(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data ^ rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0] ^ rhs.data[0],
                                self.data[1] ^ rhs.data[1],
                                self.data[2] ^ rhs.data[2],
                                self.data[3] ^ rhs.data[3],
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::BitXor<$t> for Vec4D<$t> {
                type Output = Self;

                fn bitxor(self, rhs: $t) -> Self {
                    self.bitxor(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::BitXorAssign for Vec4D<$t> {
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

            impl ::std::ops::BitXorAssign<$t> for Vec4D<$t> {
                fn bitxor_assign(&mut self, rhs: $t) {
                    self.bitxor_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_shl {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Shl for Vec4D<$t> {
                type Output = Self;

                fn shl(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data << rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0] << rhs.data[0],
                                self.data[1] << rhs.data[1],
                                self.data[2] << rhs.data[2],
                                self.data[3] << rhs.data[3],
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::Shl<$t> for Vec4D<$t> {
                type Output = Self;

                fn shl(self, rhs: $t) -> Self {
                    self.shl(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::ShlAssign for Vec4D<$t> {
                fn shl_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "simd")]
                    {
                        self.data <<= rhs.data;
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        self.data[0] <<= rhs.data[0];
                        self.data[1] <<= rhs.data[1];
                        self.data[2] <<= rhs.data[2];
                        self.data[3] <<= rhs.data[3];
                    }
                }
            }

            impl ::std::ops::ShlAssign<$t> for Vec4D<$t> {
                fn shl_assign(&mut self, rhs: $t) {
                    self.shl_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_shr {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Shr for Vec4D<$t> {
                type Output = Self;

                fn shr(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data >> rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0] >> rhs.data[0],
                                self.data[1] >> rhs.data[1],
                                self.data[2] >> rhs.data[2],
                                self.data[3] >> rhs.data[3],
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::Shr<$t> for Vec4D<$t> {
                type Output = Self;

                fn shr(self, rhs: $t) -> Self {
                    self.shr(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::ShrAssign for Vec4D<$t> {
                fn shr_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "simd")]
                    {
                        self.data >>= rhs.data;
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        self.data[0] >>= rhs.data[0];
                        self.data[1] >>= rhs.data[1];
                        self.data[2] >>= rhs.data[2];
                        self.data[3] >>= rhs.data[3];
                    }
                }
            }

            impl ::std::ops::ShrAssign<$t> for Vec4D<$t> {
                fn shr_assign(&mut self, rhs: $t) {
                    self.shr_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_rem {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Rem for Vec4D<$t> {
                type Output = Self;

                fn rem(self, rhs: Self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: self.data % rhs.data,
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                self.data[0] % rhs.data[0],
                                self.data[1] % rhs.data[1],
                                self.data[2] % rhs.data[2],
                                self.data[3] % rhs.data[3],
                            ],
                        }
                    }
                }
            }

            impl ::std::ops::Rem<$t> for Vec4D<$t> {
                type Output = Self;

                fn rem(self, rhs: $t) -> Self {
                    self.rem(Self::Output::splat(rhs))
                }
            }

            impl ::std::ops::RemAssign for Vec4D<$t> {
                fn rem_assign(&mut self, rhs: Self) {
                    #[cfg(feature = "simd")]
                    {
                        self.data %= rhs.data;
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        self.data[0] %= rhs.data[0];
                        self.data[1] %= rhs.data[1];
                        self.data[2] %= rhs.data[2];
                        self.data[3] %= rhs.data[3];
                    }
                }
            }

            impl ::std::ops::RemAssign<$t> for Vec4D<$t> {
                fn rem_assign(&mut self, rhs: $t) {
                    self.rem_assign(Self::splat(rhs));
                }
            }
        )*
    };
}

macro_rules! impl_ops_not {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Not for Vec4D<$t> {
                type Output = Self;

                fn not(self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: !self.data
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                !self.data[0],
                                !self.data[1],
                                !self.data[2],
                                !self.data[3],
                            ],
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! impl_ops_neg {
    ($($t:ty),+) => {
        $(
            impl ::std::ops::Neg for Vec4D<$t> {
                type Output = Self;

                fn neg(self) -> Self {
                    #[cfg(feature = "simd")]
                    {
                        Self::Output {
                            data: -self.data
                        }
                    }
                    #[cfg(not(feature = "simd"))]
                    {
                        Self::Output {
                            data: [
                                -self.data[0],
                                -self.data[1],
                                -self.data[2],
                                -self.data[3],
                            ],
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! impl_ops {
    ($($t:ty),+) => {
        impl_ops_add!($($t),+);
        impl_ops_sub!($($t),+);
        impl_ops_mul!($($t),+);
        impl_ops_div!($($t),+);
        impl_ops_bitand!($($t),+);
        impl_ops_bitor!($($t),+);
        impl_ops_bitxor!($($t),+);
        impl_ops_shl!($($t),+);
        impl_ops_shr!($($t),+);
        impl_ops_rem!($($t),+);
        impl_ops_not!($($t),+);
    };
}

macro_rules! impl_utils {
    ($($t:ty),+) => {
        $(
            impl Vec4D<$t> {
                pub fn rotate_left(self, shift: $t) -> Self {
                    let hi = self >> (size_of::<$t>() as $t * 8 - shift);
                    let lo = self << shift;
                    lo | hi
                }

                pub fn wrapping_add(mut self, rhs: Self) -> Self {
                    self += rhs;
                    self
                }

                pub fn wrapping_sub(mut self, rhs: Self) -> Self {
                    self -= rhs;
                    self
                }
            }
        )+
    };
}

impl_ops!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl_ops_neg!(i8, i16, i32, i64, isize);
impl_utils!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
