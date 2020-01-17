
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, RenderTarget};
use sdl2::surface::Surface;
use sdl2::pixels::Color;

pub fn test_draw()
{
    use sdl2::pixels::PixelFormatEnum;
    let width: u32 = 0xFF;
    let height: u32 = 0xFF;    
    let px_fmt = PixelFormatEnum::RGB555;
    let mut surface = Surface::new(width, height, px_fmt).unwrap();

    let rect = Rect::new(0, 0, 32, 32);
    let clr = Color::RGB(255, 255, 64);
    surface.fill_rect(rect, clr).unwrap();

    surface.save_bmp("output.bmp").unwrap();
}

// MainTSBlocks	= 0x200
// LocalTSBlocks	= 0xFE
// MainTSSize	= 0x280
// LocalTSSize	= 0x140
// MainTSHeight	= 0x140
pub const TS_WIDTH: u32 = 0x80;
pub const LOCAL_TS_HEIGHT: u32 = 0xC0;

pub fn draw_gba_image(path: &str, buf: &[u8], pal: &[u16]) -> Result<(), String>
{
    use sdl2::pixels::PixelFormatEnum;
    let width: u32 = 0xFF;
    let height: u32 = 0xFF;    
    let px_fmt = PixelFormatEnum::RGB555;
    let mut surface = Surface::new(TS_WIDTH, LOCAL_TS_HEIGHT, px_fmt)?;

    // let rect = Rect::new(0, 0, 32, 32);
    // let clr = Color::RGB(255, 255, 64);

    let mut x = 0;
    let mut y = 0;
    let mut blkx = 0;
    let mut blky = 0;
    let mut rect: Rect;
    let mut pidx = 0;

    for i in 0..(buf.len() * 2)
    {
        if x >= 8
        {
            x = 0;
            y += 1;
        }

        if y >= 8
        {
            y = 0;
            blkx += 1;
        }

        if blkx > ((TS_WIDTH / 8) - 1)
        {
            blkx = 0;
            blky += 1;
        }

        pidx = buf[i/2];
        // What does this flag mean?
        if (i & 1) == 0
        {
            pidx &= 0x0F;
        } else 
        {
            pidx &= 0xF0;
            pidx >>= 4;
        }
        pidx += 0x90;
        
        rect = Rect::new((x + (blkx * 8)) as i32,
                         (y + (blky * 8)) as i32,
                         1, 1);
        let pclr = pal[pidx as usize];
        // They condensed the bit shifting to extract the 5 bit R, G, B values 
        // and then refit them to the 255 range all in one operation.
        let pclr_r = (pclr & 0x1F) << 3;
        let pclr_g = (pclr & 0x3E0) >> 2;
        let pclr_b = (pclr & 0x7C00) >> 7;
        let clr = Color::RGB(pclr_r as u8, pclr_g as u8, pclr_b as u8);
        surface.fill_rect(rect, clr)?;
        
        x += 1;
    }

    surface.save_bmp(path)?;
    Ok(())
}