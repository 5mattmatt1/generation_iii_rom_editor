use crate::map_data::MapData;

const ID_MASK: usize = 0x3FF;
const META_MASK: usize = 0xFC00;
const META_SHIFT: usize = 10;

// Curious of the different meta values for a tile...
pub struct MapTile(u16);
// {
//     tile: u16,
// }

pub struct MapTileData(Vec<u16>);
// {
//     tiles: Vec<u16>
// }

impl MapTileData
{
    pub fn new(buffer: &[u8], map_data: MapData) -> std::io::Result<Self>
    {
        use crate::bytes::{load_vec_u16};
        let len: usize = (map_data.map_width * map_data.map_height) as usize;
        Ok(Self(load_vec_u16(buffer, map_data.map_tiles_ptr, len)?))
    }
}