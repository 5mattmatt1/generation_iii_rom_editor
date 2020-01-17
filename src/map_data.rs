use crate::map_header::MapHeader;

#[derive(Copy, Clone)]
pub struct MapData
{
    pub map_width: u32,
    pub map_height: u32,
    pub border_tile_ptr: usize,
    pub map_tiles_ptr: usize,
    pub global_tile_set_ptr: usize,
    pub local_tile_set_ptr: usize,
    pub border_width: u32,
    pub border_height: u32,
}

impl MapData
{
    pub fn new(buffer: &[u8], map_header: MapHeader) -> std::io::Result<Self>
    {
        use crate::bytes::{load_poke_ptr, load_u16, load_u32};
        let map_ptr = map_header.map_ptr;
        Ok(MapData {
            map_width: load_u32(buffer, map_ptr)?,
            map_height: load_u32(buffer, map_ptr + 0x4)?,
            border_tile_ptr: load_poke_ptr(buffer, map_ptr + 0x8)?,
            map_tiles_ptr: load_poke_ptr(buffer, map_ptr + 0xC)?,
            global_tile_set_ptr: load_poke_ptr(buffer, map_ptr + 0x10)?,
            local_tile_set_ptr: load_poke_ptr(buffer, map_ptr + 0x14)?,
            border_width: buffer[map_ptr + 0x18] as u32,
            border_height: buffer[map_ptr + 0x19] as u32
        })
    }
}