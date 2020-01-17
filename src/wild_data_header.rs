#[derive(Copy, Clone)]
pub struct WildDataHeader
{
    pub bank: u8,
    pub map: u8,
    pub grass_ptr: usize,
    pub water_ptr: usize,
    pub trees_ptr: usize,
    pub fishing_ptr: usize,
}

impl WildDataHeader
{
    pub fn new(buffer: &[u8], offset: usize) -> std::io::Result<Self>
    {
        use crate::bytes::load_poke_ptr;

        Ok(Self {
            bank: buffer[offset + 0x00],
            map: buffer[offset + 0x01],
            // Padding
            grass_ptr: load_poke_ptr(buffer, offset + 0x04)?,
            water_ptr: load_poke_ptr(buffer, offset + 0x08)?,
            trees_ptr: load_poke_ptr(buffer, offset + 0x0C)?,
            fishing_ptr: load_poke_ptr(buffer, offset + 0x10)?,
        })
    }
}