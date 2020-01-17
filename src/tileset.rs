use crate::tileset_header::TilesetHeader;

const PAL_LEN: usize = 0xFF;

pub struct Tileset
{
    header: TilesetHeader,
    palette: Vec<u16>
}

impl Tileset
{
    pub fn new(buffer: &[u8], tileset_header: TilesetHeader) -> std::io::Result<Self>
    {
        use crate::bytes::load_vec_u16;
        let palette = load_vec_u16(buffer, tileset_header.pal_ptr, PAL_LEN)?;
        // Seems to start having real data here.
        // for i in 0..0x950
        // {
        //     if de_gfx[i] != 0
        //     {
        //         println!("gfx[0x{:X}] = {:X}", i, de_gfx[i]);
        //         break;
        //     }
        // }
        Ok(Self { 
            header: tileset_header,
            palette: palette,
        })
    }

    pub fn export(&self, buffer: &[u8], fpath: &str) -> std::io::Result<()>
    {

        use crate::lz77::decompress;
        let de_gfx = decompress(buffer, self.header.gfx_ptr)?;
        
        use crate::draw::draw_gba_image;
        draw_gba_image(fpath, &de_gfx, &self.palette).unwrap();
        
        Ok(())
    }
}