// TODO: Find a better naming scheme to differentiate between 
// the pointer table of "WildDataHeader" and the actual information of encounter data
#[derive(Copy, Clone)]
pub struct WildPokemon
{
    pub min_lvl: u8,
    pub max_lvl: u8,
    pub num: u16,
}

impl WildPokemon
{
    pub fn new(buffer: &[u8], offset: usize) -> std::io::Result<Self>
    {
        use crate::bytes::load_u16;

        Ok(Self {
            min_lvl: buffer[offset + 0x00],
            max_lvl: buffer[offset + 0x01],
            num: load_u16(buffer, offset + 0x02)?,
        })
    }

    // Maybe make a write and a write_to function?
    pub fn write(&self, buffer: &mut [u8], offset: usize) -> std::io::Result<()>
    {
        use crate::bytes::write_u16;
        // Let's say for now, ratio and dn_enabled are read only fields.
        buffer[offset + 0x00] = self.min_lvl;
        buffer[offset + 0x01] = self.max_lvl; 
        write_u16(buffer, offset + 0x02, self.num)?;
        Ok(())
    }
}