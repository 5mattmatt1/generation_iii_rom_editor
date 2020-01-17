use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn load_vec_u32(buffer: &[u8], offset: usize, vec_size: usize) -> std::io::Result<Vec<u32>>
{
    let mut vec: Vec<u32> = Vec::new();
    for i in 0..vec_size
    {
        vec.push((&buffer[offset+(i*4)..offset+(i*4)+4]).read_u32::<LittleEndian>()?)
    }

    Ok(vec)
}

pub fn load_vec_u16(buffer: &[u8], offset: usize, vec_size: usize) -> std::io::Result<Vec<u16>>
{
    let mut vec: Vec<u16> = Vec::new();
    for i in 0..vec_size
    {
        vec.push((&buffer[offset+(i*2)..offset+(i*2)+2]).read_u16::<LittleEndian>()?)
    }

    Ok(vec)
}

pub fn load_u32(buffer: &[u8], offset: usize) -> std::io::Result<u32>
{
    Ok((&buffer[offset..offset+4]).read_u32::<LittleEndian>()?)
}

pub fn load_u16(buffer: &[u8], offset: usize) -> std::io::Result<u16>
{
    Ok((&buffer[offset..offset+2]).read_u16::<LittleEndian>()?)
}

pub fn load_poke_ptr(buffer: &[u8], offset: usize) -> std::io::Result<usize>
{
    Ok(((&buffer[offset..offset+4]).read_u32::<LittleEndian>()? & 0x1FFFFFF) as usize)
}

pub fn write_u16(buffer: &mut [u8], offset: usize, val: u16) -> std::io::Result<()>
{
    let mut u16_vec = Vec::new();
    u16_vec.write_u16::<LittleEndian>(val)?;

    for (idx, &byte) in u16_vec.iter().enumerate()
    {
        buffer[offset + idx] = byte;
    }
    Ok(())
    // Ok((&buffer[offset..offset+2]).read_u16::<LittleEndian>()?)
}

// Probably need to make an Option...
pub fn find_ptr(buffer: &[u8], ptr: usize) -> std::io::Result<usize>
{
    let ptr_size = 4;

    for i in (0..buffer.len()).step_by(ptr_size)
    {
        if load_poke_ptr(buffer, i)? == ptr
        {
            return Ok(i);
        }
    }

    Ok(0)
}