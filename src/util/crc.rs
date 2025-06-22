use safe_arch::{
    add_i32_m128i,
    load_unaligned_m128i,
    m128i,
    set_splat_i32_m128i,
    shuffle_av_i8z_all_m128i,
};

pub(crate) fn compute_crc(data: &[u8]) -> u32 {
    let crc = big_hash(data);

    (crc & 0xFFFFFF) as u32
}

macro_rules! process {
    // Base case: no shift values provided
    (@process $mix:ident; $a:expr, $b:expr, $c:expr;) => {};

    // Recursive case: call the step rule and rotate the state for the next shift values
    (@process $mix:ident; $a:expr, $b:expr, $c:expr; $first:expr $(, $rest:expr )*) => {
        $mix!(@step $a, $b, $c, $first);
        process!(@process $mix; $b, $c, $a; $($rest),*);
    };
}

macro_rules! mix_m128i {
    ($a:expr, $b:expr, $c:expr) => {
        process! {
            @process mix_m128i;
            $a, $b, $c;
            4, 6, 8, 16, 19, 4
        }
    };

    (@step $a:expr, $b:expr, $c:expr, $shift:expr) => {
        $a = ::safe_arch::sub_i32_m128i($a, $c);
        $a ^= {
            let lo = ::safe_arch::shl_imm_u32_m128i::<{ $shift }>($c);
            let hi = ::safe_arch::shr_imm_u32_m128i::<{ 32 - $shift }>($c);
            ::safe_arch::bitor_m128i(lo, hi)
        };
        $c = ::safe_arch::add_i32_m128i($c, $b);
    };
}

macro_rules! mix {
    ($a:expr, $b:expr, $c:expr) => {
        process! {
            @process mix;
            $a, $b, $c;
            4, 6, 8, 16, 19, 4
        }
    };

    (@step $a:expr, $b:expr, $c:expr, $shift:expr) => {
        $a = $a.wrapping_sub($c);
        $a ^= $c.rotate_left($shift);
        $c = $c.wrapping_add($b);
    };
}

macro_rules! final_mix {
    ($a:expr, $b:expr, $c:expr) => {
        process! {
            @process final_mix;
            $c, $a, $b;
            14, 11, 25, 16, 4, 14, 24
        }
    };

    (@step $a:expr, $b:expr, $c:expr, $shift:expr) => {
        $a ^= $c;
        $a = $a.wrapping_sub($c.rotate_left($shift));
    };
}

#[inline(always)]
fn big_hash(data: &[u8]) -> u64 {
    let length = data.len() as u32;

    let a: u32 = 0xDEADBEEF_u32.wrapping_add(length);
    let b: u32 = 0x206F85B3_u32;
    let c: u32 = 0x5768B525_u32.wrapping_sub(length);

    let mut a: m128i = set_splat_i32_m128i(a as i32);
    let mut b: m128i = set_splat_i32_m128i(b as i32);
    let mut c: m128i = set_splat_i32_m128i(c as i32);

    let chunks_count = data.len() / 48;
    let remainder = data.len() % 48;

    for i in 0..chunks_count {
        let offset = i * 48;

        let (chunk_a, chunk_b, chunk_c) = get_chunks(&data[offset..]);

        a = add_i32_m128i(a, chunk_a);
        b = add_i32_m128i(b, chunk_b);
        c = add_i32_m128i(c, chunk_c);

        mix_m128i!(a, b, c);
    }

    if remainder > 0 {
        let mut padded = [0; 48];
        padded[..remainder].copy_from_slice(&data[chunks_count * 48..]);

        let (chunk_a, chunk_b, chunk_c) = get_chunks(&padded);

        a = add_i32_m128i(a, chunk_a);
        b = add_i32_m128i(b, chunk_b);
        c = add_i32_m128i(c, chunk_c);

        mix_m128i!(a, b, c);
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

/// Extracts 48-byte chunks from input data and converts them to three m128i
/// vectors.
#[inline(always)]
fn get_chunks(data: &[u8]) -> (m128i, m128i, m128i) {
    debug_assert!(data.len() >= 48);

    // Wrap data slice with parentheses to ensure we call TryFrom<&[T]> for &[T; N]
    // instead of TryFrom<&[T]> for [T; N] which would copy the bytes rather than
    // reference the data, which we want to avoid unnecessary memory allocation
    let chunk_a: &[u8; 16] = (&data[0..16]).try_into().expect("slice is not 16 bytes");
    let chunk_b: &[u8; 16] = (&data[16..32]).try_into().expect("slice is not 16 bytes");
    let chunk_c: &[u8; 16] = (&data[32..48]).try_into().expect("slice is not 16 bytes");

    let chunk_a = load_unaligned_m128i(chunk_a);
    let chunk_b = load_unaligned_m128i(chunk_b);
    let chunk_c = load_unaligned_m128i(chunk_c);

    reorder_bytes(chunk_a, chunk_b, chunk_c)
}

/// Reorders bytes in the chunks to match the expected big-endian byte ordering.
///
/// The incoming data bytes need to be reordered within each 4-byte group.
#[inline(always)]
fn reorder_bytes(
    chunk_a: m128i,
    chunk_b: m128i,
    chunk_c: m128i,
) -> (m128i, m128i, m128i) {
    #[rustfmt::skip]
        let mask: m128i = m128i::from([
            3,  2,  1,  0,
            7,  6,  5,  4,
            11, 10, 9,  8,
            15, 14, 13, 12u8
        ]);

    let chunk_a = shuffle_av_i8z_all_m128i(chunk_a, mask);
    let chunk_b = shuffle_av_i8z_all_m128i(chunk_b, mask);
    let chunk_c = shuffle_av_i8z_all_m128i(chunk_c, mask);

    (chunk_a, chunk_b, chunk_c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_crc_32_bytes() {
        let data = b"Lorem ipsum dolor sit amet, cons";
        assert_eq!(compute_crc(data), 0x21AFCD);
    }

    #[test]
    fn test_compute_crc_48_bytes() {
        let data = b"Lorem ipsum dolor sit amet, consectetur adipisci";
        assert_eq!(compute_crc(data), 0xBC1751);
    }

    #[test]
    fn test_compute_crc_64_bytes() {
        let data = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do ";
        assert_eq!(compute_crc(data), 0x427779);
    }
}
