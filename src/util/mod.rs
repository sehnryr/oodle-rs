pub mod compression;
pub mod crc;
pub mod into_bindings;

macro_rules! array_range {
    ($bytes:expr, $start:expr; .. $end:expr) => {
        ::core::array::from_fn::<_, { $end - $start }, _>(|i| $bytes[$start + i])
    };
}

pub(crate) use array_range;
