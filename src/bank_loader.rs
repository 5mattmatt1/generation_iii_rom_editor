pub const MAP_HEADERS: usize = 0x84AA4;
// Where did this come from?
const MAP_LABELS: usize = 0x123B44; // 0x5a147c;
const NUM_BANKS: usize = 0x22;

// Why, I have no idea.
const POINTER_MASK: usize = 0x1FFFFFF;

use std::collections::HashMap;

pub struct BankLoader
{
    map_names: HashMap<usize, String>,
    pub maps: Vec<Vec<usize>>,
}

impl BankLoader
{
    pub fn new(buffer: &[u8], offset: usize) -> std::io::Result<Self>
    {
        use crate::bytes::*;
        use crate::text::read_poke_text;
        use byteorder::{LittleEndian, ReadBytesExt};
        let bank_num_map: Vec<usize> = vec![57,5,5,6,7,8,9,7,7,14,8,17,10,23,13,15,15,2,2,2,3,1,1,1,108,61,89,2,1,13,1,1,2,1];
        let mut map_names: HashMap<usize, String> = HashMap::new();
        let mut maps: Vec<Vec<usize>> = Vec::new();
    
        let map_labels = load_poke_ptr(buffer, 0x123B44)?;
        // Read in pointer...
        let table_offset = load_poke_ptr(buffer, offset)?;
        // println!("Table Offset: 0x{:08X}", table_offset);
        let mut num_map: usize;
        // let mut bank_offset: usize;
        let mut map_data_offset: usize;
        let mut map_name: usize;
        let mut map_name_ptr: usize;
        for (bank, bank_offset) in load_vec_u32(buffer, table_offset as usize , NUM_BANKS)?.iter().enumerate()
        {
            maps.push(Vec::new());
            num_map = bank_num_map[bank];
            // println!("0x{:08X}", bank_offset & 0x1FFFFFF); 
            // Why does the pointer need to be shortened?
            // Maybe metadata?
            for (map, map_data_offset) in load_vec_u32(buffer, (bank_offset & 0x1FFFFFF) as usize, num_map)?.iter().enumerate()
            {
                maps[bank].push(*map_data_offset as usize & 0x1FFFFFF);
                // map_data_offset = (&buffer[bank_offset+(map*4)..bank_offset+(map*4)+4]).read_u32::<LittleEndian>().unwrap() as usize;
                // println!("\t+\t0x{:08X}", map_data_offset);
                // A lot of math here, the numbers Mason, what do they mean? (MEH)
                // println!("\t+\t0x{:08X}", ((map_data_offset - (8 << 24)) + 0x14));
                let map_name_idx = buffer[((map_data_offset - (8 << 24)) + 0x14) as usize] as u32;
                // println!("\t+\t0x{:08X}", map_name_idx);
                map_name = map_labels + ((map_name_idx * 8) + 4) as usize;
                map_name_ptr = load_poke_ptr(buffer, map_name)?;
                // println!("\t+\t0x{:08X}", map_name_ptr);
                // println!("\t+\t{}", read_poke_text(buffer, map_name_ptr));
                
                map_names.insert(map_name, read_poke_text(buffer, map_name_ptr));
            }
        }

        Ok(BankLoader {
            map_names: map_names,
            maps: maps
        })
    }
}