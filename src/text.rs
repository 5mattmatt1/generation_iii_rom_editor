pub const NUM_SPECIES: usize = 412;
pub const SPECIES_NAME_LEN: usize = 11;
pub const SPECIES_NAMES: usize = 0x144;

pub const NUM_MOVES: usize = 355;
pub const MOVE_NAME_LEN: usize = 13;
pub const MOVE_NAMES: usize = 0x148;
// pub const SPECIES_NAMES: usize = 0x144;
// pub const MOVES_NAMES: usize = 0x00319789;

pub fn from_poke_char(poke_char: u8) -> char
{
    match poke_char
    {
        0x01 => 'À',
        0x02 => 'Á',
        0x03 => 'Â',
        0x04 => 'Ç',
        0x05 => 'È',
        0x06 => 'É',
        0x07 => 'Ê',
        0x08 => 'Ë',
        0x09 => 'Ì',
        // 0x0A
        0x0B => 'Î',
        0x0C => 'Ï',
        0x0D => 'Ò',
        0x0E => 'Ó',
        0x0F => 'Ô',
        0x10 => 'Œ',
        0x11 => 'Ù',
        0x12 => 'Ú',
        0x13 => 'Û',
        0x14 => 'Ñ',
        0x15 => 'ß',
        0x16 => 'à',
        0x17 => 'á',
        // 0x18
        0x19 => 'ç',
        0x1A => 'è',
        0x1B => 'é',
        0x1C => 'ê',
        0x1D => 'ë',
        0x1E => 'ì',
        // 0x1F
        0x20 => 'î',
        0x21 => 'ï',
        0x22 => 'ò',
        0x23 => 'ó',
        0x24 => 'ô',
        0x25 => 'œ',
        0x26 => 'ù',
        0x27 => 'ú',
        0x28 => 'û',
        0x29 => 'ñ',
        0x2A => 'º',
        0x2B => 'ª',
        // 0x2C
        0x2D => '&',
        0x2E => '+',
        // 0x2F
        // 0x30
        // 0x31
        // 0x32
        // 0x33
        // 0x34 => [Lv]
        0x35 => '=',
        0x36 => ';',
        // 0x37
        // 0x38
        // 0x39
        // 0x40
        // 0x41
        // 0x42
        // 0x43
        // 0x44
        // 0x45
        // 0x46
        // 0x47
        // 0x48
        // 0x49
        // 0x4A
        // 0x4B
        // 0x4C
        // 0x4D
        // 0x4E
        // 0x4F
        // 0x50
        0x51 => '¿',
        0x52 => '¡',
        // 0x53 => [pk]
        // 0x54 => [mn]
        // 0x55 => [po]
        // 0x56 => [ké]
        // 0x57 => [bl]
        // 0x58 => [oc]
        // 0x59 => [k]
        0x5A => 'Í',
        0x5B => '%',
        0x5C => '(',
        0x5D => ')',
        // 0x5E
        // 0x5F
        // 0x60
        // 0x61
        // 0x62
        // 0x63
        // 0x64
        // 0x65
        // 0x66
        // 0x67
        0x68 => 'â',
        // 0x69
        // 0x6A
        // 0x6B
        // 0x6C
        // 0x6D
        // 0x6E
        0x6F => 'í',
        // 0x70
        // 0x71
        // 0x72
        // 0x73
        // 0x74
        // 0x75
        // 0x76
        // 0x77
        // 0x78
        // 0x79 => [U]
        // 0x7A => [D]
        // 0x7B => [L]
        // 0x7C => [R]
        // 0x7D
        // 0x7E
        // 0x7F
        // 0x80
        // 0x81
        // 0x82
        // 0x83
        // 0x84
        0x85 => '<',
        0x86 => '>',
        // 0x87
        // 0x88
        // 0x89
        // 0x8A
        // 0x8B
        // 0x8C
        // 0x8D
        // 0x8E
        // 0x8F
        // 0x90
        // 0x91
        // 0x92
        // 0x93
        // 0x94
        // 0x95
        // 0x96
        // 0x97
        // 0x98
        // 0x99
        // 0x9A
        // 0x9B
        // 0x9C
        // 0x9D
        // 0x9E
        // 0x9F
        // 0xA0
        0xA1 => '0',
        0xA2 => '1',
        0xA3 => '2',
        0xA4 => '3',
        0xA5 => '4',
        0xA6 => '5',
        0xA7 => '6',
        0xA8 => '7',
        0xA9 => '8',
        0xAA => '9',
        0xAB => '!',
        0xAC => '?',
        0xAD => '.',
        0xAE => '-',
        0xAF => '·',
        // 0xB0 => '...
        0xB1 => '«',
        0xB2 => '»',
        // 0xB3 => |'|
        0xB4 => '\'',
        // 0xB5 => |m|
        // 0xB6 => |f|
        0xB7 => '$',
        0xB8 => ',',
        0xB9 => '*',
        0xBA => '/',
        0xBB => 'A',
        0xBC => 'B',
        0xBD => 'C',
        0xBE => 'D',
        0xBF => 'E',
        0xC0 => 'F',
        0xC1 => 'G',
        0xC2 => 'H',
        0xC3 => 'I',
        0xC4 => 'J',
        0xC5 => 'K',
        0xC6 => 'L',
        0xC7 => 'M',
        0xC8 => 'N',
        0xC9 => 'O',
        0xCA => 'P',
        0xCB => 'Q',
        0xCC => 'R',
        0xCD => 'S',
        0xCE => 'T',
        0xCF => 'U',
        0xD0 => 'V',
        0xD1 => 'W',
        0xD2 => 'X',
        0xD3 => 'Y',
        0xD4 => 'Z',
        0xD5 => 'a',
        0xD6 => 'b',
        0xD7 => 'c',
        0xD8 => 'd',
        0xD9 => 'e',
        0xDA => 'f',
        0xDB => 'g',
        0xDC => 'h',
        0xDD => 'i',
        0xDE => 'j',
        0xDF => 'k',
        0xE0 => 'l',
        0xE1 => 'm',
        0xE2 => 'n',
        0xE3 => 'o',
        0xE4 => 'p',
        0xE5 => 'q',
        0xE6 => 'r',
        0xE7 => 's',
        0xE8 => 't',
        0xE9 => 'u',
        0xEA => 'v',
        0xEB => 'w',
        0xEC => 'x',
        0xED => 'y',
        0xEE => 'z',
        // 0xEF => |>|
        0xF0 => ':',
        0xF1 => 'Ä',
        0xF2 => 'Ö',
        0xF3 => 'Ü',
        0xF4 => 'ä',
        0xF5 => 'ö',
        0xF6 => 'ü',
        // F7 => |A|
        // F8 => |V|
        // F9 => |<|
        // FA => |nb|
        // FB => |nb2|
        // FD => |FD|
        // FE => |br|
        ch @ _ => {
            ' '
            // println!("{:X}", ch);
            // break;
        },
    }
}

pub fn to_poke_char(ch: char) -> u8
{
    match ch
    {
        'À' => 0x01,
        'Á' => 0x02,
        'Â' => 0x03,
        'Ç' => 0x04,
        'È' => 0x05,
        'É' => 0x06,
        'Ê' => 0x07,
        'Ë' => 0x08,
        'Ì' => 0x09,
        'Î' => 0x0B,
        'Ï' => 0x0C,
        'Ò' => 0x0D,
        'Ó' => 0x0E,
        'Ô' => 0x0F,
        'Œ' => 0x10,
        'Ù' => 0x11,
        'Ú' => 0x12,
        'Û' => 0x13,
        'Ñ' => 0x14,
        'ß' => 0x15,
        'à' => 0x16,
        'á' => 0x17,
        'ç' => 0x19,
        'è' => 0x1A,
        'é' => 0x1B,
        'ê' => 0x1C,
        'ë' => 0x1D,
        'ì' => 0x1E,
        'î' => 0x20,
        'ï' => 0x21,
        'ò' => 0x22,
        'ó' => 0x23,
        'ô' => 0x24,
        'œ' => 0x25,
        'ù' => 0x26,
        'ú' => 0x27,
        'û' => 0x28,
        'ñ' => 0x29,
        'º' => 0x2A,
        'ª' => 0x2B,
        '&' => 0x2D,
        '+' => 0x2E,
        '=' => 0x35,
        ';' => 0x36,
        '¿' => 0x51,
        '¡' => 0x52,
        'Í' => 0x5A,
        '%' => 0x5B,
        '(' => 0x5C,
        ')' => 0x5D,
        'â' => 0x68,
        'í' => 0x6F,
        '<' => 0x85,
        '>' => 0x86,
        '0' => 0xA1,
        '1' => 0xA2,
        '2' => 0xA3,
        '3' => 0xA4,
        '4' => 0xA5,
        '5' => 0xA6,
        '6' => 0xA7,
        '7' => 0xA8,
        '8' => 0xA9,
        '9' => 0xAA,
        '!' => 0xAB,
        '?' => 0xAC,
        '.' => 0xAD,
        '-' => 0xAE,
        '·' => 0xAF,
        '«' => 0xB1,
        '»' => 0xB2,
        '\'' => 0xB4,
        '$' => 0xB7,
        ',' => 0xB8,
        '*' => 0xB9,
        '/' => 0xBA,
        'A' => 0xBB,
        'B' => 0xBC,
        'C' => 0xBD,
        'D' => 0xBE,
        'E' => 0xBF,
        'F' => 0xC0,
        'G' => 0xC1,
        'H' => 0xC2,
        'I' => 0xC3,
        'J' => 0xC4,
        'K' => 0xC5,
        'L' => 0xC6,
        'M' => 0xC7,
        'N' => 0xC8,
        'O' => 0xC9,
        'P' => 0xCA,
        'Q' => 0xCB,
        'R' => 0xCC,
        'S' => 0xCD,
        'T' => 0xCE,
        'U' => 0xCF,
        'V' => 0xD0,
        'W' => 0xD1,
        'X' => 0xD2,
        'Y' => 0xD3,
        'Z' => 0xD4,
        'a' => 0xD5,
        'b' => 0xD6,
        'c' => 0xD7,
        'd' => 0xD8,
        'e' => 0xD9,
        'f' => 0xDA,
        'g' => 0xDB,
        'h' => 0xDC,
        'i' => 0xDD,
        'j' => 0xDE,
        'k' => 0xDF,
        'l' => 0xE0,
        'm' => 0xE1,
        'n' => 0xE2,
        'o' => 0xE3,
        'p' => 0xE4,
        'q' => 0xE5,
        'r' => 0xE6,
        's' => 0xE7,
        't' => 0xE8,
        'u' => 0xE9,
        'v' => 0xEA,
        'w' => 0xEB,
        'x' => 0xEC,
        'y' => 0xED,
        'z' => 0xEE,
        ':' => 0xF0,
        'Ä' => 0xF1,
        'Ö' => 0xF2,
        'Ü' => 0xF3,
        'ä' => 0xF4,
        'ö' => 0xF5,
        'ü' => 0xF6,
        _ => 0x00
    }
}

// TODO: Make BankLoader use a function closer to the BufferedReader's version
pub fn read_poke_text(buffer: &[u8], offset: usize) -> String
{
    let mut result = String::new();
    let mut i = 0;
    let mut b: u8;
    loop 
    {
        b = buffer[offset + i];
        if b == 0xFF
        {
            break;
        }
        result.push(from_poke_char(b));
        i += 1;
    }
    // println!("{}", result.len());
    return result;
}

pub fn to_pokestring(string: &str) -> Vec<u8>
{
    // No real need to optimize here, but eh why not.
    let mut pokestring: Vec<u8> = Vec::with_capacity(string.len());
    pokestring.resize(string.len(), 0);

    for (i, ch) in string.chars().enumerate()
    {
        pokestring[i] = to_poke_char(ch);
    }

    return pokestring;
}

struct BufferedReader<'a>
{
    buffer: &'a mut [u8],
    idx: usize
}

impl<'a> BufferedReader<'a>
{
    pub fn new(buffer: &'a mut [u8]) -> Self
    {
        Self {
            buffer: buffer, // .iter().cloned().collect(),
            idx: 0
        }
    }

    pub fn bytes(&self) -> &[u8]
    {
        &self.buffer
    }

    pub fn seek(&mut self, offset: usize)
    {
        self.idx = offset;
    }

    pub fn write_poke_text(&mut self, text: &str, len: usize)
    {
        let mut i = 0;
        for ch in text.chars()
        {
            self.buffer[self.idx] = to_poke_char(ch);
            i += 1;
            self.idx += 1
        }
        self.buffer[self.idx] = 0xFF;
        i += 1;
        self.idx += 1;
        while i < len
        {
            self.buffer[self.idx] = 0x00;
            i += 1;
            self.idx += 1
        }
    }

    pub fn read_poke_text(&mut self, len: usize) -> String
    {
        let mut result = String::new();
        let mut b: u8;
        let mut i = 0;
        loop 
        {
            b = self.buffer[self.idx+i];
            i += 1;
            if b == 0xFF
            {
                break;
            }
            result.push(from_poke_char(b));
        }
        self.idx += i;
        // Pokemon names must be aligned so that they are all 10 characters long + null terminator.
        while i < len
        {
            self.idx += 1;
            i += 1;
        }
        // println!("{}", result.len());
        return result;
    }
}

pub fn capitalize(input: String) -> String
{
    let mut output = String::new();
    for (i, ch) in input.chars().enumerate()
    {
        if char::is_uppercase(ch) && i != 0
        {
            output.push_str(&ch.to_lowercase().to_string());
        } else {
            output.push(ch);
        }
    }

    return output;
}

// 
pub fn capitalize_string_table(buffer: &mut [u8], ptr: usize, 
                               table_len: usize, entry_len: usize) -> std::io::Result<()>
{
    use crate::bytes::load_poke_ptr;

    let table_offset = load_poke_ptr(buffer, ptr)?;
    
    let mut buf_reader = BufferedReader::new(buffer);
    let mut table_entries: Vec<String> = Vec::new();
    // Read in all the names
    buf_reader.seek(table_offset);
    for i in 0..table_len
    {
        table_entries.push(capitalize(buf_reader.read_poke_text(entry_len)));
    }

    buf_reader.seek(table_offset);
    for i in 0..table_len
    {
        buf_reader.write_poke_text(&table_entries[i], entry_len);
    }

    Ok(())
}

pub fn load_string_table(buffer: &mut [u8], ptr: usize, 
                        table_len: usize, entry_len: usize) -> std::io::Result<Vec<String>>
{
    use crate::bytes::load_poke_ptr;

    let table_offset = load_poke_ptr(buffer, ptr)?;

    let mut buf_reader = BufferedReader::new(buffer);
    let mut idx = 0;
    let mut move_name: String;
    // Going to take the performance hit at the moment,
    // because I'm not sure a good value to initialize to...
    let mut entries: Vec<String> = Vec::new();
    // probably wouldn't hurt to make a PokeString struct
    // and implement format and what not for it.
    buf_reader.seek(table_offset);
    for i in 0..table_len
    {
        entries.push(buf_reader.read_poke_text(entry_len));
    }

    Ok(entries)
}

pub fn find_string(buffer: &[u8], string: &str) -> usize
{
    // Need to create one that gets *ALL* matches.
    let pokestring = to_pokestring(string);
    let sch: u8 = pokestring[0]; // Starting character

    let mut matches: bool;
    for (i, &byte) in buffer.iter().enumerate()
    {
        if byte != sch
        {
            continue;
        }

        matches = true;
        // pch = poke char
        for (j, &pch) in pokestring.iter().enumerate()
        {
            matches &= buffer[i+j] == pch;
            if !matches
            {
                break;
            }
        } 

        if matches
        {
            return i;
        }
    }

    return 0;
}