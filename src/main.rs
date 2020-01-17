#![allow(dead_code)]

pub mod draw;
pub mod lz77;

pub mod text;
pub mod bytes;
// Will probably move this up into a map module
pub mod bank_loader;
pub mod map;
pub mod map_header;
pub mod map_data;
pub mod map_tile_data;
pub mod tileset_header;
pub mod tileset;
pub mod io;
// Wild data
pub mod wild_data_cache;
pub mod wild_data_header;
pub mod wild_data;
pub mod wild_pokemon;
pub mod wild_pokemon_data;
pub mod wild_terrain;

// Char block's
// Seperate from tiles, but made up of them.
const PETALBURG_CITY_BLOCK_OFFSET: usize = 0x3960F0;
const PETALBURG_CITY_BLOCK_END: usize = 0x398DE0;

// Right side, palette?

// use std::io;

// fn dump_mapinfo(buffer: &[u8])
// {
//     let offset = 0x849CC;
//     print!("{:08X} | ", offset);

//     for i in 0..0x10
//     {
//         print!("{:02X} ", buffer[offset + i]);
//     }

//     println!("");
// }

// fn dump_petalburg(buffer: Vec<u8>)
// {
//     for offset in (PETALBURG_CITY_BLOCK_OFFSET..PETALBURG_CITY_BLOCK_END).step_by(0x10)
//     {
//         print!("{:08X} | ", offset);

//         for i in 0..0x8
//         {
//             print!("{:02X} ", buffer[offset + i]);
//         }

//         print!("| ");

//         for i in 0x8..0x10
//         {
//             print!("{:02X} ", buffer[offset + i]);
//         }
//         println!("");
//     }
// }

fn read_gba() -> std::io::Result<()>
{
    use std::io::prelude::*;
    use std::fs::File;
    
    let mut f = File::open("emerald.gba")?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;

    // dump_mapinfo(&buffer);
    use crate::bank_loader::{BankLoader, MAP_HEADERS};
    use crate::text::*;
    use crate::bytes::{find_ptr, load_poke_ptr};
    let bank_loader = BankLoader::new(&buffer, MAP_HEADERS).unwrap();
    let map_ptr = bank_loader.maps[0][0]; // Petalburg Overworld map
    use crate::map_header::MapHeader;
    use crate::map::Map;
    use crate::wild_data_cache::WildDataCache;
    // MapHeader::new(&buffer, map_ptr);
    // let petalburg_map = Map::new(&buffer, map_ptr).unwrap();

    // let move_names = load_string_table(&mut buffer, MOVE_NAMES, NUM_MOVES, MOVE_NAME_LEN).unwrap();
    // for move_name in move_names
    // {
    //     println!("{}", move_name);
    // }
    use crate::wild_pokemon::WildPokemon;
    let mut wild_data_cache = WildDataCache::new();
    wild_data_cache.load_wild_data(&buffer).unwrap();
    let route_101 = wild_data_cache.get(0, 16).unwrap();
    let route_101_pokemon = &route_101.grass_data.as_ref().unwrap().wild_pokemon;
    for pokemon in route_101_pokemon
    {
        println!("({}-{}) {}", pokemon.min_lvl, pokemon.max_lvl, pokemon.num);
    }
    let mut route_101_pokemon_grass = route_101.grass_data.as_ref().unwrap().clone();
    route_101_pokemon_grass.wild_pokemon[0] = WildPokemon {
        min_lvl: 3,
        max_lvl: 3,
        num: 277
    };
    route_101_pokemon_grass.wild_pokemon[1] = WildPokemon {
        min_lvl: 3,
        max_lvl: 3,
        num: 277
    };
    route_101_pokemon_grass.wild_pokemon[2] = WildPokemon {
        min_lvl: 3,
        max_lvl: 3,
        num: 277
    };
    route_101_pokemon_grass.wild_pokemon[3] = WildPokemon {
        min_lvl: 3,
        max_lvl: 3,
        num: 277
    };
    route_101_pokemon_grass.write(&mut buffer);
    // Seedot route, west of Mauvile
    // let route_117 = wild_data_cache.get(0, 32).unwrap();
    // let route_117_pokemon = &route_117.grass_data.as_ref().unwrap().wild_pokemon;
    // for pokemon in route_117_pokemon
    // {
    //     println!("({}-{}) {}", pokemon.min_lvl, pokemon.max_lvl, pokemon.num);
    // }

    capitalize_string_table(&mut buffer, SPECIES_NAMES, NUM_SPECIES, SPECIES_NAME_LEN)?;
    capitalize_string_table(&mut buffer, MOVE_NAMES, NUM_MOVES, MOVE_NAME_LEN)?;
    // println!("0x{:08X} poke ptr to 0x{:08X}", 0x144, load_poke_ptr(&buffer, 0x148).unwrap());
    // let poke_text = read_poke_text(&buffer, load_poke_ptr(&buffer, 0x148).unwrap());
    // println!("{}", poke_text);
    // println!("Found pointer to data: 0x{:08X} at 0x{:08X}", 0x003185C8, find_ptr(&buffer, 0x003185C8).unwrap());
    // println!("Found pointer to data: 0x{:08X} at 0x{:08X}", 0x00319789, find_ptr(&buffer, 0x00319789).unwrap());
    // println!("Found instance of string {} at 0x{:08X}", "KARATE CHOP", find_string(&buffer, "KARATE CHOP"));

    // println!("Found pointer to data: 0x{:08X} at 0x{:08X}", 0xB4D48, find_ptr(&buffer, 0xB4D48).unwrap());
    // println!("Found instance of string {} at 0x{:08X}", "POUND", find_string(&buffer, "POUND"));
    // println!("Found instance of string {} at 0x{:08X}", "SCRATCH", find_string(&buffer, "SCRATCH"));
    // println!("Found instance of string {} at 0x{:08X}", "CUT", find_string(&buffer, "CUT"));
    // println!("Found instance of string {} at 0x{:08X}", "GUST", find_string(&buffer, "GUST"));
    // println!("Found instance of string {} at 0x{:08X}", "WHIRLWIND", find_string(&buffer, "WHIRLWIND"));
    // println!("Found instance of string {} at 0x{:08X}", "FLY", find_string(&buffer, "FLY"));
    // println!("Found instance of string {} at 0x{:08X}", "TOXIC", find_string(&buffer, "TOXIC"));
    // println!("Found instance of string {} at 0x{:08X}", "EMBER", find_string(&buffer, "EMBER"));
    // println!("Found instance of string {} at 0x{:08X}", "GROWL", find_string(&buffer, "GROWL"));
    // println!("Found instance of string {} at 0x{:08X}", "LEER", find_string(&buffer, "LEER"));
    // println!("Found instance of string {} at 0x{:08X}", "TACKLE", find_string(&buffer, "TACKLE"));
    // println!("Found instance of string {} at 0x{:08X}", "HARDEN", find_string(&buffer, "HARDEN"));
    // capitalize_species_names(&buffer).unwrap();

    // use crate::draw::test_draw;
    // test_draw();
    
    let mut file = File::create("emerald_capital.gba")?;
    file.write_all(&buffer)?;

    Ok(())
}

fn main() {
    read_gba().unwrap();
}