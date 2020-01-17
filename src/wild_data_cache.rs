pub const WILD_POKEMON: usize = 0xB4D48;
pub const WILD_POKEMON_LEN: usize = 0x14;

use std::collections::HashMap;

use crate::wild_data::WildData;
use crate::wild_data_header::WildDataHeader;

pub struct WildDataCache
{
    wild_data: HashMap<usize, WildData>,
}

impl WildDataCache
{
    pub fn new() -> Self
    {
        Self {
            wild_data: HashMap::new()
        }
    }

    // 0, 11 - Dewford
    pub fn get(&self, bank: usize, map: usize) -> Option<&WildData>
    {
        let wd_idx: usize = (map << 8) | bank;
        self.wild_data.get(&wd_idx)
    }

    pub fn contains(&self, bank: usize, map: usize) -> bool
    {
        let wd_idx: usize = (map << 8) | bank;
        self.wild_data.contains_key(&wd_idx)
    }

    pub fn load_wild_data(&mut self, buffer: &[u8]) -> std::io::Result<()>
    {       
        use crate::bytes::load_poke_ptr;
        let wild_pokemon = load_poke_ptr(buffer, WILD_POKEMON)?;
        let mut idx: usize = 0; 
        let mut hdr: WildDataHeader;
        let mut data: WildData;
        let mut wd_idx: usize;
        loop {
            hdr = WildDataHeader::new(buffer, wild_pokemon + idx)?;
            data = WildData::new(buffer, hdr)?;
            if hdr.bank == 0xFF && hdr.map == 0xFF
            {
                break;
            }
            wd_idx = ((hdr.map as usize) << 8) | (hdr.bank as usize);
            self.wild_data.insert(wd_idx, data);
            // println!("Found wild data for bank {}, map {}", hdr.bank, hdr.map);
            idx += WILD_POKEMON_LEN; // W
        }
        Ok(())
    }
}