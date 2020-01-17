use std::io::Result;

pub trait BytesRead
{
    fn load_vec_u32(vec_size: usize) -> Result<Vec<u32>>;
    fn load_vec_u16(vec_size: usize) -> Result<Vec<u32>>;
    fn read_ptr() -> Result<usize>;
    fn read_u32() -> Result<u32>;
    fn read_u16() -> Result<u16>;
    fn read_u8() -> Result<u8>;
}

pub trait Read
{
    fn read(buffer: &[u8], offset: usize) -> Result<Self>
    where 
        Self: Sized;

    fn read_buffered<R>(breader: R) -> Result<Self>
    where 
        R: BytesRead, Self: Sized;
}

// Create a version for a buffered bytes-io style system.