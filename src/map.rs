use crate::map_header::MapHeader;
use crate::map_data::MapData;
use crate::tileset::Tileset;
use crate::tileset_header::TilesetHeader;

pub struct Map
{
    header: MapHeader,
    data: MapData,
}

impl Map
{
    pub fn new(buffer: &[u8], offset: usize) -> std::io::Result<Self>
    {
        let map_header = MapHeader::new(buffer, offset)?;
        let map_data = MapData::new(buffer, map_header)?;
        println!("Width: {}", map_data.map_width);
        println!("Height: {}", map_data.map_height);
        let global_tile_set_hdr = TilesetHeader::new(buffer, map_data.global_tile_set_ptr)?;
        let global_tile_set = Tileset::new(buffer, global_tile_set_hdr)?;
        global_tile_set.export(buffer, "global_tileset.bmp")?;
        let local_tile_set_hdr = TilesetHeader::new(buffer, map_data.local_tile_set_ptr)?;
        let local_tile_set = Tileset::new(buffer, local_tile_set_hdr)?;
        local_tile_set.export(buffer, "local_tileset.bmp")?;
        // map_data.global_tile_set_ptr:
        Ok(Self {
            header: map_header,
            data: map_data
        })
    }
}