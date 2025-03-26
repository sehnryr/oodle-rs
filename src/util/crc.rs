use std::array::from_fn;

use super::vec::Vec4D;

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

fn big_hash(data: &[u8]) -> u64 {
    let length = data.len() as u32;

    let mut a = Vec4D::splat(0xdeadbeef_u32.wrapping_add(length));
    let mut b = Vec4D::splat(0x206F85B3_u32);
    let mut c = Vec4D::splat(0x5768B525_u32.wrapping_sub(length));

    let exact_chunks = data.chunks_exact(48);
    let remainder = exact_chunks.remainder();

    let last: Box<dyn Iterator<Item = [u8; 48]>> = if remainder.is_empty() {
        Box::new(std::iter::empty())
    } else {
        let mut padded = [0; 48];
        padded[..remainder.len()].copy_from_slice(remainder);
        Box::new(std::iter::once(padded))
    };

    let mut chunks = exact_chunks
        .map(|chunk| chunk.try_into().unwrap()) // convert slice into array
        .chain(last);

    while let Some(chunk) = chunks.next() {
        let mut quads = chunk.chunks_exact(16).map(|chunk| {
            Vec4D::from_array([
                u32::from_be_bytes(from_fn(|i| chunk[i])),
                u32::from_be_bytes(from_fn(|i| chunk[i + 4])),
                u32::from_be_bytes(from_fn(|i| chunk[i + 8])),
                u32::from_be_bytes(from_fn(|i| chunk[i + 12])),
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
