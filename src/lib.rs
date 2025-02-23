mod error;

use std::io::{Read, Write};

use crate::error::Result;

pub fn compress<R, W>(decompressed: R, compressed: W) -> Result<usize>
where
    R: Read,
    W: Write,
{
    todo!()
}

pub fn decompress<R, W>(compressed: R, decompressed: W) -> Result<usize>
where
    R: Read,
    W: Write,
{
    todo!()
}
