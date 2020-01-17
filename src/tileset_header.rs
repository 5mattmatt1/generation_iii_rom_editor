pub struct TilesetHeader
{
    compressed: u8,
    is_primary: bool,
    b2: u8,
    b3: u8,
    pub gfx_ptr: usize,
    pub pal_ptr: usize,
    blocks_ptr: usize,
    behavior_ptr: usize,
    animation_ptr: usize
}

impl TilesetHeader
{
    pub fn new(buffer: &[u8], offset: usize) -> std::io::Result<Self> 
    {
        use crate::bytes::{load_poke_ptr, load_u16};
        println!("compressed: {}", buffer[offset + 0]);
        println!("primary: {}", buffer[offset + 1]);
        Ok(TilesetHeader {
            compressed: buffer[offset + 0],
            is_primary: (buffer[offset + 1] == 0),
            b2: buffer[offset + 2],
            b3: buffer[offset + 3],
            gfx_ptr: load_poke_ptr(buffer, offset + 0x4)?,
            pal_ptr: load_poke_ptr(buffer, offset + 0x8)?,
            blocks_ptr: load_poke_ptr(buffer, offset + 0xC)?,
            behavior_ptr: load_poke_ptr(buffer, offset + 0x10)?,
            animation_ptr: load_poke_ptr(buffer, offset + 0x14)?
        })
    }
}