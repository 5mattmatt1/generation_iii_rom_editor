use crate::wild_pokemon::WildPokemon;
use crate::wild_terrain::WildTerrain;

pub struct WildPokemonData
{
    ptr: usize,
    terrain: WildTerrain,
    pub ratio: u8,
    pub dn_enabled: u8,
    pub pokemon_ptr: usize,
    pub wild_pokemon: Vec<WildPokemon>,
}

// WildDataCache::gatherData in MEH is the best resource for how this works.
// It seems like Wild Pokemon is a table seperate from the data within the map headers.
impl WildPokemonData
{
    pub fn new(buffer: &[u8], offset: usize, terrain: WildTerrain) -> std::io::Result<Self>
    {
        use std::collections::HashMap;
        use crate::bytes::load_poke_ptr;

        let mut terrain_lens: HashMap<WildTerrain, usize> = HashMap::new();
        terrain_lens.insert(WildTerrain::Grass, 12);
        terrain_lens.insert(WildTerrain::Water, 5);
        terrain_lens.insert(WildTerrain::Tree, 5);
        terrain_lens.insert(WildTerrain::Fishing, 10);
        // Will probably want to create logic for reading in dn data,
        // but atm all maps have it disabled so...
        let pokemon_ptr: usize = load_poke_ptr(buffer, offset + 0x4)?;
        let mut wild_pokemon: Vec<WildPokemon> = Vec::new();
        let mut idx: usize = 0;
        let terrain_len = *terrain_lens.get(&terrain).unwrap();

        for i in 0..terrain_len
        {
            wild_pokemon.push(WildPokemon::new(buffer, pokemon_ptr+idx)?);
            idx += 4; // 32 bit entry
        }

        Ok(Self {
            ptr: offset,
            terrain: terrain,
            ratio: buffer[offset + 0],
            dn_enabled: buffer[offset + 1],
            pokemon_ptr: pokemon_ptr,
            wild_pokemon: wild_pokemon,
        })
    }

    pub fn write(&self, buffer: &mut [u8]) -> std::io::Result<()>
    {
        // Let's say for now, ratio and dn_enabled are read only fields.
        for (i, wild_pokemon) in self.wild_pokemon.iter().enumerate()
        {
            wild_pokemon.write(buffer, self.pokemon_ptr+(i*4))?;
        }

        Ok(())
    }

    pub fn clone(&self) -> Self
    {
        Self {
            ptr: self.ptr,
            terrain: self.terrain,
            ratio: self.ratio,
            dn_enabled: self.dn_enabled,
            pokemon_ptr: self.pokemon_ptr,
            wild_pokemon: self.wild_pokemon.clone(),
        }
    }
}