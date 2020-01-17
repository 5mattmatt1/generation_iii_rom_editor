#[derive(Copy, Clone)]
pub struct MapHeader
{
    offset: u32,
    pub map_ptr: usize,
    sprites_ptr: usize,
    script_ptr: usize,
    connect_ptr: usize,
    hsong: u16,
    hmap: u16,
    label_id: u8,
    flash: u8,
    weather: u8,
    mtype: u8,
    unused_1: u8,
    unused_2: u8,
    label_toggle: u8,
    unused_3: u8,
}

impl MapHeader
{
    pub fn new(buffer: &[u8], offset: usize) -> std::io::Result<Self>
    {
        use crate::bytes::{load_poke_ptr, load_u16};
        Ok(MapHeader {
            offset: offset as u32,
            map_ptr: load_poke_ptr(buffer, offset)?,
            sprites_ptr: load_poke_ptr(buffer, offset + 0x4)?,
            script_ptr: load_poke_ptr(buffer, offset + 0x8)?,
            connect_ptr: load_poke_ptr(buffer, offset + 0xC)?,
            hsong: load_u16(buffer, offset + 0x10)?,
            hmap: load_u16(buffer, offset + 0x12)?,
            label_id: buffer[offset+0x14],
            flash: buffer[offset+0x15],
            weather: buffer[offset+0x16],
            mtype: buffer[offset+0x17],
            unused_1: buffer[offset+0x18],
            unused_2: buffer[offset+0x19],
            label_toggle: buffer[offset+0x1A],
            unused_3: buffer[offset+0x1B],
        })
    }
}