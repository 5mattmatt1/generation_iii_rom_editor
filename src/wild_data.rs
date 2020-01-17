use crate::wild_data_header::WildDataHeader;
use crate::wild_pokemon_data::WildPokemonData;
use crate::wild_terrain::WildTerrain;

pub struct WildData
{
    header: WildDataHeader,
    pub grass_data: Option<WildPokemonData>,
    pub water_data: Option<WildPokemonData>,
    pub trees_data: Option<WildPokemonData>,
    pub fishing_data: Option<WildPokemonData>,
}

impl WildData
{
    // Having moves on header data is a nother good reason to pass offset to top level, 
    // and delegate top level to loading in the header and using the information.
    pub fn new(buffer: &[u8], header: WildDataHeader) -> std::io::Result<Self>
    {
        let mut grass_data: Option<WildPokemonData> = None;
        let mut water_data: Option<WildPokemonData> = None;
        let mut trees_data: Option<WildPokemonData> = None;
        let mut fishing_data: Option<WildPokemonData> = None;

        if header.grass_ptr != 0
        {
            grass_data = Some(WildPokemonData::new(buffer, header.grass_ptr, WildTerrain::Grass)?);
        }

        if header.water_ptr != 0
        {
            water_data = Some(WildPokemonData::new(buffer, header.water_ptr, WildTerrain::Water)?);
        }
        
        if header.trees_ptr != 0
        {
            trees_data = Some(WildPokemonData::new(buffer, header.trees_ptr, WildTerrain::Tree)?);
        }

        if header.fishing_ptr != 0
        {
            fishing_data = Some(WildPokemonData::new(buffer, header.fishing_ptr, WildTerrain::Fishing)?);
        }

        Ok(Self {
            header: header,
            grass_data: grass_data,
            water_data: water_data,
            trees_data: trees_data,
            fishing_data: fishing_data
        })
    }
}